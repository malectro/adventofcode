fn main() {
  let mut string = std::env::args().nth(1).expect("No input given.");

  for _ in 0..40 {
    let mut new_string = String::new();

    let mut current_char = 'a';
    let mut current_char_count = 0;

    for c in string.chars() {
      if c != current_char {
        if current_char != 'a' {
          new_string = format!("{}{}", new_string, current_char_count);
          new_string.push(current_char);
        }
        current_char = c;
        current_char_count = 1;
      } else {
        current_char_count += 1;
      }
    }

    new_string = format!("{}{}", new_string, current_char_count);
    new_string.push(current_char);

    string = new_string;
  }

  println!("final string {}", string);
  println!("final number {}", string.len());
}
