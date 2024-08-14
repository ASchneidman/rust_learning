pub fn doubles() {
  fn double(num: u128) -> u128 {
    num * 2
  }
  fn double_integer(num: i32) -> i32 {
    num * 2
  }
  fn double_float(num: f64) -> f64 {
    num * 2.0
  }

  let int: i32 = 32;
  let big_int = 10;
  let float = 1.2;

  println!("big_int doubled: {}", double(big_int));
  println!("int doubled: {}", double_integer(int));
  println!("float doubled: {}", double_float(float));
} 