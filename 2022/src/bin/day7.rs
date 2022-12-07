use std::collections::HashMap;
use utils;

#[derive(Debug, Clone, PartialEq)]
struct ElfFile {
  path: Vec<String>,
  name: String,
  size: usize,
}

fn main() {
  let lines = utils::read_input_file_lines();

  let mut files: Vec<ElfFile> = Vec::new();
  let mut path: Vec<String> = ["".to_string()].to_vec();

  for line in lines {
    let tokens: Vec<&str> = line.split(" ").collect();

    if tokens[0] == "$" {
      if tokens[1] == "cd" {
        match tokens[2] {
          "/" => {
            path = ["".to_string()].to_vec();
          }
          ".." => {
            path.pop();
          }
          dir_name => {
            path.push(dir_name.to_string());
          }
        }
      }
    } else {
      if tokens[0] != "dir" {
        files.push(ElfFile {
          path: path.iter().map(|s| s.to_string()).collect(),
          name: tokens[1].to_string(),
          size: tokens[0].parse().expect("invalid file size"),
        });
      }
    }
  }

  let mut dir_sizes: HashMap<String, usize> = HashMap::new();
  let mut total_size: usize = 0;

  for file in files {
    let mut pathname = "".to_string();
    total_size += file.size;
    for part in file.path {
      pathname += &(part + "/");
      let size = dir_sizes.get(&pathname).unwrap_or(&0);
      dir_sizes.insert(pathname.clone(), size + file.size);
    }
  }

  let mut total: usize = 0;
  for size in dir_sizes.values() {
    if size <= &100000 {
      total += size;
    }
  }

  println!("size total {}", total_size);

  let total_space = 70000000;
  let space_needed = 30000000;

  let diff = space_needed - (total_space - total_size);

  let mut lowest = total_space;
  let mut best_dir = "/".to_string();

  for (name, size) in dir_sizes {
    if size >= diff && size < lowest {
      lowest = size;
      best_dir = name.clone();
    }
  }

  println!("should delete {} {}", best_dir, lowest);
}
