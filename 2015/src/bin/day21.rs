#[macro_use]
extern crate lazy_static;

use regex::Regex;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Fighter {
  id: usize,
  name: String,
  hp: usize,
  damage: usize,
  armor: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Item {
  name: String,
  cost: usize,
  damage: usize,
  armor: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Player {
  fighter: Fighter,
  weapon: Option<Item>,
  armor: Option<Item>,
  rings: [Option<Item>; 2],
}

fn main() {
  let base_boss = Fighter {
    id: 2,
    name: "Boss".to_string(),
    hp: 109,
    damage: 8,
    armor: 2,
  };

  let base_player = Fighter {
    id: 1,
    name: "Player".to_string(),
    hp: 100,
    damage: 0,
    armor: 0,
  };

  let weapons = parse_items(
    "Dagger        8     4       0
Shortsword   10     5       0
Warhammer    25     6       0
Longsword    40     7       0
Greataxe     74     8       0",
  );

  let armors = parse_items(
    "Leather      13     0       1
Chainmail    31     0       2
Splintmail   53     0       3
Bandedmail   75     0       4
Platemail   102     0       5",
  );

  let rings = parse_items(
    "Damage +1    25     1       0
Damage +2    50     2       0
Damage +3   100     3       0
Defense +1   20     0       1
Defense +2   40     0       2
Defense +3   80     0       3",
  );

  let mut best_player = None;

  let mut player = base_player.clone();
  let mut boss = base_boss.clone();
  let winner = fight(&mut player, &mut boss);
  println!("winner {:?}", winner);

  let armor_store = make_store(&armors);
  let ring_store = make_store(&rings);

  for weapon in weapons.iter() {
    for armor in armor_store.iter() {
      for ring1 in ring_store.iter() {
        for ring2 in ring_store.iter() {
          if ring1 != ring2 || ring1 == &None {
            let mut player = Player {
              fighter: base_player.clone(),
              weapon: Some(weapon.clone()),
              armor: armor.clone(),
              rings: [ring1.clone(), ring2.clone()],
            };

            player.fighter.damage = weapon.damage;
            if let Some(armor) = armor {
              player.fighter.armor = armor.armor;
            }
            for ring in player.rings.iter() {
              if let Some(ring) = ring {
                player.fighter.armor += ring.armor;
                player.fighter.damage += ring.damage;
              }
            }

            if fight(&mut player.fighter, &mut base_boss.clone()).id == player.fighter.id {
              let other_score = if let Some(other_player) = &best_player {
                get_player_cost(other_player)
              } else {
                usize::MAX
              };

              let cost = get_player_cost(&player);
              if other_score > cost {
                println!("it's better {:?} {}", player, cost);
                best_player = Some(player);
              }
            }
          }
        }
      }
    }
  }

  if let Some(player) = best_player {
    println!("best fighter {:?} {}", player, get_player_cost(&player));
  }
}

fn get_player_cost(player: &Player) -> usize {
  get_equip_cost(&player.weapon)
    + get_equip_cost(&player.armor)
    + get_equip_cost(&player.rings[0])
    + get_equip_cost(&player.rings[1])
}

fn get_equip_cost(item: &Option<Item>) -> usize {
  if let Some(item) = item {
    item.cost
  } else {
    0
  }
}

fn fight<'a>(fighter1: &'a mut Fighter, fighter2: &'a mut Fighter) -> &'a mut Fighter {
  let mut attacker = fighter1;
  let mut defender = fighter2;
  loop {
    let damage = if defender.armor < attacker.damage {
      attacker.damage - defender.armor
    } else {
      1
    };

    if defender.hp < damage {
      return attacker;
    }

    defender.hp -= damage;

    let temp = attacker;
    attacker = defender;
    defender = temp;
  }
}

fn make_store(items: &Vec<Item>) -> Vec<Option<Item>> {
  let mut store: Vec<Option<Item>> = items.iter().map(|item| Some(item.clone())).collect();
  store.push(None);
  store
}

fn parse_items(string: &str) -> Vec<Item> {
  string.split("\n").map(parse_item).collect()
}

fn parse_item(string: &str) -> Item {
  println!("hi {}", string);
  lazy_static! {
    static ref RE: Regex = Regex::new(r"^(.+) +(\d+) +(\d+) +(\d+)$").unwrap();
  }
  let elements = RE.captures(string).expect("Invalid string");
  Item {
    name: elements[1].trim().to_string(),
    cost: elements[2].parse().expect("Invalid number"),
    damage: elements[3].parse().expect("Invalid number"),
    armor: elements[4].parse().expect("Invalid number"),
  }
}
