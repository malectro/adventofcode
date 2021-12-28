use itertools::Itertools;
use regex::Regex;
use std::cmp;
use utils;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Point {
  x: usize,
  y: usize,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Rect {
  origin: Point,
  end: Point,
}

fn main() {
  let lines = utils::read_input_file_lines();

  let mut rects = Vec::new();
  let re = Regex::new(r"^(?P<command>turn on|turn off|toggle) (?P<ox>\d+),(?P<oy>\d+) through (?P<ex>\d+),(?P<ey>\d+)$").expect("Invalid regex");

  for line in lines {
    let captured = re.captures(line.as_ref()).expect("Invalid line");
    let new_rect = Rect {
      origin: Point {
        x: captured["ox"].parse::<usize>().expect("Invalid origin x"),
        y: captured["oy"].parse::<usize>().expect("Invalid origin y"),
      },
      end: Point {
        x: captured["ex"].parse::<usize>().expect("Invalid end x") + 1,
        y: captured["ey"].parse::<usize>().expect("Invalid end y") + 1,
      },
    };

    let mut new_rects = Vec::new();
    let mut intersections = Vec::new();
    for rect in rects {
      if let Some(intersection) = get_intersection(&rect, &new_rect) {
        new_rects.extend(
          split_rect(&rect, &intersection)
            .into_iter()
            .filter(|r| *r != intersection),
        );
        if captured["command"] == *"toggle" {
          intersections.push(intersection);
        }
      } else {
        new_rects.push(rect);
      }
    }

    match &captured["command"] {
      "turn on" => {
        new_rects.push(new_rect);
      }
      "toggle" => {
        // break down the rect
        let mut rects = vec![new_rect];
        for intersection in intersections {
          let mut split_rects = Vec::new();
          for rect in rects {
            if let Some(new_intersection) = get_intersection(&rect, &intersection) {
              split_rects.extend(
                split_rect(&rect, &new_intersection)
                  .into_iter()
                  .filter(|r| *r != new_intersection),
              );
            } else {
              split_rects.push(rect);
            }
          }
          rects = split_rects;
        }

        new_rects.append(&mut rects);
      }
      _ => {}
    }

    rects = new_rects;
  }

  let total_area = rects.iter().fold(0, |acc, r| {
    acc + (r.end.x - r.origin.x) * (r.end.y - r.origin.y)
  });
  println!("total area {}", total_area);
}

fn get_intersection(r1: &Rect, r2: &Rect) -> Option<Rect> {
  let origin = Point {
    x: cmp::max(r1.origin.x, r2.origin.x),
    y: cmp::max(r1.origin.y, r2.origin.y),
  };
  let end = Point {
    x: cmp::min(r1.end.x, r2.end.x),
    y: cmp::min(r1.end.y, r2.end.y),
  };

  if origin.x >= end.x || origin.y >= end.y {
    return None;
  }
  Some(Rect { origin, end })
}

fn split_rect(r1: &Rect, r2: &Rect) -> Vec<Rect> {
  [r1.origin.x, r2.origin.x, r2.end.x, r1.end.x]
    .iter()
    .tuple_windows()
    .flat_map(|(ox, ex)| {
      [r1.origin.y, r2.origin.y, r2.end.y, r1.end.y]
        .iter()
        .tuple_windows()
        .map(|(oy, ey)| Rect {
          origin: Point { x: *ox, y: *oy },
          end: Point { x: *ex, y: *ey },
        })
        .collect::<Vec<Rect>>()
    })
    .filter(is_valid_rect)
    .collect()
}

fn is_valid_rect(rect: &Rect) -> bool {
  match rect {
    Rect { origin, end } if origin.x >= end.x || origin.y >= end.y => false,
    _ => true,
  }
}
