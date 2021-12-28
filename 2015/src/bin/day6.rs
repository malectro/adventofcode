use itertools::Itertools;
use regex::Regex;
use std::cmp;
use utils;

#[derive(Clone, Copy, Eq, PartialEq)]
struct Point {
  x: usize,
  y: usize,
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct Rect {
  origin: Point,
  end: Point,
}

fn main() {
  let lines = utils::read_input_file_lines();

  let mut rects = Vec::new();
  let re = Regex::new(r"^(?P<command>turn on|turn off|toggle) (?P<ox>\d+),(?P<oy>\d+) through (?P<ex>\d+),(?<ey>\d+)$").expect("Invalid regex");

  for line in lines {
    let captured = re.captures(line.as_ref()).expect("Invalid line");
    let new_rect = Rect {
      origin: Point {
        x: captured["ox"].parse().expect("Invalid origin x"),
        y: captured["oy"].parse().expect("Invalid origin y"),
      },
      end: Point {
        x: captured["ex"].parse().expect("Invalid end x"),
        y: captured["ey"].parse().expect("Invalid end y"),
      },
    };

    let mut new_rects = Vec::new();
    let mut toggle_rects = vec![new_rect];
    for rect in rects {
      let mut new_toggle_rects = Vec::new();
      for new_rect in toggle_rects {
        if let Some(intersection) = get_intersection(rect, &new_rect) {
          new_rects.extend(split_rect(rect, &intersection).filter(|r| r != intersection));
          if captured["command"] == "toggle" {
            new_toggle_rects.extend(split_rect(new_rect, &intersection));
          }
        } else {
          new_rects.push(*rect);
          new_toggle_rects.push(new_rect);
        }
      }
      toggle_rects = new_toggle_rects;
    }

    match &captured["command"] {
      "turn on" => {
        new_rects.push(new_rect);
      }
      "toggle" => {
        new_rects.append(toggle_rects);
      }
      _ => {}
    }

    rects = new_rects;
  }
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

  if origin.x > end.x || origin.y > end.y {
    return None;
  }
  Some(Rect { origin, end })
}

fn split_rect<'a>(r1: &'a Rect, r2: &'a Rect) -> impl Iterator<Item = Rect> + 'a {
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
    })
    .filter(is_valid_rect)
}

fn is_valid_rect(rect: &Rect) -> bool {
  match rect {
    Rect { origin, end } if origin.x > end.x || origin.y > end.y => false,
    _ => true,
  }
}
