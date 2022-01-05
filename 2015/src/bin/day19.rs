use std::collections::HashSet;
use utils;

struct Rule {
  from: String,
  to: String,
}

fn main() {
  let mut lines = utils::read_input_file_lines();

  let rules: Vec<Rule> = lines
    .by_ref()
    .take_while(|line| line != "")
    .map(|line| {
      let (from, to) = line.split_once(" => ").expect("Invalid rule");
      Rule {
        from: from.to_string(),
        to: to.to_string(),
      }
    })
    .collect();

  let medicine = lines.next().expect("Should have one more string");

  let mut molecules: HashSet<String> = HashSet::new();

  for rule in rules {
    for (i, _) in medicine.match_indices(&rule.from) {
      let j = i + rule.from.len();
      molecules.insert(format!("{}{}{}", &medicine[0..i], rule.to, &medicine[j..]));
    }
  }

  println!("number of molecules: {}", molecules.len());
}
