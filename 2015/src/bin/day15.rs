use utils;

#[derive(Debug)]
struct Ingredient {
  id: usize,
  name: String,
  props: [isize; 4],
  calories: usize,
}

fn main() {
  let ingredients: Vec<Ingredient> = utils::read_input_file_lines()
    .enumerate()
    .map(|(i, line)| {
      let (name, rest) = line.split_once(": ").expect("Invalid line");

      let props: Vec<isize> = rest
        .split(", ")
        .map(|string| {
          string
            .split_once(" ")
            .expect("Invalid prop")
            .1
            .parse::<isize>()
            .expect("Invalid prop number")
        })
        .collect();
      let props_array = to_props(&props);

      Ingredient {
        id: i,
        name: name.to_string(),
        props: props_array,
        calories: props[4] as usize,
      }
    })
    .collect();

  println!("ingredients {:?}", ingredients);

  let (recipe, max_score) = get_max(&ingredients, 100);
  println!("max score: {}", get_score(&max_score));

  let mut calories = get_calories(&ingredients, &recipe);
  println!("calories {}", calories);

  let mut new_recipe = recipe;
  let mut attrs = max_score;

  // Go through all possible changes and see which gets closer to the calorie count with the
  // highest possible score.
  while calories != 500 {
    let mut best_score = 0;
    let mut best_recipe = vec![];
    let mut best_attrs = [0isize; 4];
    let mut best_calories = 0;

    for (i, ingredient_1) in ingredients.iter().enumerate() {
      for (j, ingredient_2) in ingredients.iter().enumerate() {
        let mut try_recipe = new_recipe.clone();
        try_recipe[i] += 1;
        try_recipe[j] -= 1;

        let mut try_attrs = attrs;
        add(&mut try_attrs, &ingredient_1.props);
        sub(&mut try_attrs, &ingredient_2.props);

        let try_calories = get_calories(&ingredients, &try_recipe);
        if (500 - try_calories as isize).abs() < (500 - calories as isize).abs() {
          let score = get_score(&try_attrs);
          if score > best_score {
            best_score = score;
            best_calories = try_calories;
            best_recipe = try_recipe;
            best_attrs = try_attrs;
          }
        }
      }
    }

    calories = best_calories;
    new_recipe = best_recipe;
    attrs = best_attrs;
  }

  println!(
    "new recipe {:?} {} {}",
    new_recipe,
    get_score(&attrs),
    calories
  );

  let (brute_recipe, brute_attrs) = get_max_brute(&ingredients, 100, 500);
  println!(
    "brute recipe {:?} {}",
    brute_recipe,
    get_score(&brute_attrs),
  );
}

fn get_max(ingredients: &Vec<Ingredient>, amount: usize) -> (Vec<usize>, [isize; 4]) {
  let mut recipe = [0isize; 4];
  let mut counts = vec![0usize; ingredients.len()];

  let mut count = 0;
  while count < amount {
    while get_score(&recipe) <= 0 {
      let (i, _) =
        recipe.iter().enumerate().fold(
          (0, &isize::MAX),
          |acc, entry| if entry.1 < acc.1 { entry } else { acc },
        );

      for (j, ingredient) in ingredients.iter().enumerate() {
        if ingredient.props[i] > 0 {
          add(&mut recipe, &ingredient.props);
          count += 1;
          counts[j] += 1;
        }
      }
    }

    let (i, _) = ingredients
      .iter()
      .enumerate()
      .map(|(i, ingredient)| (i, get_score(add(&mut recipe.clone(), &ingredient.props))))
      .reduce(|acc, entry| if entry.1 > acc.1 { entry } else { acc })
      .expect("Ingredients are empty");

    add(&mut recipe, &ingredients[i].props);
    count += 1;
    counts[i] += 1;
  }

  println!("counts {:?}", counts);

  (counts, recipe)
}

#[derive(Clone)]
struct Scope {
  recipe: Vec<usize>,
  index: usize,
  amount: usize,
}
fn get_max_brute(
  ingredients: &Vec<Ingredient>,
  amount: usize,
  desired_calories: usize,
) -> (Vec<usize>, [isize; 4]) {
  let mut permutations = Vec::new();
  let mut scopes = Vec::new();

  scopes.push(Scope {
    recipe: vec![0usize; ingredients.len()],
    index: 0,
    amount: amount,
  });

  while let Some(scope) = scopes.pop() {
    if scope.index == ingredients.len() {
      permutations.push(scope.recipe.clone());
    } else {
      let i = scope.index;

      for j in 0..(scope.amount + 1) {
        let mut new_scope = scope.clone();
        new_scope.recipe[i] = j;
        new_scope.index += 1;
        new_scope.amount = scope.amount - j;
        scopes.push(new_scope);
      }
    }
  }

  let mut max_recipe = Vec::new();
  let mut max_attributes = [0isize; 4];
  let mut max_score = 0;
  for recipe in permutations {
    let attributes = get_attributes_from_recipe(&ingredients, &recipe);
    let score = get_score(&attributes);
    if score > max_score && get_calories(&ingredients, &recipe) == desired_calories {
      max_recipe = recipe;
      max_attributes = attributes;
      max_score = score;
    }
  }

  (max_recipe, max_attributes)
}

fn get_score(props: &[isize]) -> isize {
  return props
    .iter()
    .fold(1, |acc, value| std::cmp::max(value, &0) * acc);
}

fn get_attributes_from_recipe(ingredients: &Vec<Ingredient>, recipe: &Vec<usize>) -> [isize; 4] {
  recipe
    .iter()
    .enumerate()
    .fold([0isize; 4], |mut acc, (i, amount)| {
      let ingredient = &ingredients[i];
      *add(
        &mut acc,
        scale(add(&mut [0isize; 4], &ingredient.props), *amount as isize),
      )
    })
}

fn to_props(props: &Vec<isize>) -> [isize; 4] {
  let mut props_array = [0isize; 4];
  for (i, prop) in props.iter().take(4).enumerate() {
    props_array[i] = *prop;
  }
  props_array
}

fn add<'a>(recipe: &'a mut [isize; 4], props: &[isize; 4]) -> &'a mut [isize; 4] {
  for i in 0..4 {
    recipe[i] += props[i];
  }
  recipe
}

fn sub<'a>(recipe: &'a mut [isize; 4], props: &[isize; 4]) -> &'a mut [isize; 4] {
  for i in 0..4 {
    recipe[i] -= props[i];
  }
  recipe
}

fn scale<'a>(recipe: &'a mut [isize; 4], amount: isize) -> &'a mut [isize; 4] {
  for i in 0..4 {
    recipe[i] *= amount;
  }
  recipe
}

fn get_sum(props: &[isize]) -> isize {
  return props.iter().fold(1, |acc, value| acc + value);
}

fn get_calories(ingredients: &Vec<Ingredient>, recipe: &Vec<usize>) -> usize {
  recipe
    .iter()
    .enumerate()
    .fold(0, |acc, (i, count)| acc + *count * ingredients[i].calories)
}
