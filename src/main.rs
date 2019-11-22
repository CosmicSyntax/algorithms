mod inversion;
use crate::inversion::invert;

fn main() {

    // This portion of the program takes an arbitrary array
    // sorts it using mergesort and returns the number of
    // inversion that exists in the array

    let mut array = [9, 3, 1, 8, 2];
    println!("Original Array: {:?}", array);

    let n = array.len();
    let inverse = invert(array.as_mut_ptr(), n);

    println!("Sorted through MergeSort: {:?}", array);
    println!("Number of inversions: {}", inverse);
}
