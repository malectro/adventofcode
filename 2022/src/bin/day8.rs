use utils;

type TreeHeights = Vec<Vec<isize>>;

fn main() {
  let lines = utils::read_input_file_lines();

  let grid: Vec<Vec<isize>> = lines
    .map(|line| line.bytes().map(|n| (n as isize) - 48).collect())
    .collect();

  part_1(&grid);
  part_2(&grid);
}

fn part_1(grid: &TreeHeights) {
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

fn check_rows(grid: &TreeHeights, visibility: &mut Vec<Vec<bool>>, y_dir: isize, x_dir: isize) {
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

fn check_cols(grid: &TreeHeights, visibility: &mut Vec<Vec<bool>>, y_dir: isize, x_dir: isize) {
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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Tree {
  up: usize,
  down: usize,
  left: usize,
  right: usize,
}

fn part_2(grid: &TreeHeights) {
  let mut forest: Vec<Vec<Tree>> = grid
    .iter()
    .map(|row| {
      vec![
        Tree {
          up: 0,
          down: 0,
          left: 0,
          right: 0
        };
        row.len()
      ]
    })
    .collect();

  for (y, trees) in forest.iter_mut().enumerate() {
    for (x, tree) in trees.iter_mut().enumerate() {
      get_view(&grid, tree, x, y);
    }
  }

  let mut top_score = 0;
  for trees in forest {
    for tree in trees {
      let score = tree.left * tree.up * tree.right * tree.down;
      if score > top_score {
        top_score = score;
      }
    }
  }

  println!("top score {}", top_score);
}

fn get_view(grid: &Vec<Vec<isize>>, tree: &mut Tree, x: usize, y: usize) {
  let tree_height = grid[y][x];

  let mut view_count = 0;
  for i in (0..x).rev() {
    view_count += 1;
    if grid[y][i] >= tree_height {
      break;
    }
  }
  tree.left = view_count;

  view_count = 0;
  for i in (x + 1)..grid[0].len() {
    view_count += 1;
    if grid[y][i] >= tree_height {
      break;
    }
  }
  tree.right = view_count;

  view_count = 0;
  for i in (y + 1)..grid.len() {
    view_count += 1;
    if grid[i][x] >= tree_height {
      break;
    }
  }
  tree.down = view_count;

  view_count = 0;
  for i in (0..y).rev() {
    view_count += 1;
    if grid[i][x] >= tree_height {
      break;
    }
  }
  tree.up = view_count;
}
