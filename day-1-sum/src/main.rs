use itertools::Itertools;

mod input;
use input::input;

fn main() {
    let result = input
        .iter()
        .tuple_combinations()
        .filter(|(a, b)| *a + *b == 2020)
        .map(|(a, b)| a * b)
        .next()
        .unwrap();

    println!("Result: {}", &result);
}
