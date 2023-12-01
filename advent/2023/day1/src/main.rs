fn main() {
    let file = include_str!("input");
    match day1::part_one::sum_lines(file) {
        Some(n) => println!("Part 1: {n}"),
        None => println!("Part 1 Error!!"),
    };
    match day1::part_two::sum_lines(file) {
        Some(n) => println!("Part 2: {n}"),
        None => println!("Part 2 Error!!"),
    }
}
