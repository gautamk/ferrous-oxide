extern crate projects;
use projects::sieve_of_eratosthenes;
fn main(){
    let sieve = sieve_of_eratosthenes::sieve();
    for i in (0..sieve.len()){
        if sieve[i] {
            print!("{}, ", i);
        }
    }
}