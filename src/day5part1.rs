//The solution is far from optimal, however there is basically no need for optimization, it runs fast enough.
//We could however sort both ranges and go through each list basically once.

struct IngredientRange{
    low: u64,
    high: u64
}
impl IngredientRange{
    fn includes(&self, ingredient: u64) -> bool{
        self.low <= ingredient && ingredient <= self.high
    }
}

pub fn solution(input: &str) -> String {
    let ingredient_ranges: Vec<IngredientRange> = input.lines()
                                                       .take_while(|line| !line.is_empty())
                                                       .map(|line|
                                                            line.splitn(2, '-').collect::<Vec<&str>>())
                                                       .map(|parts| 
                                                            IngredientRange{
                                                                low: parts[0].parse().unwrap(), 
                                                                high: parts[1].parse().unwrap()
                                                            })
                                                       .collect();
    
    let ingredients: Vec<u64> = input.lines()
                                     .skip_while(|line| 
                                                 !line.is_empty())
                                     .skip(1)
                                     .map(|line|
                                          line.parse::<u64>().unwrap()).collect();
    format!("{:?}",ingredients.iter()
                              .filter(|&&ingredient| 
                                      ingredient_ranges.iter()
                                                       .any(|ingredient_range| 
                                                            ingredient_range.includes(ingredient)))
                              .count())
}

#[test]
fn basic_test() {
    let input:String = "3-5
                        10-14
                        16-20
                        12-18

                        1
                        5
                        8
                        11
                        17
                        32"
                        .chars().filter(|&c| c!=' ').collect::<String>();
    assert_eq!(solution(&input), "3".to_string())
}