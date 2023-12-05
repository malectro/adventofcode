use nom::{
  bytes::complete::tag,
  character::complete::{digit1, space1},
  combinator::map_res,
  multi::separated_list0,
  sequence::{preceded, tuple},
  IResult,
};
use std::cmp;

fn main() {
  part1();
}

fn part1() {
  let mut lines = utils::read_input_file_lines();

  let line1 = lines.next().expect("First line");
  let (_, numbers) =
    preceded(tag("seeds: "), separated_list0(space1, parse_usize))(&line1).expect("Invalid seeds");

  let (_, ranges) =
    preceded(tag("seeds: "), separated_list0(space1, parse_range))(&line1).expect("Invalid seeds");

  let mut seeds = numbers;
  let mut next_seeds: Vec<usize> = Vec::new();

  let mut seed_ranges = ranges;
  let mut next_ranges: Vec<SeedRange> = Vec::new();

  for line in lines {
    if line == "" {
      next_seeds.append(&mut seeds);
      seeds = next_seeds;
      next_seeds = Vec::new();

      next_ranges.append(&mut seed_ranges);
      seed_ranges = next_ranges;
      next_ranges = Vec::new();

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

      for range in seed_ranges.iter_mut() {
        if from <= range.from && from + size > range.from {
          let clip_point = cmp::min(range.to, from + size);

          let new_range = SeedRange {
            from: to + range.from - from,
            to: to + clip_point - from,
          };
          next_ranges.push(new_range);

          range.from = clip_point;
        } else if from < range.to {
          let clip_point = cmp::max(range.from, from);
          let end_clip_point = cmp::min(range.to, from + size);

          if clip_point < end_clip_point {
            let new_range = SeedRange {
              from: to + clip_point - from,
              to: to + end_clip_point - from,
            };
            next_ranges.push(new_range);

            range.to = clip_point;
          }
        }
      }

      // kinda wasteful
      seed_ranges.retain(|range| range.to > range.from)
    }
  }

  next_seeds.append(&mut seeds);
  next_ranges.append(&mut seed_ranges);

  println!(
    "Part 1 {}",
    next_seeds
      .iter()
      .reduce(|acc, v| if v < acc { v } else { acc })
      .expect("No final seeds")
  );

  println!(
    "Part 2 {}",
    next_ranges
      .iter()
      .reduce(|acc, range| if range.from < acc.from { range } else { acc })
      .expect("No final ranges")
      .from,
  );
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct SeedRange {
  from: usize,
  to: usize,
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
  return map_res(digit1, str::parse)(input);
}

fn parse_range(input: &str) -> IResult<&str, SeedRange> {
  return map_res(
    tuple((parse_usize, space1, parse_usize)),
    |(from, _, size)| {
      Ok::<SeedRange, Never>(SeedRange {
        from,
        to: from + size,
      })
    },
  )(input);
}

enum Never {}
