use std::io::Write;

fn main() {
     let line_a = vec!["Lager         ","   Stout     ","   Non-alcoholic"];
     let line_b = vec!["\n33 Export   ","   Legend    ","   Maltina      "];
     let line_c = vec!["\nDesperados  ","   Turbo king","   Amstel Malta "];
     let line_d = vec!["\nGoldberg    ","   Williams  ","   Malta Gold   "];
     let line_e = vec!["\nGulder      ","      -      ","   Fayrouz      "];
     let line_f = vec!["\nHeineken    ","      -      ","      -         "];
     let line_g = vec!["\nStar        ","      -      ","      -         "];

     let mut file = std::fs::File::create("Nigeria Brewery Limited.txt").expect("create failed");
     for i in line_a {
        file.write_all(i.as_bytes());
    }
    for i in line_b{
        file.write_all(i.as_bytes());
    }
    for i in line_c{
        file.write_all(i.as_bytes());
    }
    for i in line_d{
        file.write_all(i.as_bytes());
    }

    for i in line_e{
        file.write_all(i.as_bytes());
    }
    for i in line_f{
        file.write_all(i.as_bytes());
    }
    for i in line_g{
        file.write_all(i.as_bytes());
    }
     println!("Data written to file.");

}