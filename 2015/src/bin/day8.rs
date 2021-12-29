use utils;

fn main() {
  let lines = utils::read_input_file_lines();

  let mut code_count = 0;
  let mut char_count = 0;
  let mut enc_count = 0;

  for line in lines {
    let len = line.len();
    code_count += len;

    enc_count += 6;

    let mut is_escaped = false;
    let mut is_hex = 0;
    for c in line[1..len - 1].chars() {
      enc_count += match c {
        '\\' => 2,
        '"' => 2,
        _ => 1,
      };

      if is_hex > 0 {
        if c.is_digit(16) {
          if is_hex == 1 {
            is_hex += 1;
          } else {
            is_hex = 0;
            char_count += 1;
          }
        } else {
          char_count += is_hex + 2;
          is_hex = 0;
        }
      } else if is_escaped {
        char_count += match c {
          '\\' => 1,
          '"' => 1,
          'x' => {
            is_hex = 1;
            0
          }
          _ => 2,
        };
        is_escaped = false;
      } else if c == '\\' {
        is_escaped = true;
      } else {
        char_count += 1;
      }
    }
  }

  println!(
    "code count {} - char count {} = {}",
    code_count,
    char_count,
    code_count - char_count
  );

  println!(
    "enc count {} - code count {} = {}",
    enc_count,
    code_count,
    enc_count - code_count
  );
}
