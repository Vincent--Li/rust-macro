use macros::EnumFrom;

#[derive(EnumFrom, Debug)]
enum Direction {
  Up(DirectionUp),
  Down,
}

#[derive(Debug)]
struct DirectionUp {
  speed: u32,
}

fn main() {
  let up = Direction::Up(DirectionUp { speed: 42 });
  println!("{:?}", up);
}