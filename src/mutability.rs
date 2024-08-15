pub fn mutability() {
  // allows one to modify s passed as an argument
  fn question(s: &mut String) {
    s.pop();
    s.push('?');
  }

  let mut sentence = String::from("I am.");
  println!("{}", sentence);

  let immutable_reference = &mut sentence;
  println!("{}", immutable_reference);

  question(&mut sentence);
  println!("{}", sentence);

  fn half_float(f: &mut f64) {
    // *f dereferences the reference of f
    *f /= 2.0;
  }

  let mut val = 1.23;
  half_float(&mut val);
  println!("{}", val);
}