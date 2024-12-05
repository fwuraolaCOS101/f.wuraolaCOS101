fn main() {
    let num:i32 = 5;
    mutate_num_to_one(num);
    println!("The value of no is: {}",num);
}
 fn mutate_num_to_one(mut param_num: i32) {
    param_num = param_num*0;
    println!("param_num value is: {}",param_num);
 }