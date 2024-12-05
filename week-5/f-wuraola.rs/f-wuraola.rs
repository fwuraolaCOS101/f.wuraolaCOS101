 use std::io;

 fn main() {
      let mut input = String::new();
      
      println!("Do you have siblings? (true/false)");
      io::stdin().read_line(&mut input).expect("Not a valid response");
      let quest:bool = input.trim().parse().expect("Not a valid response");

      if quest==true 
      {
      	println!("How many siblings do you have? ");
      io::stdin().read_line(&mut input).expect("Not a valid string");
      let no_of_siblings:u32 = input.trim().parse().expect("Not a valid input");
      
      while no_of_siblings < 5 {
      println!("Name of sibling #:  ");
      io::stdin().read_line(&mut input).expect("Not a valid response");

      println!("Age of sibling? ");
      io::stdin().read_line(&mut input).expect("Not a valid input");
      let age:u32 = input.trim().parse().expect("Not a valid response");

      println!("Gender of sibling: ");
      io::stdin().read_line(&mut input).expect("Not a valid input");

      println!("Country of residence: ");
      io::stdin().read_line(&mut input).expect("Not a valid response");
      }
     }
      if age >= 18
      {
         println!("Are you married?: (true/false)");
         io::stdin().read_line(&mut input).expect("Not a valid input");
         let marital_status:bool = input.trim().parse().expect("Not a valid response");

         if marital_status==true
         {
             println!("Do you have children? (true/false) ");
             io::stdin().read_line(&mut input).expect("Not a valid response");
             let children:bool = input.trim().parse().expect("Not a valid input");

         if children==true 
         {
            println!("Name, age, school/daycare?");
            io::stdin().read_line(&mut input).expect("Not a valid response");
         }
         }
         else if marital_status==false
         {
            println!("Are you a student or employed? true=student false=employed");
            io::stdin().read_line(&mut input).expect("Not a valid response");
            let student_or_employed:bool = input.trim().parse().expect("Not a valid input");
    
          if student_or_employed==true
          {
          	println!("University, course of study and year of study: ");
          	io::stdin().read_line(&mut input).expect("Not a valid response");

          	println!("Are you studying in your home country or abroad? (true=home/false=abroad) ");
          	io::stdin().read_line(&mut input).expect("Not a valid response");
          	let home_or_abroad:bool = input.trim().parse().expect("Not a valid response");

          	if home_or_abroad==false 
          	{
              println!("Country?: ");
              io::stdin().read_line(&mut input).expect("Not a valid response");
          	}
          }
          else if student_or_employed==false
          {
          	println!("Is the job remote, on-site or hybrid? true=on-site/false=others ");
          	io::stdin().read_line(&mut input).expect("Not a valid response");
          	let job_site:bool = input.trim().parse().expect("Not a valid response");

          	if job_site==true
          	{
              println!("Company name, job title, industry sector? ");
              io::stdin().read_line(&mut input).expect("Not a valid response");
          	}
          	println!("Are you in a relationship? (true/false)");
          	io::stdin().read_line(&mut input).expect("Not a valid response");
          	let relationship:bool = input.trim().parse().expect("Not a valid response");

          	if relationship==true 
          	{
              println!("How many years? ");
              io::stdin().read_line(&mut input).expect("Not a valid response");
              let years:u32 = input.trim().parse().expect("Not a valid response");

              println!("Partner's first name: ");
              io::stdin().read_line(&mut input).expect("Not a valid response");
          	}
          }
         }
      }
       else
       {
          println!("Thank you for your response!");
       }
}        