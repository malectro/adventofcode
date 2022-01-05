use std::cmp;
use std::collections::HashSet;
use utils;

fn main() {
  let grid: Vec<Vec<bool>> = utils::read_input_file_lines()
    .map(|line| line.chars().map(|c| c == '#').collect())
    .collect();

  let grid_size = grid.len();

  let mut final_grid = grid;
  let corners = HashSet::from([
    [0, 0],
    [0, grid_size - 1],
    [grid_size - 1, 0],
    [grid_size - 1, grid_size - 1],
  ]);

  for [x, y] in corners.iter() {
    final_grid[*y][*x] = true;
  }

  //println!("{}", serialize_grid(&grid));

  for _ in 0..100 {
    let mut new_grid = final_grid.clone();

    for y in 0..grid_size {
      for x in 0..grid_size {
        let light = final_grid[y][x];

        if corners.contains(&[x, y]) {
          new_grid[y][x] = true;
        } else {
          let mut neighbor_count = 0;

          let mx = if x > 0 { x - 1 } else { x };
          let my = if y > 0 { y - 1 } else { y };

          for nx in mx..cmp::min(grid_size, x + 2) {
            for ny in my..cmp::min(grid_size, y + 2) {
              if (nx != x || ny != y) && final_grid[ny][nx] {
                neighbor_count += 1;
              }
            }
          }

          if light {
            new_grid[y][x] = neighbor_count == 2 || neighbor_count == 3;
          } else {
            new_grid[y][x] = neighbor_count == 3;
          }
        }
      }
    }

    //println!("{}", serialize_grid(&new_grid));

    final_grid = new_grid;
  }

  let light_count = final_grid.iter().fold(0, |acc, row| {
    acc
      + row
        .iter()
        .fold(0, |acc, value| acc + if *value { 1 } else { 0 })
  });

  //println!("final grid: {:?}", final_grid);
  println!("light count: {}", light_count);
}

fn serialize_grid(grid: &Vec<Vec<bool>>) -> String {
  grid.iter().fold(String::new(), |acc, row| {
    format!(
      "{}{}\n",
      acc,
      row
        .iter()
        .map(|value| if *value { '#' } else { '.' })
        .collect::<String>()
    )
  })
}
