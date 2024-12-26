use std::io::Write;

fn main() {
    let line_a = vec!["Student name  ","  Matric Number  ","  Department  ","  Level  "];
    let line_b = vec!["\nOluchi Mordi  ","   ACC10211111   ","  Accounting  ","  300  "];
    let line_c = vec!["\nAdams Aliyu   ","   ECO10110101   ","  Economics   ","  100  "];
    let line_d = vec!["\nShania Bolade ","   CSC10328828   ","  Computer    ","  200  "];
    let line_e = vec!["\nAdekunle Gold ","   EEE11020202   ","  Electrical  ","  200  "];
    let line_f = vec!["\nBlanca Edemoh ","   MEE10202001   ","  Mechanical  ","  100  "];

    let mut file = std::fs::File::create("PAU-SMIS.txt").expect("Invalid file name");
    file.write_all("PAU-SMIS\n".as_bytes()).expect("Write failed");
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
    println!("Data written to file");
}
