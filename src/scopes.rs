struct Stringy {
    pub pretend_string: String,
}

pub fn scopes() {
    let number = 10;
    {
        let number = 22;
        println!("number set by block: {}", number);
    }
    println!("number set outside of block: {}", number);
    
    
    fn abc() -> String {
        // Transfering abc to String
        "abc".to_string()
    }
    
    let letters = abc();
    let cloned_letters = abc().clone();
    
    println!("{}", letters);
    println!("{}", cloned_letters);

    let mut stringy = Stringy { pretend_string: "hmm".to_string() };
    stringy.pretend_string += "hmm";
    println!("{}", stringy.pretend_string);
}