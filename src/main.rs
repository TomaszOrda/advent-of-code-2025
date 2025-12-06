mod utils;

mod day1part1; mod day1part2;
mod day2part1; mod day2part2;
mod day3part1; mod day3part2;
mod day4part1; mod day4part2;
mod day5part1; mod day5part2;
mod day6part1; mod day6part2;
mod day7part1; mod day7part2;
mod day8part1; mod day8part2;
mod day9part1; mod day9part2;
mod day10part1; mod day10part2;
mod day11part1; mod day11part2;
mod day12part1; mod day12part2;
fn main() {
    type SolutionPointer = fn(&str) -> String;
    let solutions: [[SolutionPointer; 2]; 12] = [
        [day1part1::solution, day1part2::solution], 
        [day2part1::solution, day2part2::solution], 
        [day3part1::solution, day3part2::solution], 
        [day4part1::solution, day4part2::solution], 
        [day5part1::solution, day5part2::solution], 
        [day6part1::solution, day6part2::solution], 
        [day7part1::solution, day7part2::solution], 
        [day8part1::solution, day8part2::solution], 
        [day9part1::solution, day9part2::solution], 
        [day10part1::solution, day10part2::solution], 
        [day11part1::solution, day11part2::solution], 
        [day12part1::solution, day12part2::solution]
    ];

    let args = &mut std::env::args().skip(1);
    let day: usize = args
        .next()
        .expect("No day provided")
        .parse()
        .expect("Invalid day");
    let part: usize = args
        .next()
        .expect("No part provided")
        .parse()
        .expect("Invalid part");
    assert!(!(!(1..=12).contains(&day) || !(1..=2).contains(&part)), "Wrong day/part!");
    let suffix: String = args.next().map_or(String::new(), |arg| arg);
    
    let input_file: String = format!("data/day{day}{suffix}.txt");
    let input: String = std::fs::read_to_string(input_file).unwrap();

    println!("{}", solutions[day - 1][part - 1](&input));
}