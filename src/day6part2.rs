fn operation_str_to_closure(operation: &str) -> fn(u64, &u64) -> u64{
    let mul = |x:u64, y:&u64| x*y;
    let add = |x:u64, y:&u64| x+y;
     match operation {
        "*" => mul,
        "+" => add,
        _   => panic!("Unknown operation {operation}!")
    }
}
fn operation_str_neutral_value(operation: &str) -> u64{
     match operation {
        "*" => 1,
        "+" => 0,
        _   => panic!("Unknown operation {operation}!")
    }
}

fn solve_math_problem(problem: &[u64], operation: &str) -> u64{
    let closure = operation_str_to_closure(operation);
    problem.iter().fold(operation_str_neutral_value(operation), closure)
}


pub fn solution(input: &str) -> String { 
    let operations: Vec<&str> = input.lines().next_back().unwrap().split_whitespace().collect::<Vec<&str>>();

    let mut input_rot_90: Vec<String> = vec![String::new(); input.lines().next().unwrap().len()];
    input.lines()
         .take_while(|line: &&str| line.chars().any(|c: char| c.is_ascii_digit()))
         .for_each(|line: &str|
                   line.chars()
                       .enumerate()
                       .for_each(|(column_id, value)|
                                 input_rot_90[column_id].push(value)));

    let math_problems: Vec<Vec<u64>> = input_rot_90.chunk_by(|str1, str2| str1.chars().any(|c| c.is_ascii_digit()) && str2.chars().any(|c| c.is_ascii_digit()))
                                                    .map(|str:&[String]| 
                                                         str.iter()
                                                            .filter(|str: &&String| str.chars().any(|c| c.is_ascii_digit()))
                                                            .map(|line: &String|
                                                                 line.trim().parse::<u64>().unwrap())
                                                            .collect::<Vec<u64>>()
                                                    )
                                                    .filter(|problem: &Vec<u64>| !problem.is_empty())
                                                    .collect::<Vec<Vec<u64>>>();

    format!("{:?}",math_problems.iter()
                                .zip(operations.iter())
                                .map(|(problem, operation)| 
                                     solve_math_problem(problem, operation))
                                .sum::<u64>())
} 
 
#[test] 
fn basic_test() { 
    let input:String = 
"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "
                         .chars().collect::<String>();
    assert_eq!(solution(&input), "3263827".to_string()) 
} 