use crate::utils::Grid;

fn is_paper_roll(grid: &Grid<char>, pos: (i32, i32)) -> bool{
    grid.get(pos.0, pos.1) == Some(&'@')
}

fn can_be_accessed(grid: &Grid<char>, pos: (i32, i32)) -> bool{
   [-1, 0, 1].iter()
             .flat_map(|x_off| 
                       [-1, 0, 1].iter().map(|&y_off| (pos.0 + *x_off, pos.1 + y_off)))
             .filter(|&pos| is_paper_roll(grid, pos))
             .count() < 5
}


pub fn solution(input: &str) -> String {
    let grid:Grid<char> = Grid::new(input.lines()
                                             .map(|line|line.chars().collect::<Vec<char>>())
                                             .collect::<Vec<Vec<char>>>());
    format!("{:?}", grid.iter_flat()
                        .filter(|&pos| is_paper_roll(&grid, pos))
                        .filter(|&pos| can_be_accessed(&grid, pos))
                        .count())
}

#[test]
fn basic_test() {
    let input:String = "..@@.@@@@.
                        @@@.@.@.@@
                        @@@@@.@.@@
                        @.@@@@..@.
                        @@.@@@@.@@
                        .@@@@@@@.@
                        .@.@.@.@@@
                        @.@@@.@@@@
                        .@@@@@@@@.
                        @.@.@@@.@."
                        .chars().filter(|&c| c!=' ').collect::<String>();
    assert_eq!(solution(&input), "13".to_string())
}