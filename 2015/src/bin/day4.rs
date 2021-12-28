use md5::{Digest, Md5};

fn main() {
  let secret: String = std::env::args().nth(1).expect("no secret");

  let mut i = 0;
  let mut found = 0;
  while found == 0 {
    let digest = Md5::digest(format!("{}{}", secret, i));
    if has_5_zeros(&digest) {
      found = i;
    }
    i += 1;
  }

  println!("your secret {}", secret);
  println!("your salt {}", found);
}

fn has_5_zeros(digest: &[u8]) -> bool {
  for j in 0..2 {
    if digest[j] != 0 {
      return false;
    }
  }
  digest[2] < 16
}
