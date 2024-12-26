use std::io;
 
struct Candidate {
    name: String,
    exp: u32,
}

fn main() {
   let mut candidates = Vec::new();

   let mut input1 = String::new();
   println!("Number of candidates to be interviewed:"); 
   io::stdin().read_line(&mut input1).expect("Failed to read input");
   let no_of_candidates:usize = input1.trim().parse().expect("Invalid input");

   for i in 0..no_of_candidates {
     println!("Enter candidate no. {} details:",i+1);
     
     let mut input2 = String::new();
     println!("Name:");
     io::stdin().read_line(&mut input2).expect("Failed to read input");
     let name = input2.trim().to_string();

     let mut input3 = String::new();
     println!("How many years experience?");
     io::stdin().read_line(&mut input3).expect("Failed to read input");
     let exp:u32 = input3.trim().parse().expect("Not a valid input");

     candidates.push(Candidate{name,exp});  
   }
    
    let mut max_experience = 0;
    let mut most_experienced = "".to_string();

    for candidate in &candidates {
        if candidate.exp > max_experience {
            max_experience = candidate.exp;
            most_experienced = candidate.name.clone();
 
        }
        
    }
    println!("The candidate {} with {} years of experience is the most experienced",most_experienced, max_experience);
}
