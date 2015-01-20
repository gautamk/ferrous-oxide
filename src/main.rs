fn print_number(i: i32){
    println!("{}",i);
}
fn main(){
    let x = 4;

    let y: i32 = if x == 5 { 10 } else { 15 };

    print_number(y)
}
