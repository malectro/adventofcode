fn main() {
  let mut answer: u64 = 20151125;

  let mut x = 0;
  let mut y = 0;
  let mut steps = 0;

  while steps < 6 {
    //steps += 1;
    answer = (answer * 252533) % 33554393;

    if y == 0 {
      y = x + 1;
      x = 0;
    } else {
      y -= 1;
      x += 1;
    }

    //println!("{},{} = {}", x, y, answer);

    if x == 3083 - 1 && y == 2978 - 1 {
      break;
    }
  }

  println!("{}", answer);
}
