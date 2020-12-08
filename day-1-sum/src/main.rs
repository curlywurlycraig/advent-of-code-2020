use itertools::Itertools;

mod input;
use input::input;

fn main() {
    let part_one_result = input
        .iter()
        .tuple_combinations()
        .filter(|(a, b)| *a + *b == 2020)
        .map(|(a, b)| a * b)
        .next()
        .unwrap();

    println!("Result: {}", &part_one_result);

    let part_two_result = input
        .iter()
        .tuple_combinations()
        .filter(|(a, b, c)| *a + *b + *c == 2020)
        .map(|(a, b, c)| a * b * c)
        .next()
        .unwrap();

    println!("Part two result: {}", &part_two_result);
}
