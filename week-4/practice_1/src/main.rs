  // Rust program to output name and age

  use std::io;

  fn main() {
      println!("\nStudent Information Management System!");

       // input name
       println!("\nPlease Enter your name.");
       let mut name = String::new();
           io::stdin()
           .read_line(&mut name)
           .expect("Shame on you");
        println!("Your name is: {}", name);
        
        // input age 
        println!("\nEnter your age.");
        let mut age = String::new();
            io::stdin().read_line(&mut age).expect("Shame on you");
        let age:i32 = age.trim().parse().expect("Input is not an integer");
        println!("Your age is: {}", age);    

  }