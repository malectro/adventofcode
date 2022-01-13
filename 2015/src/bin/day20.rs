fn main() {
  let number_string = std::env::args().nth(1).expect("no number given");
  let goal: usize = number_string.parse::<usize>().expect("invalid number");
  let goal_div_10 = goal / 10;

  //println!("{} -> {}", goal, get_score(goal) * 10);
  
  println!("{}", better(goal_div_10));
  println!("{}", part_2(goal));
}

fn naive(goal: usize) -> usize {
  let mut guess = 0;
  loop {
    //println!("trying {}", guess);
    if get_score(guess) >= goal {
      return guess;
    }
    guess += 1000;
  }
}

fn better(goal: usize) -> usize {
  let mut calcs: Vec<usize> = vec![0usize; goal];
  for i in 1..goal {
    let mut j = i;
    while j < goal {
      calcs[j] += i;
      j += i;
    }
    if calcs[i] >= goal {
      return i;
    }
  }
  goal
}

fn part_2(goal: usize) -> usize {
  let mut calcs: Vec<usize> = vec![0usize; goal];
  for i in 1..goal {
    let mut j = i;
    let mut steps = 0;
    while j < goal && steps < 50 {
      calcs[j] += i * 11;
      j += i;
      steps += 1;
    }
    if calcs[i] >= goal {
      return i;
    }
  }
  goal
}

fn get_score(guess: usize) -> usize {
  let mut score = 0;
  for i in 1..(guess + 1) {
    if guess % i == 0 {
      score += i;
    }
  }
  score
}
