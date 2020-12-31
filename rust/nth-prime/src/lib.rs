pub fn nth(n: u32) -> u32 {
    let mut i: u32 = 0;
    let mut number: u32 = 0;
    loop {
        if is_prime(number) {
            if i == n {
                return number;
            }
            i += 1;
        }
        number += 1;
    }
}

fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    true
}
