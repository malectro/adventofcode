use std::collections::{HashMap, HashSet};
use utils;

fn main() {
  let packages: Vec<usize> = utils::read_input_file_lines()
    .map(|line| line.parse().expect("Invalid package"))
    .collect();

  println!(
    "{:?}",
    Permutations::of(vec![1, 2, 3]).collect::<Vec<Vec<usize>>>(),
  );

  let mut best = (usize::MAX, usize::MAX);
  let goal_weight = get_weight(&packages) / 3;

  for group in Permutations::of(packages.clone()) {
    if group.len() <= best.0 && get_weight(&group) == goal_weight {
      let group_set: HashSet<usize> = group.iter().cloned().collect();
      let rest: Vec<usize> = packages
        .iter()
        .cloned()
        .filter(|value| !group_set.contains(value))
        .collect();
      if Permutations::of(rest).any(|group| get_weight(&group) == goal_weight) {
          println!("winner! {:?}", group);
        let score = (group.len(), get_score(&group));
        if score < best {
          best = score;
        }
      }
    }
  }

  println!("best {:?}", best);

  //let groups = [Vec::new(), Vec::new(), Vec::new()];

  //if let Some((_, score)) = arrange(get_weight(&packages) / 3, packages, groups, None) {
  /*
  if let Some((_, score)) = arrange2(
    &mut HashMap::new(),
    get_weight(&packages) / 3,
    packages.into_iter().collect(),
    Vec::new(),
  ) {
    println!("Success {}", score);
  }
  */
}

fn arrange(
  goal_weight: usize,
  packages: Vec<usize>,
  groups: [Vec<usize>; 3],
  best: Option<(usize, usize)>,
) -> Option<(usize, usize)> {
  println!("{:?} {:?}", best, groups);
  let count = groups[0].len();
  let score = groups[0].iter().fold(1, |acc, value| value * acc);
  let weights: Vec<usize> = groups.iter().map(|group| get_weight(group)).collect();

  if packages.len() == 0 {
    return if weights[0] == weights[1] && weights[0] == weights[2] {
      Some((count, score))
    } else {
      None
    };
  }

  let packages_weight = get_weight(&packages);

  if let Some((best_count, best_score)) = best {
    if count > best_count {
      return None;
    } else if count == best_count {
      if weights[0] != goal_weight {
        return None;
      }
      if score >= best_score {
        return None;
      }
      if (packages_weight + weights[1] + weights[2]) / 2 != weights[0] {
        return None;
      }
    } else if count == best_count - 1 {
      if !packages
        .iter()
        .any(|package| package + weights[0] == goal_weight)
      {
        return None;
      }
    }
  }

  if weights.iter().any(|weight| weight > &goal_weight) {
    return None;
  }

  if packages_weight < abs_diff(weights[0], weights[1]) + abs_diff(weights[0], weights[2]) {
    return None;
  }

  if packages_weight < abs_diff(weights[1], weights[2]) {
    return None;
  }

  let mut ret = best;

  for i in 0..groups.len() {
    let mut current_packages = packages.clone();
    let mut current_groups = groups.clone();

    current_groups[i].push(current_packages.pop().expect("should have a package"));
    let result = arrange(goal_weight, current_packages, current_groups, ret);
    if result != None && (ret == None || result < ret) {
      ret = result;
    }
  }

  ret
}

fn arrange2(
  results: &mut HashMap<Vec<usize>, Option<(usize, usize)>>,
  goal_weight: usize,
  packages: HashSet<usize>,
  group: Vec<usize>,
) -> Option<(usize, usize)> {
  //println!("{:?}", group);

  let weight = get_weight(&group);
  if weight > goal_weight {
    results.insert(group, None);
    return None;
  } else if weight == goal_weight {
    let result;
    let score = Some((group.len(), 0));
    if packages.len() > 0 {
      let other_result = arrange2(
        &mut HashMap::new(),
        goal_weight,
        packages.clone(),
        Vec::new(),
      );
      if other_result != None {
        if other_result < score {
          result = other_result;
        } else {
          result = score;
        }
      } else {
        result = None;
      }
    } else {
      result = score;
    }
    results.insert(group, result);
    return result;
  }

  let mut ret = None;
  for package in packages.iter() {
    let mut current_packages = packages.clone();
    let mut current_group = group.clone();

    current_group.push(*package);
    current_group.sort_unstable();
    current_packages.remove(package);

    let result;
    if let Some(result_) = results.get(&current_group) {
      result = *result_
    } else {
      result = arrange2(
        results,
        goal_weight,
        current_packages,
        current_group.clone(),
      );
      results.insert(current_group, result);
    }
    if result != None {
      if ret == None || result < ret {
        ret = result;
      }
    }
  }
  ret
}

fn get_weight(group: &Vec<usize>) -> usize {
  group.iter().fold(0, |acc, value| acc + value)
}

fn get_score(group: &Vec<usize>) -> usize {
  group.iter().fold(1, |acc, value| acc * value)
}

fn abs_diff(a: usize, b: usize) -> usize {
  if a > b {
    a - b
  } else {
    b - a
  }
}

/*
fn permutations(v: Vec<usize>) {
  let result = Vec::new();

  if v.len() == 0 {
    return result;
  }

  let value = v.pop().unwrap();
  result.push(vec![value]);
  for permutation in permutations(v) {
    result.push(permutation);
    let mut extra = permutation.clone();
    extra.push(value);
    result.push(extra);
  }

  result
}

fn permutations2(v: &Vec<usize>) {}
*/

struct Permutations {
  //vector: Vec<usize>,
  value: Option<usize>,
  append_to: Option<Vec<usize>>,
  sub_iter: Box<Option<Permutations>>,
}

impl Permutations {
  fn of(mut v: Vec<usize>) -> Permutations {
    let value = v.pop();
    Permutations {
      value,
      append_to: None,
      sub_iter: Box::new(if v.len() > 0 {
        Some(Permutations::of(v))
      } else {
        None
      }),
    }
  }
}

impl Iterator for Permutations {
  type Item = Vec<usize>;
  fn next(&mut self) -> Option<Vec<usize>> {
    if let Some(value) = self.value {
      if let Some(mut append_to) = self.append_to.take() {
        append_to.push(value);
        return Some(append_to);
      }

      // TODO (kyle): i shouldn't have to _take_ this?
      if let Some(mut sub_iter) = self.sub_iter.take() {
        if let Some(permutation) = sub_iter.next() {
          self.append_to = Some(permutation.clone());
          self.sub_iter = Box::new(Some(sub_iter));
          return Some(permutation);
        }

        self.sub_iter = Box::new(Some(sub_iter));
      }
      self.value = None;
      return Some(vec![value]);
    }
    None
  }
}
