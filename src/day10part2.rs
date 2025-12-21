
use std::{array, collections::{HashMap, HashSet}};
use itertools::Itertools;

//I could try making prettier code, but this already is a rework of a dfferent solution
//This seems to be more canonical for what i know. It is also ~20 times faster than my original approrach.
//I do not intend to optimize this further, I implemented it only as an exercise.
//I am not certain if it is *the* canonical algorithm though.

const MAX_BUTTONS: usize = 15;
const MAX_REQ: usize = 10;
type AugmentedMatrix = [[f64; MAX_BUTTONS]; MAX_REQ];
const EPS: f64 = 1e-7;

fn get_all_divisions(n:u16, length: usize, max_value: u16) -> Vec<Vec<f64>>{
    if length == 0{
        return vec![vec![]]
    }
    if length == 1{
        return vec![vec![n as f64]]
    }
    (0..=std::cmp::min(n, max_value))
            .flat_map(|x| 
                 get_all_divisions(n-x, length-1, max_value)
                    .iter()
                    .map(|valuation| 
                        (x..=x).map(|x| x as f64)
                                .chain(valuation.iter()
                                                .copied())
                                .collect::<Vec<f64>>())
                    .collect::<Vec<Vec<f64>>>())
                      .collect::<_>()
}

fn valid_presses(value:f64) -> bool{
    (value.fract().abs() < EPS || 1.0 - value.fract().abs() < EPS) && value > -EPS
}


struct EquationSystem{
    free_variables: HashSet<usize>,
    free_variables_bitmap: [bool; MAX_BUTTONS],
    determinations: HashMap<usize, (f64, HashMap<usize, f64>)>
}
impl EquationSystem{
    fn get(&self, variable: usize, free_variables_valuation: &[f64; MAX_BUTTONS]) -> f64{
            if self.free_variables_bitmap[variable]{
                free_variables_valuation[variable]
            }else{
                self.determinations[&variable].1
                                            .iter()
                                            .filter(|(_, value)|
                                                value.abs() > 1e-9)
                                            .map(|(var, value)|
                                                value * free_variables_valuation[*var] )
                                            .sum::<f64>() 
                                            + self.determinations[&variable].0
            }
    }

    fn new(matrix: AugmentedMatrix, number_of_variables: usize) -> Self{
        let mut determinations : HashMap<usize, (f64, HashMap<usize, f64>)> = HashMap::<_,_>::new();
        for line in matrix{
            if !line.iter().any(|x| x.abs() > EPS){
                continue;
            }
            let variable = line.iter().position(|x| x.abs() > EPS).unwrap();
            determinations.insert(variable,
                                  (line[MAX_BUTTONS-1],
                                  HashMap::<_,_>::from_iter(line.iter()
                                                                                          .take(MAX_BUTTONS-1)
                                                                                          .copied()
                                                                                          .enumerate()
                                                                                          .filter(|x| 
                                                                                                 variable != x.0)
                                                                                          .map(|(pos, coef)| (pos, -coef)))));
        }
        Self { 
               free_variables: HashSet::<_,_>::from_iter((0..number_of_variables).filter(|n| !determinations.keys().contains(n))),
               free_variables_bitmap: array::from_fn(|id| id < number_of_variables && !determinations.keys().contains(&id)),
               determinations }
    }
}

struct Machine{
    // light_diagram: u16,
    buttons: Vec<Vec<usize>>,
    joltage_requirements: [u16; 10]
}
impl Machine{
    fn from_str(text:&str) -> Self{
        Self{
            // light_diagram : text.split(']').next().unwrap().trim_start_matches('[').chars().rev().map(|c| if c== '#' {1} else {0} ).fold(0,|acc, el| acc*2 + el ),
            buttons : text.split(']').nth(1).unwrap().split('{').next().unwrap().split_ascii_whitespace().map(Machine::button_from_str).collect(),
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

        let rref = self.rref(buttons_bitmap, &self.joltage_requirements);
        let equation_system = EquationSystem::new(rref, self.buttons.len());

        let max_one_button_press = equation_system.free_variables.iter().map(|v| self.buttons[*v].iter().map(|b| self.joltage_requirements[*b]).max().unwrap()).max().unwrap_or(u16::MAX); 
        
        let presses_at_most: u16 = 
            (self.joltage_requirements.iter().sum::<u16>()/self.buttons.iter().map(|b| b.len()).min().unwrap() as u16)
            .min(equation_system.free_variables.len() as u16 * self.joltage_requirements.iter().max().unwrap())
            .min(self.joltage_requirements.iter().sum::<u16>() / equation_system.free_variables.iter().map(|v| self.buttons[*v].len()).min().unwrap_or(1) as u16)
            .min(equation_system.free_variables.len() as u16 * max_one_button_press);

        (0..=presses_at_most)
            .filter_map(|free_presses| 
                get_all_divisions(free_presses, equation_system.free_variables.len(), max_one_button_press)
                    .iter_mut()
                    .map(|values| values.iter_mut())
                    .map(|mut values_iter_mut| 
                         array::from_fn(|id| if equation_system.free_variables_bitmap[id] {*values_iter_mut.next().unwrap()} else {0.0}))
                    .filter(|valuation| (0..self.buttons.len()).all(|x| 
                            equation_system.free_variables_bitmap[x] ||
                            valid_presses(equation_system.get(x, valuation))))
                    .map(|valuation|  
                         (0..self.buttons.len())
                            .map(|x| 
                                 equation_system.get(x, &valuation) as u16 )
                            .sum::<u16>())
                    .min()
                )
            .min()
            .unwrap()

    }

    
    fn rref(&self, buttons_bitmap: Vec<[bool; 10]>, joltage_requirements: &[u16;10]) -> AugmentedMatrix{
        let mut matrix: AugmentedMatrix = [[0.0; MAX_BUTTONS]; MAX_REQ];

        for (i, row) in buttons_bitmap.iter().enumerate().take(MAX_BUTTONS-1) {
            for (j, &b) in row.iter().enumerate().take(MAX_REQ) {
                matrix[j][i] = b as u8 as f64;
            }
        }

        for (j, &b) in joltage_requirements.iter().enumerate().take(MAX_REQ) {
            matrix[j][MAX_BUTTONS-1] = b as f64;
        }

        // Gauss elimination https://en.wikipedia.org/wiki/Gaussian_elimination#:~:text=order%2D2%20tensors).-,Pseudocode,-%5Bedit%5D
        let (height, width) = (MAX_REQ, MAX_BUTTONS);
        let mut h = 0; 
        let mut k = 0; 
        while h < height && k < width {
            let i_max = (h..height)
                .max_by(|&i, &j| matrix[i][k].abs().partial_cmp(&matrix[j][k].abs()).unwrap())
                .unwrap();
            if matrix[i_max][k].abs() < EPS {
                k += 1;
            } else {
                matrix.swap(h, i_max);
                for i in (h + 1)..height {
                    let f = matrix[i][k] / matrix[h][k];
                    matrix[i][k] = 0.0;
                    #[allow(clippy::needless_range_loop)] for j in (k + 1)..width { 
                        matrix[i][j] -= matrix[h][j] * f;
                    }
                }
                h += 1;
                k += 1;
            }
        }
        for row in matrix.iter_mut().take(height) {
            let pivot_col = row.iter().position(|&x| x.abs() > EPS);
            if let Some(k) = pivot_col {
                let pivot_val = row[k];
                for val in row.iter_mut().take(width).skip(k) {
                    *val /= pivot_val;
                }
            }
        }
        for i in (0..height).rev() {
            let pivot_col = matrix[i].iter().position(|&x| x.abs() > EPS);
            if let Some(k) = pivot_col {
                let pivot_val = matrix[i][k];
                for j in 0..i {
                    let f = matrix[j][k] / pivot_val;
                    matrix[j][k] = 0.0;
                    #[allow(clippy::needless_range_loop)] for l in (k + 1)..width {
                        matrix[j][l] -= matrix[i][l] * f;
                    }
                }
            }
        }
        matrix
    }
}


pub fn solution(input: &str) -> String { 
    input.lines()
         .map(|line| {dbg!(line); Machine::from_str(line)})
         .map(|machine| machine.fewest_button_presses())
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