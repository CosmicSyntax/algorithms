mod closest;
mod insertion;
mod inversion;
mod quicksort;

fn main() {
    // This portion of the program takes an arbitrary array
    // sorts it using mergesort and returns the number of
    // inversion that exists in the array

    let mut array = [9, 3, 1, 8, 2];
    println!("Original Array: {:?}", array);

    let n = array.len();
    let inverse = inversion::invert(array.as_mut_ptr(), n);

    println!("Sorted through MergeSort: {:?}", array);
    println!("Number of inversions: {}\n", inverse);

    // This portion of the program takes an vector of 2D points
    // and finds the closest pair

    let mut mp = closest::Points::new();
    mp.add(2, 3);
    mp.add(5, 4);
    mp.add(9, 10);
    println!("{}\n", mp);

    // QuickSort implementation;

    let mut array = [9, 3, 1, 8, 2];
    let n = array.len();
    println!("Original Array: {:?}", array);
    quicksort::quicksort(&mut array, 0, n - 1);
    println!("Sorted through Quicksort: {:?}\n", array);

    // Insertion Sort Implementation

    let mut array = [9, 3, 1, 8, 2, 0];
    println!("Original Array: {:?}", array);
    insertion::insertion(&mut array);
    println!("Sorted through insertion sort: {:?}\n", array);
}
