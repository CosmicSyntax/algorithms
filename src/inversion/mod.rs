use std::slice;

pub fn invert(ar: *mut i32, n: usize) -> u32 {
    /*
        Implementing sorting algorithm was a bit troublsome using rc and refcell.
        It required quite a number of borrow and borrow_mut() methods.
        Instead used unsafe and within a safe abstraction. Code is not optimized.
    */
    unsafe {
        let mid = ((n as f32) / 2 as f32).ceil() as usize;

        let left = slice::from_raw_parts_mut(ar, mid);
        let mut left: Vec<i32> = left.iter().cloned().collect();
        let right = slice::from_raw_parts_mut(ar.offset(mid as isize), n - mid);
        let mut right: Vec<i32> = right.iter().cloned().collect();

        let (ilim, jlim) = (left.len(), right.len());

        let mut cnt = 0;

        if ilim > 1 {
            cnt += invert(left.as_mut_ptr(), ilim);
        }
        if jlim > 1 {
            cnt += invert(right.as_mut_ptr(), jlim);
        }

        let (mut i, mut j) = (0, 0);

        for k in 0..n {
            if jlim > j && ilim > i {
                if left[i] > right[j] {
                    *ar.offset(k as isize) = right[j];
                    j += 1;
                    cnt += (ilim - i) as u32;
                } else if left[i] == right[j] {
                    // edge case where equals
                    *ar.offset(k as isize) = right[j];
                    j += 1;
                } else {
                    *ar.offset(k as isize) = left[i];
                    i += 1;
                }
            } else {
                if jlim == j {
                    *ar.offset(k as isize) = left[i];
                    i += 1;
                } else if ilim == i {
                    *ar.offset(k as isize) = right[j];
                    j += 1;
                }
            }
        }

        return cnt;
    }
}

// Unit Testing

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_order() {
        let mut start = [9, 1, 2, 8, 10, 10];
        let end = [1, 2, 8, 9, 10, 10];

        let _ = invert(start.as_mut_ptr(), 5usize);

        assert_eq!(end, start);
    }

    #[test]
    fn check_invert() {
        let mut start = [9, 1, 2, 8, 10];

        let n = invert(start.as_mut_ptr(), 5usize);

        assert_eq!(3, n);
    }
}
