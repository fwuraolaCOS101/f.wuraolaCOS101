use std::io;

fn main() {
	let mut input1 = String::new();
	let mut input2 = String::new();
	let mut input3 = String::new();
	let mut input4 = String::new();
	let mut input4 = String::new();
	let mut input5 = String::new();
	let mut input6 = String::new();
	let mut input7 = String::new();
	let mut input8 = String::new();

	println!("Enter your name: ");
	io::stdin().read_line(&mut input1).expect("Not a valid string");

	println!("Enter your year of birth:  ");
	io::stdin().read_line(&mut input2).expect("Not a valid input");
	let age:u64 = input2.trim().parse().expect("Not a valid input");

	let age = 2024 - age;
	println!("Age = {}",age);

	println!("Enter your email address:  ");
	io::stdin().read_line(&mut input3).expect("Not a valid string");

	println!("Enter your phone number:  ");
	io::stdin().read_line(&mut input4).expect("Not a valid input");
	let phone_number:u128 = input4.trim().parse().expect("Not a valid input");

	println!("How many siblings do you have?:  ");
	io::stdin().read_line(&mut input5).expect("Not a valid input");
	let no_of_siblings:u32 = input5.trim().parse().expect("Not a valid input");

    println!("How many children do you have?:  ");
    io::stdin().read_line(&mut input6).expect("Not a valid input");
    let no_of_children:u32 = input6.trim().parse().expect("Not a valid input");

    println!("What medical condition do you have? ");
    io::stdin().read_line(&mut input7).expect("Not a valid input");
    let mut medical_condition = input7; 

    println!("What village do you reside in? ");
    io::stdin().read_line(&mut input8).expect("Not a valid input");
    let mut village = input8;

    let discount:f64 = amount * d;


    if medical_condition== Alzheimer && age>50 && no_of_children>4 && village==Akpabom; 
    {
       let d = 0.02;
       let amount = 1_200_000;
        println!("Your calculated amount is: {}",discount);
    }
        else {
        	println!("Your calculated amount is: {}",amount);
        }
    
    if medical_condition== Arrythmia && age==30 && no_of_siblings>4 && village==Nbauji; 
    {
       let d = 0.05;
       let amount = 550_000;
          println!("Your calculated amount is: {}",discount);
    }
        else {
        	println!("Your calculated amount is: {}",amount);
        }
}