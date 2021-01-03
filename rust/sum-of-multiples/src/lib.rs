pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    if factors.len() == 0 {
        return 0;
    }
    let mut sum = 0;
    for i in 0..limit {
        for factor in factors.iter() {
            match factor {
                0 => break,
                _ => {
                    if i % factor == 0 {
                        sum += i;
                        break;
                    }
                }
            }
        }
    }
    sum
}
