#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct IngredientRange{
    low: u64,
    high: u64
}
impl IngredientRange{
    fn includes(&self, ingredient: u64) -> bool{
        self.low <= ingredient && ingredient <= self.high
    }
    fn overlaps(&self, other: &IngredientRange) -> bool{
        //I could use one include
        self.includes(other.low)  || 
        self.includes(other.high) || 
        other.includes(self.low)  || 
        other.includes(self.high)
    }
    fn join(&mut self, other: &IngredientRange){
        self.low = std::cmp::min(self.low, other.low);
        self.high = std::cmp::max(self.high, other.high);
    }
    fn len(&self) -> u64{
        self.high - self.low + 1
    }
}

pub fn solution(input: &str) -> String {
    let mut ingredient_ranges: Vec<IngredientRange> = input.lines()
                                                       .take_while(|line| !line.is_empty())
                                                       .map(|line|
                                                            line.splitn(2, '-').collect::<Vec<&str>>())
                                                       .map(|parts| 
                                                            IngredientRange{
                                                                low: parts[0].parse().unwrap(), 
                                                                high: parts[1].parse().unwrap()
                                                            })
                                                       .collect();

    ingredient_ranges.sort();
    let mut ingredient_ranges_joined: Vec<IngredientRange> = ingredient_ranges[..1].to_vec();
    for &ingredient_range in &ingredient_ranges{
        let last = ingredient_ranges_joined.len() - 1;
        if ingredient_ranges_joined[last].overlaps(&ingredient_range){
            ingredient_ranges_joined[last].join(&ingredient_range);
        }else{
            ingredient_ranges_joined.push(ingredient_range);
        } 
                        
    }
    format!("{:?}",ingredient_ranges_joined.iter()
                                           .map(IngredientRange::len)
                                           .sum::<u64>())
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
    assert_eq!(solution(&input), "14".to_string())
}