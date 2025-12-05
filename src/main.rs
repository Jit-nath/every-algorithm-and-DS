use every_algorithm_and_ds::algorithms::sorting::quick_sort::quick_sort;


fn main() {
    let mut arr: [i32; 6] = [6, 5, 4, 3, 2, 1];

    quick_sort(&mut arr);

    println!("{:?}", arr);
}
