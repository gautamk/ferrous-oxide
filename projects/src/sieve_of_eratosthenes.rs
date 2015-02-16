static MAX: i32 = 1000;


fn mark_multiples(num: Vec<bool> ,multiples_of: i32){
    for i in range(2, MAX){
        if(multiples_of * i <= MAX){
            num.insert(i, false)
        }
    }
}

fn sieve() -> Vec<bool>{
    let mut num: Vec<bool> = Vec::with_capacity(MAX);
    num.insert(0, false);
    num.insert(1, false);
    num.insert(2, true);
    
    for i in range(2, MAX) {
        if(num[i]){
            mark_multiples(num, i)
        }
    }
    num;
}