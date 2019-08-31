use std::borrow::Borrow;

pub fn find<T: Ord, V: AsRef<[T]>, U: Borrow<T>>(array: V, key: U) -> Option<usize> {
    let array = array.as_ref();
    let mut len = array.len();
    if len == 0 {
        return None;
    }
    let mut start = 0;

    while len > 1 {
        let mid = start + len / 2;
        // mid < 0 + len/2 + len/4 + ... + len/2**n < len
        let mid_val = unsafe { array.get_unchecked(mid) };
        if mid_val <= key.borrow() {
            start = mid;
        }
        len -= len / 2;
    }
    // start <= mid
    let val = unsafe { array.get_unchecked(start) };
    if val == key.borrow() {
        Some(start)
    } else {
        None
    }
}
