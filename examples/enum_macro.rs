use macros::EnumFrom;

#[allow(unused)]
#[derive(EnumFrom, Debug)]
enum Direction<T> {
  Up(DirectionUp<T>),
  Down,
}

#[allow(unused)]
#[derive(Debug)]
struct DirectionUp<T> {
  speed: T,
}

fn main() {
  let up: Direction<i32> = DirectionUp::new(32).into();
  println!("{:?}", up);
}

impl<T> DirectionUp<T> {
  fn new(speed: T) -> Self {
    Self { speed }
  }
}