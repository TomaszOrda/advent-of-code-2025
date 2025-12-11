// use std::collections::HashMap;
use itertools::Itertools;

struct Machine{
    light_diagram: u64,
    buttons: Vec<u64>,
}
impl Machine{
    fn from_str(text:&str) -> Self{
        let buttons: Vec<u64> = text.split(']').nth(1).unwrap().split('{').next().unwrap().split_ascii_whitespace().map(Machine::button_from_str).collect();
        Self{
            light_diagram : text.split(']').next().unwrap().trim_start_matches('[').chars().rev().map(|c| if c== '#' {1} else {0} ).fold(0,|acc, el| acc*2 + el ),
            buttons : buttons.clone(),
        }
    }
    fn button_from_str(text:&str) -> u64{
        text.trim_matches(|c| c == '(' || c == ')' ).split(',').map(|c| 2_u64.pow(c.parse::<u32>().unwrap())).sum::<u64>()
    }
}

fn fewest_button_presses(machine: &Machine) -> usize{
    let initial_lights:u64 = 0;
    machine.buttons.iter()
                    //This might be slow, but performance is acceptable, optmization has been deemed unnecessary
                    .powerset()
                    .sorted_by(|a,b| a.len().cmp(&b.len()))
                    .find(|button_choice|
                            button_choice.iter()
                                            .fold(initial_lights, 
                                                |acc, &&button|
                                                acc^button ) == machine.light_diagram )
                    // .next()
                    .unwrap()
                    .len()
        
}

pub fn solution(input: &str) -> String { 
    input.lines()
         .map(Machine::from_str)
         .map(|machine| fewest_button_presses(&machine))
         .sum::<usize>()
         .to_string() 
} 
 
#[test] 
fn basic_test() { 
    let input:String = 
"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"
.chars().collect::<String>();
    assert_eq!(solution(&input), "7".to_string()) 
} 
 
#[test] 
fn unit_tests() { 
    assert!(fewest_button_presses(&Machine::from_str("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}")) == 2);
    assert!(fewest_button_presses(&Machine::from_str("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}}")) == 3);
    assert!(fewest_button_presses(&Machine::from_str("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}")) == 2);
} 