 use std::io;

fn main() {

 let mut input1 = String::new();
 let mut input2 = String::new();
 let mut incentive = 000;

 println!("Do you have experience? (true/false)");
 io::stdin().read_line(&mut input1).expect("Not a valid response");
 let experience:bool = input1.trim().parse().expect("Not a valid response");

 println!("How old are you?: ");
 io::stdin().read_line(&mut input2).expect("Not a valid response");
 let age:u32 = input2.trim().parse().expect("Not a valid response");

if experience==true
{
  if age>= 40 
  {
     incentive = 1_560_000;
  }
  else if age>=30 && age<40 
  {
      incentive = 1_480_000;
  }
  else if age<28 
  {
      incentive = 1_300_000;
  }
else {
    incentive = 0;
}  
}

if experience==false 
{
  incentive = 100_000;
} 
   println!("Your annual incentive is: {}",incentive);
}
