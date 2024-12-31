use std::io;

fn main(){
    let mut in1 = String::new();
    println!("Number of siblings?");
    io::stdin().read_line(&mut in1).expect("Not a valid response");
    let no_siblings:i32 = in1.trim().parse().expect("Not a  valid response");
    
    //FOR EACH SIBLING
    for i in 0..no_siblings {
        println!("Sibling {} details:",i+1);
        let mut in2 = String::new();
        println!("First name: ");
        io::stdin().read_line(&mut in2).expect("Not a valid response");
        
        let mut in3 = String::new();
        println!("Age: ");
        io::stdin().read_line(&mut in3).expect("Not a valid input");
        let age:i32 = in3.trim().parse().expect("Not a valid input");
        
        let mut in4 = String::new();
        println!("Gender: ");
        io::stdin().read_line(&mut in4).expect("Not a valid response");
        
        let mut in5 = String::new();
        println!("Country of residence: ");
        io::stdin().read_line(&mut in5).expect("Invalid response");

    if age>=18 {
        let mut in6 = String::new();
        println!("Are you married/single/in a relationship? (Answer 1/2/3 accordingly)");
        io::stdin().read_line(&mut in6).expect("Invalid response");
        let rels:i32 = in6.trim().parse().expect("Not a valid response");
        
        if rels==1 {
            let mut in7 = String::new();
            println!("Do you have children? (true/false)");
            io::stdin().read_line(&mut in7).expect("Invalid response");
            let child:bool = in7.trim().parse().expect("Invalid response");

            let mut in8 = String::new();
            println!("How many children? ");
            io::stdin().read_line(&mut in8).expect("Invalid response");
            let no_child:i32 = in8.trim().parse().expect("Invalid response");

            for i in 0..no_child {
                println!("Child {} details: ",i+1);
                let mut in9 = String::new();
                println!("Name: ");
                io::stdin().read_line(&mut in9).expect("Invalid response");

                let mut in10 = String::new();
                println!("Age: ");
                io::stdin().read_line(&mut in10).expect("Invalid response");
                let age1:u32 = in10.trim().parse().expect("Invalid response");

                let mut in11 = String::new();
                println!("School or daycare name: ");
                io::stdin().read_line(&mut in11).expect("Invalid response");
            }
            println!("Family lives in {}",in5);
        }
        else if rels==2{
            let mut in12 = String::new();
            println!("Are you a student or employed? (Answer 1 or 2 accordingly)");
            io::stdin().read_line(&mut in12).expect("Invalid response");
            let occup:i32 = in12.trim().parse().expect("Invalid response");

            if occup== 1 {
                let mut in13 = String::new();
                println!("University: ");
                io::stdin().read_line(&mut in13).expect("Invalid response");
                
                let mut in14 = String::new();
                println!("Course of study: ");
                io::stdin().read_line(&mut in14).expect("Invalid response");
                
                let mut in15 = String::new();
                println!("Year of study: ");
                io::stdin().read_line(&mut in15).expect("Invalid response");
                
                let mut in16 = String::new();
                println!("Are you studying home or abroad? (Answer 1 or 2 accordingly)");
                io::stdin().read_line(&mut in16).expect("Invalid response");
                let place_of_study:u32 = in16.trim().parse().expect("Invalid response");

                if place_of_study==2{
                    let mut in17 = String::new();
                    println!("Country: ");
                    io::stdin().read_line(&mut in17).expect("Invalid response");
                }
            }
            else if occup== 2 {
                let mut in18 = String::new();
                println!("Is the job remote/on-site/hybrid? (Answer 1/2/3 accordingly)");
                io::stdin().read_line(&mut in18).expect("Invalid response");
                let place:i32 = in18.trim().parse().expect("Invalid response");

                if place==2 {
                    let mut in19 = String::new();
                    println!("Company name: ");
                    io::stdin().read_line(&mut in19).expect("Invalid response");

                    let mut in20 = String::new();
                    println!("Job title:");
                    io::stdin().read_line(&mut in20).expect("Invalid response");

                    let mut in21 = String::new();
                    println!("Industry sector: ");
                    io::stdin().read_line(&mut in21).expect("Invalid response");
                }
            }
        }
        else if rels== 3{
            let mut in22 = String::new();
            println!("Relationship duration(in years):");
            io::stdin().read_line(&mut in22).expect("Invalid response");

            let mut in23 = String::new();
            println!("Partner's first name: ");
            io::stdin().read_line(&mut in23).expect("Invalid response");

            let mut in24 = String::new();
            println!("Do you live with your partner? (true/false)");
            io::stdin().read_line(&mut in24).expect("Invalid response");
            let res:bool = in24.trim().parse().expect("Invalid response");

            if res==true {
                println!("Place of residence is {}",in5);
            }
        }
    }
    else if age<18 {
        let mut in25 = String::new();
        println!("Has sibling completed WAEC? (true/false) ");
        io::stdin().read_line(&mut in25).expect("Invalid response");
        let waec:bool = in25.trim().parse().expect("Invalid response");

        if waec==true{
            let mut in26 = String::new();
            println!("Secondary school attended: ");
            io::stdin().read_line(&mut in26).expect("Invalid response");

            let mut in27 = String::new();
            println!("Final grade: ");
            io::stdin().read_line(&mut in27).expect("Invalid response");

            let mut in28 = String::new();
            println!("Year of completion: ");
            io::stdin().read_line(&mut in28).expect("Invalid response");
        }
        else if waec==false{
            let mut in29 = String::new();
            println!("Current class level: ");
            io::stdin().read_line(&mut in29).expect("Invalid response");

            let mut in30 = String::new();
            println!("Do you plan to take WAEC soon? (true/false)");
            io::stdin().read_line(&mut in30).expect("Invalid response");
            let plan:bool = in30.trim().parse().expect("Invalid response");
            
            if plan==true{
                println!("Planned year: ");
            }
        }
    }
  }
}    