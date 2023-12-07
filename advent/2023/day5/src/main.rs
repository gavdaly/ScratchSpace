mod part_one;
mod part_two;

fn main() {
    let file = include_str!("input");
    println!("{}", part_one::calculate(file));
    println!("{}", part_two::calculate(file));
}
