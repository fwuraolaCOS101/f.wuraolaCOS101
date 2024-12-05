 use std::io;

 fn main() {
     let mut input1 = String::new();
     let mut input2 = String::new();
     let mut input3 = String::new();

     println!("Enter value, a: {}",input1);
     io::stdin().read_line(&mut input1).expect("Not a valid number");
     let a:f32 = input1.trim().parse().expect("Not a valid number");

     println!("Enter value, b: {}",input2);
     io::stdin().read_line(&mut input2).expect("Not a valid input");
     let b:f32 = input2.trim().parse().expect("Not a valid input");

     println!("Enter value, c: {}",input3);
     io::stdin().read_line(&mut input3).expect("Not a valid response");
     let c:f32 = input3.trim().parse().expect("Not a valid response");

     let d:f32 = b.powf(2.0) - (4.0)*a*c;
     if d > 0.0 
     {
        println!("\nThe discriminant is {}. Therefore, the equation has two distinct roots",d);
     }
     else if d < 0.0 
     {
        println!("\nThe discriminant is {}. Therefore, the equation has no real roots",d);
     } 
     else if d==0.0 
     {
        println!("\nThe discriminant is {}. Therefore, the equation has two equal roots",d);
     }

}

