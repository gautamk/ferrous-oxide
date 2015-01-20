fn main(){
    let x = 4;

    let x = (let y = 5); // expected identifier, found keyword `let`

    println!("Y is {}",y );
}
