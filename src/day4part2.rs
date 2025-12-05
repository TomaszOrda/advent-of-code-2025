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

fn try_remove_roll_off_paper(grid: &mut Grid<char>, pos: (i32, i32)) -> Option::<()>{
    if !is_paper_roll(grid, pos) || !can_be_accessed(grid, pos){
        return None
    }
    *grid.get_mut(pos.0, pos.1).unwrap() = 'x';
    Some(())
}

pub fn solution(input: String) -> String {
    let mut grid:Grid<char> = Grid::new(input.lines()
                                             .map(|line|line.chars().collect::<Vec<char>>())
                                             .collect::<Vec<Vec<char>>>());
    let grid_ids = grid.iter_flat().collect::<Vec<(i32,i32)>>();
    while grid_ids.iter()
                  .map(|&pos| try_remove_roll_off_paper(&mut grid, pos))
                  .filter(|res| res.is_some())
                  .count() > 0 {};
            
    format!("{:?}", grid.iter_flat()
                        .filter(|&pos| grid.get(pos.0,pos.1) == Some(&'x'))
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
    assert_eq!(solution(input), "43".to_string())
}