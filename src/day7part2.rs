use std::collections::HashMap;

use crate::utils::Grid;

fn get_split_beams(grid: &Grid<char>, beams_x:&HashMap<i32, u64>, beams_y:i32) -> HashMap<i32, u64> {
    let mut result = HashMap::<i32, u64>::new();
    beams_x.iter()
                  .flat_map(|(&x, &n)| 
                    //I could do this prettier
                    match grid.get(x, beams_y){
                        Some('^') => vec![(x-1,n), (x+1,n)],
                        _         => vec![(x,n)]
                    })
                  .for_each(|(x,n)|{
                            result.entry(x)
                                  .and_modify(|old_n| *old_n += n)
                                  .or_insert(n);
                            });
    result
}

fn calculate_beam_timelines(grid: &Grid<char>, beams_x:&HashMap<i32, u64>, beams_y:i32) -> u64 {
    if beams_y == grid.height{
        return beams_x.iter().map(|tupple| tupple.1).sum()
    }
    calculate_beam_timelines(grid, 
                             &get_split_beams(grid, beams_x, beams_y), 
                             beams_y+1)
}

pub fn solution(input: &str) -> String { 
    let grid:Grid<char> = Grid::new(input.lines()
                                             .map(|line|line.chars().collect::<Vec<char>>())
                                             .collect::<Vec<Vec<char>>>());
    //We can assume that all the starting positions are one the line 0
    let initial_beams: HashMap<i32, u64> = (0..grid.width).filter(|&x|
                                                                  grid.get(x,0)==Some(&'S'))
                                                          .map(|x| (x,1))
                                                          .collect();
    

    format!("{:?}",calculate_beam_timelines(&grid, &initial_beams, 0)) 
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
    assert_eq!(solution(&input), "40".to_string()) 
} 