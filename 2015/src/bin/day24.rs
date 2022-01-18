use utils;

fn main() {
  let packages: Vec<usize> = utils::read_input_file_lines()
    .map(|line| line.parse().expect("Invalid package"))
    .collect();

  let groups = [Vec::new(), Vec::new(), Vec::new()];

  if let Some((_, score)) = arrange(packages, groups) {
    println!("Success {}", score);
  }
}

fn arrange(packages: Vec<usize>, groups: [Vec<usize>; 3]) -> Option<(usize, usize)> {
  if packages.len() == 0 {
    return if get_weight(&groups[0]) == get_weight(&groups[1])
      && get_weight(&groups[0]) == get_weight(&groups[2])
    {
      Some((
        groups[0].len(),
        groups[0].iter().fold(1, |acc, value| value * acc),
      ))
    } else {
      None
    };
  }

  let mut ret = None;

  for i in 0..groups.len() {
    let mut current_packages = packages.clone();
    let mut current_groups = groups.clone();

    current_groups[i].push(current_packages.pop().expect("should have a package"));
    let result = arrange(current_packages, current_groups);
    if ret == None || result < ret {
      ret = result;
    }
  }

  ret
}

fn get_weight(group: &Vec<usize>) -> usize {
  group.iter().fold(0, |acc, value| acc + value)
}
