extern crate projects;
use projects::sieve_of_eratosthenes;
fn main(){
	println!("{:?}", sieve_of_eratosthenes::sieve() );
}