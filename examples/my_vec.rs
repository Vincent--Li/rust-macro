#[macro_export]
macro_rules! my_vec {
   ($($x:expr),*) => {
      {
         let mut temp_vec = Vec::new();
         $(temp_vec.push($x);)*
         temp_vec
      }
   };
}

fn main() {
    let v = my_vec! {1,3,5};
    println!("{:?}", v);
}
