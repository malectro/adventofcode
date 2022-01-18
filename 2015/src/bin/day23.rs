use utils;

fn main() {
  let mut reg = [0, 0];

  let instructions: Vec<String> = utils::read_input_file_lines().collect();

  let mut i: usize = 0;
  while i < instructions.len() {
    let (command, rest) = instructions[i].split_once(' ').expect("Invalid line");
    let params: Vec<&str> = rest.split(", ").collect();

    match command {
      "hlf" => {
        reg[r(params[0])] /= 2;
      }
      "tpl" => {
        reg[r(params[0])] *= 3;
      }
      "inc" => {
        reg[r(params[0])] += 1;
      }
      "jmp" => {
        i = (i as isize + get_jump_index(params[0])) as usize;
      }
      "jie" => {
        if reg[r(params[0])] % 2 == 0 {
          i = (i as isize + get_jump_index(params[1])) as usize;
        }
      }
      "jio" => {
        if reg[r(params[0])] == 1 {
          i = (i as isize + get_jump_index(params[1])) as usize;
        }
      }
      _ => panic!("Invalid command"),
    }

    i += 1;
  }

  println!("b: {}", reg[1]);
}

fn r(name: &str) -> usize {
  match name {
    "a" => 0,
    "b" => 1,
    _ => panic!("Invalid register name"),
  }
}

fn get_jump_index(param: &str) -> isize {
  param.parse::<isize>().expect("Invalid number") - 1
}
