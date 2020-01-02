// Insertion Sort O(n^2)

pub fn insertion(a: &mut [i32]) {
    let length = a.len();
    for i in 1..length {
        let key = a[i];
        for j in (0..i).rev() {
            if a[j] > key {
                a.swap(j, j + 1);
            }
        }
    }
}

// Unit Testing

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_order() {
        let mut start = [9, 1, 2, 8, 10, 10, 0];
        let end = [0, 1, 2, 8, 9, 10, 10];

        insertion(&mut start);

        assert_eq!(end, start);
    }
}
