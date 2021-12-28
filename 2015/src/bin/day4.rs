use md5::{Digest, Md5};

fn main() {
  let secret: String = std::env::args().nth(1).expect("no secret");

  let mut i = 0;
  let mut found_5 = 0;
  let mut found_6 = 0;
  while found_6 == 0 || found_5 == 0 {
    let digest = Md5::digest(format!("{}{}", secret, i));
    let zero_count = count_zeros(&digest);
    if zero_count >= 5 && found_5 == 0 {
      found_5 = i;
    }
    if zero_count >= 6 && found_6 == 0 {
      found_6 = i;
    }
    i += 1;
  }

  println!("your secret {}", secret);
  println!("salt for 5 {}", found_5);
  println!("salt for 6 {}", found_6);
}

fn count_zeros(digest: &[u8]) -> usize {
  let mut count = 0;
  for value in digest {
    if *value == 0 {
      count += 2;
    } else {
      if *value < 16 {
        count += 1;
      }
      return count;
    }
  }
  count
}
