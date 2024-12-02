use macros::EnumFrom;

#[allow(unused)]
#[derive(EnumFrom, Debug)]
enum Direction {
  Up(DirectionUp),
  Down,
}

#[allow(unused)]
#[derive(Debug)]
struct DirectionUp {
  speed: u32,
}

fn main() {
  let up: Direction = DirectionUp::new(32).into();
  println!("{:?}", up);
}

impl DirectionUp {
  fn new(speed: u32) -> Self {
    DirectionUp { speed }
  }
}