use utils;

#[derive(Debug)]
struct Ingredient {
  id: usize,
  name: String,
  props: [isize; 4],
}

fn main() {
  let ingredients: Vec<Ingredient> = utils::read_input_file_lines()
    .enumerate()
    .map(|(i, line)| {
      let (name, rest) = line.split_once(": ").expect("Invalid line");

      let props = rest.split(", ").map(|string| {
        string
          .split_once(" ")
          .expect("Invalid prop")
          .1
          .parse::<isize>()
          .expect("Invalid prop number")
      });

      /*
      let mut props_array = [0isize; 4];
      for (i, prop) in props.take(4).enumerate() {
        props_array[i] = prop;
      }
      */
      let props_array = to_props(props);

      Ingredient {
        id: i,
        name: name.to_string(),
        props: props_array,
      }
    })
    .collect();

  println!("ingredients {:?}", ingredients);

  // brute force
  let max_score = get_max_2(&ingredients, 100);
  println!("max score: {}", get_score(&max_score));

  println!(
    "{} {}",
    get_score(&[-100, -200, 600, 300]),
    get_score(&[200, 300, -200, -100])
  );
}

fn get_max_2(ingredients: &Vec<Ingredient>, amount: usize) -> [isize; 4] {
  let mut recipe = [0isize; 4];
  let mut counts = vec![0usize; ingredients.len()];

  let max_ingredient = ingredients
    .iter()
    .reduce(|acc, ingredient| {
      if get_sum(&ingredient.props) > get_sum(&acc.props) {
        ingredient
      } else {
        acc
      }
    })
    .expect("Must have one");

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
      .reduce(|acc, entry| {
        if entry.1 > acc.1 {
          entry
        } else {
          acc
        }
      }).expect("Ingredients are empty");

    add(&mut recipe, &ingredients[i].props);
    count += 1;
    counts[i] += 1;

    /*
    let possible_maxes: Vec<(usize, isize)> = ingredients
      .iter()
      .map(|ingredient| get_score(add(&mut recipe.clone(), &ingredient.props)))
      .collect();
          */

    /*
    if !did_add {
      add(&mut recipe, &max_ingredient.props);
      counts[0] += 1;
      count += 1;
    }
    did_add = false;
    */
  }

  let possible_maxes: Vec<isize> = ingredients
    .iter()
    .map(|ingredient| get_score(add(&mut recipe.clone(), &ingredient.props)))
    .collect();

  println!("counts {:?}", counts);
  println!("possible maxes {:?}", possible_maxes);

  recipe
}

fn get_max(ingredients: &Vec<Ingredient>, amount: usize) -> [isize; 4] {
  let mut max = [0isize; 4];

  if amount == 0 {
    return max;
  }

  let mut max_score = 0;

  let best = get_max(ingredients, amount - 1);
  for ingredient in ingredients {
    let candidate: Vec<isize> = best
      .iter()
      .zip(ingredient.props)
      .map(|(a, b)| a + b)
      .collect();
    let score = get_score(candidate.as_slice());

    if score > max_score {
      max = to_props(candidate.into_iter());
      max_score = score;
    }
  }

  max
}

fn get_score(props: &[isize]) -> isize {
  return props
    .iter()
    .fold(1, |acc, value| std::cmp::max(value, &0) * acc);
}

fn to_props(props: impl Iterator<Item = isize>) -> [isize; 4] {
  let mut props_array = [0isize; 4];
  for (i, prop) in props.take(4).enumerate() {
    props_array[i] = prop;
  }
  props_array
}

fn add<'a>(recipe: &'a mut [isize; 4], props: &[isize; 4]) -> &'a mut [isize; 4] {
  for i in 0..4 {
    recipe[i] += props[i];
  }
  recipe
}

fn get_sum(props: &[isize]) -> isize {
  return props.iter().fold(1, |acc, value| acc + value);
}
