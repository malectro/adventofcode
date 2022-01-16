#[derive(Debug, Clone, Eq, PartialEq)]
enum Attribute {
  Hp,
  Damage,
  Armor,
  Mana,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum EffectTarget {
  SpellTarget,
  Caster,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Effect {
  on: EffectTarget,
  attribute: Attribute,
  is_buff: bool,
  value: isize,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Spell {
  id: usize,
  name: String,
  duration: usize, // 0 = instant
  mana_cost: usize,
  effects: Vec<Effect>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Status {
  created_at: usize,
  caster: usize,
  spell: Spell,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Fighter {
  id: usize,
  name: String,
  hp: usize,
  damage: usize,
  armor: usize,
  mana: usize,
  statuses: Vec<Status>,
}

fn main() {
  let spells = [
    Spell {
      id: 1,
      name: "Magic Missile".to_string(),
      duration: 0,
      mana_cost: 53,
      effects: vec![Effect {
        on: EffectTarget::SpellTarget,
        attribute: Attribute::Hp,
        is_buff: false,
        value: -4,
      }],
    },
    Spell {
      id: 2,
      name: "Drain".to_string(),
      duration: 0,
      mana_cost: 73,
      effects: vec![
        Effect {
          on: EffectTarget::SpellTarget,
          attribute: Attribute::Hp,
          is_buff: false,
          value: -2,
        },
        Effect {
          on: EffectTarget::Caster,
          attribute: Attribute::Hp,
          is_buff: false,
          value: 2,
        },
      ],
    },
    Spell {
      id: 3,
      name: "Shield".to_string(),
      duration: 6,
      mana_cost: 113,
      effects: vec![Effect {
        on: EffectTarget::Caster,
        attribute: Attribute::Armor,
        is_buff: true,
        value: 7,
      }],
    },
    Spell {
      id: 4,
      name: "Poison".to_string(),
      duration: 6,
      mana_cost: 173,
      effects: vec![Effect {
        on: EffectTarget::SpellTarget,
        attribute: Attribute::Hp,
        is_buff: false,
        value: -3,
      }],
    },
    Spell {
      id: 5,
      name: "Recharge".to_string(),
      duration: 5,
      mana_cost: 229,
      effects: vec![Effect {
        on: EffectTarget::Caster,
        attribute: Attribute::Mana,
        is_buff: false,
        value: 101,
      }],
    },
  ];

  let player = Fighter {
    id: 1,
    name: "Player".to_string(),
    hp: 50,
    armor: 0,
    damage: 0,
    mana: 500,
    statuses: vec![],
  };

  let boss = Fighter {
    id: 2,
    name: "Boss".to_string(),
    hp: 51,
    armor: 0,
    damage: 9,
    mana: 0,
    statuses: vec![],
  };

  if let Some(lowest_cost) = fight(&spells, 1, player, boss) {
    println!("lowest cost: {}", lowest_cost);
  }
}

fn fight(spells: &[Spell], turn: usize, player: Fighter, boss: Fighter) -> Option<usize> {
  if player.hp == 0 {
    return None;
  }

  if boss.hp == 0 {
    return Some(0);
  }

  let mut lowest_cost = None;

  for spell in spells {
    if spell.mana_cost <= player.mana
      && boss
        .statuses
        .iter()
        .find(|status| status.spell.id == spell.id)
        == None
    {
      let mut current_turn = turn;
      let mut next_player = player.clone();
      let mut next_boss = boss.clone();

      if spell.duration == 0 {
        for effect in spell.effects.iter() {
          match effect.on {
            EffectTarget::SpellTarget => apply_effect(&mut next_boss, &effect),
            EffectTarget::Caster => apply_effect(&mut next_player, &effect),
          };
        }
      } else {
        next_boss.statuses.push(Status {
          created_at: current_turn,
          caster: player.id,
          spell: spell.clone(),
        });
      }
      next_player.mana = sub_to_zero(next_player.mana, spell.mana_cost);

      current_turn += 1;
      apply_longterm_spells(current_turn, &mut next_player, &mut next_boss);

      if next_boss.hp > 0 {
        // TODO (kyle): shield special case
        let buff_armor = next_boss.statuses.iter().fold(0, |acc, status| {
          status.spell.effects.iter().fold(acc, |acc, effect| {
            if effect.is_buff {
              effect.value + acc
            } else {
              acc
            }
          })
        });
        let armor: usize = match next_player.armor as isize + buff_armor {
          armor if armor < 0 => 0,
          armor => armor as usize,
        };

        let damage = if next_boss.damage > armor {
          next_boss.damage - armor
        } else {
          1
        };
        next_player.hp = sub_to_zero(next_player.hp, damage);
      }

      current_turn += 1;
      apply_longterm_spells(current_turn, &mut next_player, &mut next_boss);

      if let Some(cost) = fight(spells, current_turn, next_player, next_boss) {
        let total_cost = Some(cost + spell.mana_cost);
        if lowest_cost == None || total_cost < lowest_cost {
          lowest_cost = total_cost;
        }
      }
    }
  }

  lowest_cost
}

fn apply_effect(fighter: &mut Fighter, effect: &Effect) {
  match effect.attribute {
    Attribute::Hp => fighter.hp = add_to_zero(fighter.hp, effect.value),
    Attribute::Mana => fighter.mana = add_to_zero(fighter.mana, effect.value),
    _ => (),
  }
}

fn apply_longterm_spells<'a>(turn: usize, player: &'a mut Fighter, boss: &'a mut Fighter) {
  boss
    .statuses
    .retain(|status| turn - status.created_at <= status.spell.duration);

  // TODO (kyle): this feels silly
  for status in boss.statuses.clone().iter() {
    for effect in status.spell.effects.iter() {
      match effect.on {
        EffectTarget::SpellTarget => {
          apply_effect(boss, &effect);
        }
        EffectTarget::Caster => {
          apply_effect(player, &effect);
        }
      }
    }
  }
}

fn add_to_zero(a: usize, b: isize) -> usize {
  let c = a as isize + b;
  if c < 0 {
    0
  } else {
    c as usize
  }
}

fn sub_to_zero(a: usize, b: usize) -> usize {
  if b > a {
    0
  } else {
    a - b
  }
}
