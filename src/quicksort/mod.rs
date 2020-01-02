pub fn quicksort(a: &mut [i32], l: usize, h: usize) {
    if l < h {
        let p = partition(a, l, h);
        quicksort(a, l, p - 1); // left of pivot
        quicksort(a, p + 1, h); // right of pivot
    }
}

fn partition(a: &mut [i32], l: usize, h: usize) -> usize {
    let index = h;
    let pivot = a[index];
    let mut i = l;

    for j in l..h {
        if a[j] < pivot {
            a.swap(i, j);
            i += 1;
        }
    }

    a.swap(i, index);

    i
}

// Unit Testing

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_order() {
        let mut start = [9, 1, 2, 8, 10, 10];
        let end = [1, 2, 8, 9, 10, 10];
        let n = start.len();

        quicksort(&mut start, 0, n - 1);

        assert_eq!(end, start);
    }
}
