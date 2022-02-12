use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    (num > 0).then(|| {
        match (1..num)
            // For performance reasons, we only look for factors below and equal to sqrt(num)...
            .take_while(|k| k * k <= num)
            .filter(|k| num % k == 0)
            // ...and use them to derive the remaining factors.
            .map(|k| k + if k > 1 && k * k < num { num / k } else { 0 })
            .sum::<u64>()
            .cmp(&num)
        {
            Ordering::Less => Classification::Deficient,
            Ordering::Equal => Classification::Perfect,
            Ordering::Greater => Classification::Abundant,
        }
    })
}
