use std::io;

fn main() {

    println!("Welcome to Golden Restaurant");
    println!("Below is the menu and their code names: ");
    println!();
    println!("p = Pounded yam/Edikaikong soup");
    println!("f = Fried Rice & ");
    println!("a = Amala & Ewedu Soup");
    println!("e = Eba & Egusi Soup");
    println!("w = White rice & Stew");

    let mut in1 = String::new();
    let mut in2 = String::new();

    println!("What would you like to order? ");
    io::stdin().read_line(&mut in1).expect("Not a valid code name");
    let food_choice:i64 = in1.trim().parse().expect("Not a valid code name");
    
    let p:i64 = 3_200;
     let f:i64 = 3_000;
     let a:i64 = 2_500;
     let e:i64 = 2_000;
     let w:i64 = 2_500;

    println!("Quantity of food ordered: ");
    io::stdin().read_line(&mut in2).expect("Failed to read input");
    let quantity:i64 = in2.trim().parse().expect("Failed to read input");


    let mut price:i64 = food_choice * quantity;

    if food_choice == p 
    {
        price = &p * quantity;
        println!("Your price is: {}",price);
    }
    else if food_choice == f 
    {
        price = &f * quantity;
        println!("Your price is: {}",price);
    }
    else if food_choice==a
    {
      price = &a * quantity;
      println!("Your price is: {}",price);
    }
    else if food_choice==e
    {
      price = &e * quantity;
      println!("Your price is: {}",price);
    }
    else if food_choice==w
    {
      price = &w * quantity;
      println!("Your price is: {}",price);
    }
    else 
    {
      println!("Sorry, that is unavailable...");
    }
}
