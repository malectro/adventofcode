use regex::Regex;
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

  for rule in rules.iter() {
    for (i, _) in medicine.match_indices(&rule.from) {
      let j = i + rule.from.len();
      molecules.insert(format!("{}{}{}", &medicine[0..i], rule.to, &medicine[j..]));
    }
  }

  println!("number of molecules: {}", molecules.len());

  let re = Regex::new(r"[A-Z][a-z]*").expect("Invalid Regex");
  let elements = re.find_iter(&medicine).map(|matches| matches.as_str());

  let step_count = elements.fold(0, |acc, element| {
    acc
      + match element {
        "Rn" | "Ar" => 0,
        "Y" => -1,
        _ => 1,
      }
  }) - 1;

  println!("shortest trip: {}", step_count);
}
