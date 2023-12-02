fn main() {
    let file = include_str!("input");
    let part1 = day2::part_one::calculate(file);
    println!("Part 1: {part1}");

    match day2::part_two::calculate(file) {
        Some(n) => println!("Part 2: {n}"),
        None => println!("Part 2 Error!!"),
    }
}
