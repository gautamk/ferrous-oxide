const MAX: usize = 100000;


fn mark_multiples(num: &mut[bool; MAX], multiples_of: usize){
    for i in (2..MAX) {
        let multiple = multiples_of * i;
        if multiple < MAX {
            // println!("\t marking {:?} as not prime ", multiple);
            num[multiple] = false;
        }
    }
}

pub fn sieve() -> [bool; MAX] {
    let mut num = [true; MAX];
    num[0] = false;
    num[1] = false;
    for i in (0..MAX) {
        if num[i] {
            // println!("{:?} is prime", i);
            mark_multiples(&mut num, i);
        } else {
            // println!("{:?} is not prime", i);
        }
    }
    return num;
}