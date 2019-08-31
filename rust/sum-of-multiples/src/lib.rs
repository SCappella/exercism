use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut heap: BinaryHeap<_> = factors
        .iter()
        .filter(|&&x| 0 < x && x < limit)
        .map(|&x| Reverse((x, x)))
        .collect();
    let mut sum = 0;

    while let Some(Reverse((next, multiple))) = heap.pop() {
        sum += next;
        if next + multiple < limit {
            heap.push(Reverse((next + multiple, multiple)));
        }
        while heap.peek().map(|x| (x.0).0) == Some(next) {
            let Reverse((next, multiple)) = heap.pop().expect("We just confirmed that this is Some");
            if next + multiple < limit {
                heap.push(Reverse((next + multiple, multiple)));
            }
        }
    }

    sum
}
