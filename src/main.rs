use std::collections::HashMap;

fn main() {
    let mut input: Vec<i32> = vec![23, 8, 7, 5, 23, 23, 5];

    let median = calc_median(&mut input);
    let modus = calc_modus(&mut input);

    println!("Median: {}", median);
    println!("Modus: {:?}", modus);
}

fn calc_median(list: &mut Vec<i32>) -> f64 {
    list.sort();
    let len = list.len();
    if len % 2 == 0 {
        let index = len / 2;
        let sum: f64 = (list[index] + list[index - 1]).into();
        sum / 2.0
    } else {
        let index = (len - 1) / 2;
        list[index] as f64
    }
}

fn calc_modus(list: &mut Vec<i32>) -> Vec<i32> {
    let mut element_map: HashMap<i32, i32> = HashMap::new();
    list.sort();

    for element in list {
        let count = element_map.entry(*element).or_insert(0);
        *count += 1;
    }

    let max_value = element_map.values().max().cloned().unwrap_or(0);

    element_map
        .iter()
        .filter(|(_, &v)| v == max_value)
        .map(|(k, _v)| *k)
        .into_iter()
        .collect()
}
