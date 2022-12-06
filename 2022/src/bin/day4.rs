use utils;

fn main() {
  let lines = utils::read_input_file_lines();

  let mut contains_count = 0;
  let mut overlap_count = 0;

  for line in lines {
    let (left, right) = line.split_once(",").expect("Invalid line");
    let elf1 = parse_elf(left);
    let elf2 = parse_elf(right);

    if contains(elf1, elf2) || contains(elf2, elf1) {
      contains_count += 1;
    }
    println!("{}: {}", line, overlaps(elf1, elf2));
    if overlaps(elf1, elf2) {
      overlap_count += 1;
    }
  }

  println!("contains count {}", contains_count);
  println!("overlaps count {}", overlap_count);
}

fn parse_elf(str: &str) -> [usize; 2] {
  let stuff = str.split_once("-").unwrap();
  return [stuff.0.parse().unwrap(), stuff.1.parse().unwrap()];
}

fn contains(elf1: [usize; 2], elf2: [usize; 2]) -> bool {
  return elf1[0] <= elf2[0] && elf1[1] >= elf2[1];
}

fn overlaps(elf1: [usize; 2], elf2: [usize; 2]) -> bool {
  return (elf1[0] <= elf2[0] && elf2[0] <= elf1[1]) || (elf2[0] <= elf1[0] && elf1[0] <= elf2[1]);
}
