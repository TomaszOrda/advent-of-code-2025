use itertools::Itertools;

fn get_rectangle_size(red_square_1: &(u64, u64), red_square_2: &(u64, u64)) -> u64{
    (red_square_1.0.abs_diff(red_square_2.0) + 1) * 
    (red_square_1.1.abs_diff(red_square_2.1) + 1) 
}

pub fn solution(input: &str) -> String { 
    let red_squares: Vec<(u64, u64)> = input.lines()
                                            .map(|line| line.split(',')
                                                                  .map(|s| s.parse::<u64>().unwrap())
                                                                  .collect_tuple::<(u64, u64)>()
                                                                  .unwrap())
                                            .collect::<Vec<(u64, u64)>>();
    
    let red_squares_pairs: Vec<((u64, u64), (u64,u64))> = red_squares.clone()
                                                                     .into_iter()
                                                                     .cartesian_product(red_squares.clone())
                                                                     .filter(|(square_1, square_2)| square_1.0 < square_2.0 || square_1.0 == square_2.0 && square_1.1 < square_2.1 )
                                                                     .collect::<Vec<((u64, u64), (u64,u64))>>();
                
    format!("{}",red_squares_pairs.iter()
                                  .map(|pair| get_rectangle_size(&pair.0, &pair.1))
                                  .max()
                                  .unwrap()) 
} 
 
#[test] 
fn basic_test() { 
    let input:String = 
"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"
.chars().collect::<String>();
    assert_eq!(solution(&input), "50".to_string()) 
} 