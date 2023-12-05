use nom::{
  bytes::complete::tag,
  character::complete::{digit1, space1},
  combinator::map_res,
  multi::separated_list0,
  sequence::{pair, preceded, tuple},
  IResult,
};

fn main() {
  part1();
}

fn part1() {
  let mut lines = utils::read_input_file_lines();

  let line1 = lines.next().expect("First line");
  let (_, numbers) =
    preceded(tag("seeds: "), separated_list0(space1, parse_usize))(&line1).expect("Invalid seeds");

  let mut seeds = numbers;

  let mut next_seeds: Vec<usize> = Vec::new();
  for line in lines {
    if line == "" {
      next_seeds.append(&mut seeds);
      seeds = next_seeds;
      next_seeds = Vec::new();
    } else if let Some(range) =
      tuple((parse_usize, space1, parse_usize, space1, parse_usize))(&line).ok()
    {
      let (_, (to, _, from, _, size)) = range;
      let mut removed_count = 0;
      for (i, seed) in seeds.clone().into_iter().enumerate() {
        if seed >= from && seed < from + size {
          next_seeds.push(to + seed - from);
          seeds.remove(i - removed_count);
          removed_count += 1;
        }
      }
    }
  }

  next_seeds.append(&mut seeds);

  println!(
    "Part 1 {}",
    next_seeds
      .iter()
      .reduce(|acc, v| if v < acc { v } else { acc })
      .expect("No final seeds")
  );
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
  return map_res(digit1, str::parse)(input);
}
