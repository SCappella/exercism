use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None
    } if num == 1 {
        return Some(Classification::Deficient)
    }

    let mut sum = 1;

    for i in 2.. {
        if i * i == num {
            sum += i;
            break;
        } else if i * i > num {
            break;
        }

        if num % i == 0 {
            sum += i;
            sum += num / i;
        }
    }

    match sum.cmp(&num) {
        Ordering::Less => Some(Classification::Deficient),
        Ordering::Equal => Some(Classification::Perfect),
        Ordering::Greater => Some(Classification::Abundant),
    }
}
