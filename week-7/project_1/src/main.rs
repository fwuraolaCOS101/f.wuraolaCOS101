use std::io;

fn area_of_trapezium(h:i32, b1:i32, b2:i32) {
let area1 = h/2 * (b1 + b2);
println!("Area of trapezium is: {}",area1);
}

fn area_of_rhombus(d1:i32, d2:i32) {
let area2 = (d1 * d2)/2;
println!("Area of Rhombus is: {}",area2);
}

fn area_of_parallelogram(b:i32, a:i32) {
let area3 = b * a;
println!("Area of Parallelogram is: {}",area3);    
}

fn area_of_cube(l:i32) {
let area4 = 6 * l.pow(2);
println!("Area of cube is: {}",area4);    
}

fn volume_of_cylinder(r:i32, h1:i32) {
let volume = (22 * r.pow(2) * h1)/7;
println!("Volume of cube is: {}",volume);
}

fn main() {
    
    let mut input1 = String::new();

    println!("Choose an equation");
    println!("1. Area of Trapezium formula = h/2*(b1+b2)");
    println!("2. Area of Rhombus formula = 1/2*d1*d2");
    println!("3. Area of Parallelogram formula = b*a");
    println!("4. Area of cube = 6*l^2");
    println!("5. Volume of cylinder = 22/7*r^2*h1", );
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let choice:i32 = input1.trim().parse().expect("Invalid input");

    if choice== 1
    {
        let mut input2 = String::new();
      println!("Input height: ");
      io::stdin().read_line(&mut input2).expect("Failed to read input");
      let h:i32 = input2.trim().parse().expect("Invalid input");
      
       let mut input3 = String::new();
      println!("Input base1: ");
      io::stdin().read_line(&mut input3).expect("Invalid input");
      let b1:i32 = input3.trim().parse().expect("Failed to read input");
      
       let mut input4 = String::new();
      println!("Input base2: ");
      io::stdin().read_line(&mut input4).expect("Failed to read input");
      let b2:i32 = input4.trim().parse().expect("Invalid input");

      area_of_trapezium(h, b1, b2);
    }
    else if choice == 2 {
        let mut input5 = String::new();
        println!("Input diagonal1: ");
        io::stdin().read_line(&mut input5).expect("Failed to read input");
        let d1:i32 = input5.trim().parse().expect("Invalid input");

        let mut input6 = String::new();
        println!("Input diagonal2: ");
        io::stdin().read_line(&mut input6).expect("Failed to read input");
        let d2:i32 = input6.trim().parse().expect("Invalid input");

        area_of_rhombus(d1, d2);
    }
    else if choice == 3 {
        let mut input7 = String::new();
        println!("Input base: ");
        io::stdin().read_line(&mut input7).expect("Failed to read input");
        let b:i32 = input7.trim().parse().expect("Invalid input");

        let mut input8 = String::new();
        println!("Input area: ");
        io::stdin().read_line(&mut input8).expect("Failed to read input");
        let a:i32 = input8.trim().parse().expect("Invalid input");

        area_of_parallelogram(b, a);
    }
    else if choice == 4 {
        let mut input9 = String::new();
        println!("Input length: ");
        io::stdin().read_line(&mut input9).expect("Failed to read input");
        let l:i32 = input9.trim().parse().expect("Invalid input");

        area_of_cube(l);
    }
    else if choice == 5 {
        let mut input10 = String::new();
        println!("Input radius: ");
        io::stdin().read_line(&mut input10).expect("Failed to read input");
        let r:i32 = input10.trim().parse().expect("Invalid input");

        let mut input11 = String::new();
        println!("Input height1: ");
        io::stdin().read_line(&mut input11).expect("Failed to read input");
        let h1:i32 = input11.trim().parse().expect("Invalid input");

        volume_of_cylinder(r, h1);
    }
    else {
        println!("Sorry big head");
    }
}