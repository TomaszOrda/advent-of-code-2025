fn solve_math_problem(problem: &Vec<&str>) -> u64{
    let mul = |x:u64, y:u64| x*y;
    let add = |x:u64, y:u64| x+y;
    let operation = match *problem.last().unwrap(){
        "*" => mul,
        "+" => add,
        _   => panic!("Unknown operation {}!", problem.last().unwrap())
    };
    let init = match *problem.last().unwrap(){
        "*" => 1,
        "+" => 0,
        _   => panic!("Unknown operation {}!", problem.last().unwrap())
    };
    problem.iter()
           .rev()
           .skip(1)
           .map(|val_str: &&str|
                val_str.parse::<u64>()
                       .unwrap())
           .fold(init, operation)
}


pub fn solution(input: &str) -> String { 
    let mut math_problems: Vec<Vec<&str>> = vec![vec![]; input.lines().next().unwrap().split_whitespace().count()];
    input.lines()
         .for_each(|line|
                   line.split_whitespace()
                       .enumerate()
                       .for_each(|(column_id, value)|
                                 math_problems[column_id].push(value)));
    format!("{:?}",math_problems.iter()
                                .map(|problem| 
                                     solve_math_problem(problem))
                                .sum::<u64>()) 
} 
 
#[test] 
fn basic_test() { 
    let input:String = "123 328  51 64 
45 64  387 23 
6 98  215 314
*   +   *   +  "
                         .chars().collect::<String>();
    assert_eq!(solution(&input), "4277556".to_string()) 
} 