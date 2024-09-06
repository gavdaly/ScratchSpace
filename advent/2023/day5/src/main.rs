mod part_one;
mod part_two;

fn main() {
    let file = include_str!("input");
    match part_one::calculate(file) {
        Some(a) => println!("Part one:\n{a}"),
        None => println!("Error: Not valid"),
    }

    println!("{}", part_two::calculate(file));
}
