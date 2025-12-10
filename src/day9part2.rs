use itertools::Itertools;

type Rectangle = ((i64, i64), (i64, i64));

fn rotate_90(dir: (i64, i64)) -> (i64, i64) {
    (-dir.1, dir.0)
}
fn rotate_270(dir: (i64, i64)) -> (i64, i64) {
    (dir.1, -dir.0)
}

fn get_rectangle_size(red_square_1: &(i64, i64), red_square_2: &(i64, i64)) -> i64{
    (red_square_1.0.abs_diff(red_square_2.0) as i64 + 1) * 
    (red_square_1.1.abs_diff(red_square_2.1) as i64 + 1) 
}

fn includes(x: i64, range: (i64, i64)) -> bool{
    range.0 <= x && x <= range.1
}
fn overlaps(range_1: (i64, i64), range_2: (i64, i64)) -> bool{
    includes(range_1.0, range_2)  || 
    includes(range_1.1, range_2)  || 
    includes(range_2.0, range_1)  
}

fn intersects(rectangle:Rectangle, line : ((i64, i64),(i64, i64))) -> bool{
    let bottom_left = (rectangle.0.0.min(rectangle.1.0), rectangle.0.1.min(rectangle.1.1));
    let top_right = (rectangle.0.0.max(rectangle.1.0), rectangle.0.1.max(rectangle.1.1));
    let line = ((line.0.0.min(line.1.0), line.0.1.min(line.1.1)),(line.0.0.max(line.1.0), line.0.1.max(line.1.1)));
    if line.0.0 == line.1.0{
        bottom_left.0 <= line.0.0 && 
        line.0.0 <= top_right.0 && 
        overlaps((line.0.1, line.1.1), (bottom_left.1, top_right.1))
    }else{
        bottom_left.1 <= line.0.1 && 
        line.0.1 <= top_right.1 && 
        overlaps((line.0.0, line.1.0), (bottom_left.0, top_right.0))
    }
}

fn is_valid_rectangle(rectangle: Rectangle, excluded_lines : &[Rectangle]) -> bool{
    !excluded_lines.iter()
                .any(|&line|
                     intersects(rectangle, line) )
}

fn get_excluded_line(red_square_1: (i64, i64), red_square_2: (i64, i64), rotation: fn((i64, i64)) -> (i64, i64) ) -> ((i64, i64),(i64, i64)){
    let unit_dir = (
        (red_square_2.0 - red_square_1.0).signum(),
        (red_square_2.1 - red_square_1.1).signum()
    );
    let unit_rotation =  rotation(unit_dir);
    (
        (red_square_1.0 + unit_dir.0 + unit_rotation.0, red_square_1.1 + unit_dir.1 + unit_rotation.1 ),
        (red_square_2.0 - unit_dir.0 + unit_rotation.0, red_square_2.1 - unit_dir.1 + unit_rotation.1 )
    )
}

fn get_rotation_right_times(window: ((i64, i64),(i64, i64),(i64, i64))) -> i64{
    let vec1 = (window.1.0 - window.0.0, window.1.1 - window.0.1);
    let vec2 = (window.2.0 - window.1.0, window.2.1 - window.1.1);
    let vector_mul = vec1.0 * vec2.1 - vec1.1 * vec2.0;
    vector_mul.signum()
}

pub fn solution(input: &str) -> String { 
    let red_squares: Vec<(i64, i64)> = input.lines()
                                            .map(|line| line.split(',')
                                                                  .map(|s| s.parse::<i64>().unwrap())
                                                                  .collect_tuple::<(i64, i64)>()
                                                                  .unwrap())
                                            .collect::<Vec<(i64, i64)>>();
    
    let red_squares_pairs: Vec<((i64, i64), (i64,i64))> = red_squares.clone()
                                                                     .into_iter()
                                                                     .cartesian_product(red_squares.clone())
                                                                     .filter(|(square_1, square_2)| square_1.0 < square_2.0 || square_1.0 == square_2.0 && square_1.1 < square_2.1 )
                                                                     .collect::<Vec<((i64, i64), (i64,i64))>>();
    let rotation_opposite_to_perimeter: fn((i64, i64)) -> (i64, i64) = 
        if red_squares.clone()
                      .into_iter()
                      .chain(red_squares.clone().into_iter().take(2))
                      .tuple_windows::<((i64, i64),(i64, i64),(i64, i64))>()
                      .map(get_rotation_right_times)
                      .sum::<i64>() > 0 
           {rotate_270} 
        else 
           {rotate_90};

    let excluded_lines: Vec<((i64, i64), (i64, i64))> = 
            red_squares.clone()
                       .iter()
                       .chain(red_squares.iter().take(3))
                       .tuple_windows::<(&(i64, i64),&(i64, i64))>()
                       .map(|(red_square_1, red_square_2)|
                            get_excluded_line(*red_square_1, *red_square_2, rotation_opposite_to_perimeter))
                       .collect::<Vec<((i64, i64),(i64, i64))>>();

    let max_rectangle_size: i64 = 
        red_squares_pairs.iter()
                         .map(|rectangle| (rectangle, get_rectangle_size(&rectangle.0, &rectangle.1)))
                         //.sorted_by(|a,b| b.1.cmp(&a.1))
                         .filter(|&(square, _)| is_valid_rectangle(*square, &excluded_lines))
                         .map(|(_, size)| size)
                         .max()
                         .unwrap();
    format!("{max_rectangle_size}")
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
    assert_eq!(solution(&input), "24".to_string()) 
} 