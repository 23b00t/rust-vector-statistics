use vector_statistics::statistics;

fn main() {
    let mut input: Vec<i32> = vec![23, 8, 7, 5, 23, 23, 5];

    // mutable borrow of input to be able to use it in both functions
    let median = statistics::calc_median(&mut input);
    let modus = statistics::calc_modus(&mut input);

    println!("Median: {}", median);
    println!("Modus: {:?}", modus);
}

