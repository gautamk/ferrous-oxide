fn compute_sum(x:i32, y:i32) -> i32{
    if x > y  { return x + y ;}
    y + x
}
fn main(){
    let x = 4;

    let y: i32 = if x == 5 { 10 } else { 15 };

    println!("{}", compute_sum(x,y));
}
