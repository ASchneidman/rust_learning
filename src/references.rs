pub fn references() {
  let starship: Option<String> = Some("Omaha".to_string());

  match starship {
    Some(ref name) => println!("{}" , name),
    None => {}
  }

  let planet = "Earth";
  let earth = &&&&planet;

  assert_eq!("EARTH", earth.to_uppercase());

  // let plan: str = "hmm"; -> because strs are persistent, need to access it as a reference
  let plan: &str = "hmm";
  println!("{}", plan);

  let x = 100;
  println!("i32 reference to 100 value: {}", &x);

  let ref reference_to_plan = plan;
  println!("Making reference to plan (which is ts own ref str to hmm): {}", reference_to_plan);
}