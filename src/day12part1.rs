
use itertools::Itertools;

use crate::utils::Grid;

struct Present{
    shape: Grid<char>
}
impl Present{
    fn from_str(text:Vec<Vec<char>>) -> Self{
        Self { 
            shape: Grid::new(text)
        }
    }
    fn spaces_occupied(&self) -> usize{
        self.shape.iter_flat().filter(|&(x,y)| self.shape.get(x, y).unwrap() == &'#' ).count()
    }
}

struct Region{
    size_x: u32,
    size_y: u32,
    presents_to_place: Vec<usize>
}
impl Region{
    fn from_str(text:&str) -> Self{
        let sizes_str = text.split(':').next().unwrap();
        let presents_str = text.split(':').nth(1).unwrap();
        Self { 
            size_x: sizes_str.split('x').next().unwrap().parse::<u32>().unwrap(), 
            size_y: sizes_str.split('x').nth(1).unwrap().parse::<u32>().unwrap(), 
            presents_to_place: presents_str.split_ascii_whitespace().map(|str| str.parse::<usize>().unwrap()).collect()
        }
    }
    fn will_likely_fit(&self, presents: &[Present]) -> bool{
        let number_of_presents = self.presents_to_place.iter().sum::<usize>() as i32;
        let fluid_presents_volume: i32 = self.presents_to_place.iter().enumerate().map(|(id, count)| count * presents[id].spaces_occupied()).sum::<usize>() as i32;
        
        let avaiable_3by3_spaces: i32 = ((self.size_x/3) * (self.size_y/3)) as i32;
        let region_volume: i32 = (self.size_x * self.size_y) as i32;

        //Experiment derived by using test example.
        //In case of big grids a constant of 1 is sufficient
        //In case of small grids fluid method overflow should have 4 times as much weight compared to the box method
        let fluid_method_weight = 4;

        let box_method_overflow = (number_of_presents - avaiable_3by3_spaces) * 9;
        let fluid_method_underflow = region_volume - fluid_presents_volume;
        fluid_method_underflow * fluid_method_weight > box_method_overflow
    }
}
pub fn solution(input: &str) -> String { 
    let regions = input.lines()
                                   .skip_while(|line| !line.contains('x'))
                                   .map(Region::from_str)
                                   .collect::<Vec<Region>>();
    let presents = input
                    .lines()
                    .take_while(|line| !line.contains('x'))
                    .filter(|line| !line.is_empty() && !line.contains(':'))
                    .chunks(3)
                    .into_iter()
                    .map(|chunk| 
                        Present::from_str(chunk.into_iter().map(|c| c.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>()))
                    .collect::<Vec<Present>>();
    regions.iter()
           .filter(|region| region.will_likely_fit(&presents))
           .count()
           .to_string()
} 

#[test] 
fn basic_test() { 
    let input:String = 
"0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2"
.chars().collect::<String>();
    assert_eq!(solution(&input), "2".to_string()) 
} 
#[test]
fn basic_test_big() { 
    let input:String = 
"0:
###
#.#
#.#

1:
###
.#.
###

2:
###
##.
#..

3:
##.
.##
..#

4:
#.#
###
##.

5:
..#
###
###

39x41: 27 38 27 26 26 24
37x47: 38 39 47 48 31 66
37x42: 28 30 36 16 32 26"
.chars().collect::<String>();
    assert_eq!(solution(&input), "2".to_string()) 
} 