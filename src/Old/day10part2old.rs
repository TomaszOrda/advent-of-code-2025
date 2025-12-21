
use std::{array};
use itertools::Itertools;

struct Machine{
    // light_diagram: u16,
    buttons: Vec<Vec<usize>>,
    joltage_requirements: [u16; 10]
}
impl Machine{
    fn from_str(text:&str) -> Self{
        Self{
            // light_diagram : text.split(']').next().unwrap().trim_start_matches('[').chars().rev().map(|c| if c== '#' {1} else {0} ).fold(0,|acc, el| acc*2 + el ),
            buttons : text.split(']').nth(1).unwrap().split('{').next().unwrap().split_ascii_whitespace().map(Machine::button_from_str).sorted_by(|a,b| b.len().cmp(&a.len())).collect(),
            joltage_requirements : Machine::joltage_requirement_from_str(text.split('{').nth(1).unwrap().trim_end_matches('}')),
        }
    }
    fn button_from_str(text:&str) -> Vec<usize>{
        text.trim_matches(|c| c == '(' || c == ')' ).split(',').map(|c| c.parse::<usize>().unwrap()).collect()
    }
    fn joltage_requirement_from_str(text:&str) -> [u16;10]{
        text.split(',').map(|c| c.parse::<u16>().unwrap()).chain(std::iter::repeat(0)).take(10).collect::<Vec<u16>>().try_into().unwrap()
    }
    fn fewest_button_presses(&self) -> u16{
        let buttons_bitmap: Vec<[bool; 10]> = self.buttons.iter().map(|b| array::from_fn(|i| b.contains(&i))).collect();
        let buttons_max_lens_tail: Vec<usize> = self.buttons.iter().enumerate().map(|(id,_)| self.buttons[id..].iter().map(|b| b.len()).max().unwrap()).collect();
        
        let mut min_presses: u16 = self.joltage_requirements.iter().sum::<u16>()/self.buttons.iter().map(|b| b.len()).min().unwrap() as u16;
        
        let mut stack: Vec::<(u16, usize, [u16;10])> = vec![(0, 0, self.joltage_requirements)].into_iter().collect();
        'mainloop: while let Some((presses, button_id, remaining_joltage_requirements)) = stack.pop(){
            let sum = remaining_joltage_requirements.iter().sum::<u16>();
            if sum == 0{
                min_presses = min_presses.min(presses);
                // dbg!(min_presses);
                continue
            }
            for id in (0..remaining_joltage_requirements.len()).filter(|&id|remaining_joltage_requirements[id] != 0) {
                let mut b_s =(button_id..buttons_bitmap.len()).filter(|&b| buttons_bitmap[b][id]);
                let bttn = b_s.next();
                if bttn.is_none(){
                    continue 'mainloop
                }
                if b_s.count() == 0{
                    let times = remaining_joltage_requirements[id];
                    let button = &buttons_bitmap[bttn.unwrap()];
                    if !Machine::can_press(button, times, &remaining_joltage_requirements){
                        continue 'mainloop
                    }
                    stack.push((presses + times, button_id, Machine::after_press(button, times, &remaining_joltage_requirements)));
                    continue 'mainloop
                }
            }
            if presses + sum/buttons_max_lens_tail[button_id] as u16 >= min_presses{
                continue
            };
            let current_button_bitmap = &buttons_bitmap[button_id];
            
            let max_press = remaining_joltage_requirements.iter().zip(current_button_bitmap).filter(|(_, b)| **b).map(|p|p.0).min().unwrap();
            stack.extend((0..=*max_press).rev()
                                              .filter(|&times| presses + times < min_presses)
                                              .map(|times|( presses + times , button_id+1, Machine::after_press(current_button_bitmap, times, &remaining_joltage_requirements)) ) );
        }
        min_presses
    }


    #[inline(always)]
    fn can_press(button: &[bool; 10], times:u16, joltage_requirements: &[u16;10]) -> bool{
        joltage_requirements.iter().zip(button).all(|(&req, b)| !b || req >= times)
    }
    #[inline(always)]
    fn after_press(button: &[bool;10], times: u16, joltage_requirements: &[u16;10]) -> [u16;10]{
        array::from_fn(|id| joltage_requirements[id] - times * button[id] as u16)
    }
}


pub fn solution(input: &str) -> String { 
    input.lines()
         .map(|line| {/*dbg!(line);*/ Machine::from_str(line)})
         .map(|machine| machine.fewest_button_presses())//fewest_button_presses(&machine.buttons, &machine.joltage_requirements,0).unwrap())
         .map(|presses| presses as u64)
         .sum::<u64>()
         .to_string() 
} 
 
#[test] 
fn basic_test() { 
    let input:String = 
"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"
.chars().collect::<String>();
    assert_eq!(solution(&input), "33".to_string()) 
} 
 
#[test] 
fn unit_tests() { 
    let m1 = &Machine::from_str("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");
    let m2 = &Machine::from_str("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}}");
    let m3 = &Machine::from_str("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}");
    assert!(m1.fewest_button_presses() == 10);
    assert!(m2.fewest_button_presses() == 12);
    assert!(m3.fewest_button_presses() == 11);
} 