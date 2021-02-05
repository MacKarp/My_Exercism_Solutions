use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }

    match (1..=num / 2)
        .filter(|x| num % x == 0)
        .sum::<u64>()
        .cmp(&num)
    {
        Ordering::Equal => Some(Classification::Perfect),
        Ordering::Greater => Some(Classification::Abundant),
        Ordering::Less => Some(Classification::Deficient),
    }
}
