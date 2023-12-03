fn main() {
    let file = include_str!("input");
    let part1 = day3::part_one::calculate(file);
    println!("Part 1: {part1}");

    let part2 = day3::part_two::calculate(file);
    println!("Part 2: {part2}");
}
