use utils;

fn main() {
  let lines = utils::read_input_file_lines();

  let grid: Vec<Vec<isize>> = lines
    .map(|line| line.bytes().map(|n| (n as isize) - 48).collect())
    .collect();

  let mut visibility: Vec<Vec<bool>> = grid.iter().map(|row| vec![false; row.len()]).collect();

  check_rows(&grid, &mut visibility, 1, 1);
  check_rows(&grid, &mut visibility, 1, -1);
  check_cols(&grid, &mut visibility, 1, 1);
  check_cols(&grid, &mut visibility, -1, 1);

  let total_visible = visibility.iter().fold(0, |acc, row| {
    acc
      + row.iter().fold(0, |acc, v| {
        acc
          + match v {
            false => 0,
            true => 1,
          }
      })
  });

  println!("{}", total_visible);
}

fn check_rows(grid: &Vec<Vec<isize>>, visibility: &mut Vec<Vec<bool>>, y_dir: isize, x_dir: isize) {
  for i in 0..grid.len() {
    let y = if y_dir == 1 { i } else { grid.len() - i - 1 };
    let row = &grid[y];
    let mut prev_height = -1;

    for j in 0..row.len() {
      let x = if x_dir == 1 { j } else { row.len() - j - 1 };
      let height = grid[y][x];
      if height > prev_height {
        visibility[y][x] = true;
        prev_height = height;
      }
    }
  }
}

fn check_cols(grid: &Vec<Vec<isize>>, visibility: &mut Vec<Vec<bool>>, y_dir: isize, x_dir: isize) {
  let x_size = grid[0].len();

  for i in 0..x_size {
    let x = if x_dir == 1 { i } else { x_size - i - 1 };
    let mut prev_height = -1;

    for j in 0..grid.len() {
      let y = if y_dir == 1 { j } else { grid.len() - j - 1 };
      let height = grid[y][x];
      if height > prev_height {
        visibility[y][x] = true;
        prev_height = height;
      }
    }
  }
}
