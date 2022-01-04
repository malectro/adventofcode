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
      let props_array = to_props(props);

      Ingredient {
        id: i,
        name: name.to_string(),
        props: props_array,
      }
    })
    .collect();

  println!("ingredients {:?}", ingredients);

  let max_score = get_max_2(&ingredients, 100);
  println!("max score: {}", get_score(&max_score));
}

fn get_max_2(ingredients: &Vec<Ingredient>, amount: usize) -> [isize; 4] {
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

  recipe
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
