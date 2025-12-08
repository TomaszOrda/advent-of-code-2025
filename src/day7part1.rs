use std::collections::HashSet;

use crate::utils::Grid;

fn get_beams_splits(grid: &Grid<char>, beams_x:&HashSet<i32>, beams_y:i32) -> u32 {
    beams_x.iter()
           .filter(|&&x| grid.get(x, beams_y) == Some(&'^'))
           .count() as u32
}
fn get_split_beams(grid: &Grid<char>, beams_x:&HashSet<i32>, beams_y:i32) -> HashSet<i32> {
    beams_x.iter()
           .flat_map(|&x| 
                     //I could do this prettier
                     match grid.get(x, beams_y){
                        Some('^') => [x-1, x+1],
                        _         => [x, x]
                     })
           .collect()
}

fn calculate_beam_splits(grid: &Grid<char>, beams_x:&HashSet<i32>, beams_y:i32) -> u32 {
    if beams_y == grid.height{
        return 0
    }
    get_beams_splits(grid, 
                     beams_x, 
                     beams_y) + 
    calculate_beam_splits(grid, 
                          &get_split_beams(grid, beams_x, beams_y), 
                          beams_y+1)
}

pub fn solution(input: &str) -> String { 
    let grid:Grid<char> = Grid::new(input.lines()
                                             .map(|line|line.chars().collect::<Vec<char>>())
                                             .collect::<Vec<Vec<char>>>());
    //We can assume that all the starting positions are one the line 0
    let initial_beams: HashSet<i32> = (0..grid.width).filter(|&x|grid.get(x,0)==Some(&'S')).collect();
    

    format!("{:?}",calculate_beam_splits(&grid, &initial_beams, 0)) 
} 
 
#[test] 
fn basic_test() { 
    let input:String = 
".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."
.chars().collect::<String>();
    assert_eq!(solution(&input), "21".to_string()) 
} 