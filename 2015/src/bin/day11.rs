use itertools::Itertools;

const byte_a: u8 = 97;
const byte_z: u8 = 122;

fn main() {
  let string = std::env::args().nth(1).expect("No input given.");

  //println!("valid? {}", is_password_valid(&password));
  //println!("next password {}", increment_password(&password));

  let mut password = increment_password(&string);
  let mut steps = 0;
  while !is_password_valid(&password) && steps < 10 {
    //steps += 1;
    //println!("{}", password);
    password = increment_password(&password);
  }

  println!("next valid password: {}", password);
}

fn is_password_valid(password: &str) -> bool {
  let has_straight = password
    .bytes()
    .tuple_windows()
    .any(|(a, b, c)| b == a + 1 && c == b + 1);

  let iol = ['i', 'o', 'l'];
  let has_iol = password.chars().any(|c| iol.contains(&c));

  let mut pair_count = 0;
  let mut last_pair = (0, 0);
  for (i, (a, b)) in password.bytes().tuple_windows().enumerate() {
    if a == b {
      if !(a == last_pair.1 && last_pair.0 == i - 1) {
        pair_count += 1;
        last_pair = (i, a);
      }
    }
  }

  !has_iol && has_straight && pair_count > 1
}

fn increment_password(password: &str) -> String {
  let mut new_vec = Vec::new();

  let mut carry = 1;
  for b in password.bytes().rev() {
    let mut new_b = b + carry;
    if new_b > byte_z {
      new_b = new_b - byte_z + byte_a - 1;
      carry = 1;
    } else {
      carry = 0;
    }
    new_vec.push(new_b);
  }
  if carry > 0 {
    new_vec.push(byte_a);
  }

  new_vec.reverse();
  String::from_utf8(new_vec).expect("Invalid bytes")
}
