fn max_nth_battery_id(bank: &str, n:usize) -> usize{
    return (bank.len() - n )
        - bank[..=bank.len()-n]
            .chars()
            .zip((0..=bank.len()-n).rev())
            .max()
            .unwrap()
            .1;
}

fn max_joltage(bank: &str, batteries_to_turn: usize) -> String {
    if batteries_to_turn == 1{
        return bank.chars()
                   .max()
                   .unwrap()
                   .to_string()
                   .parse()
                   .unwrap();
    }
    let nth_battery_id = max_nth_battery_id(bank, batteries_to_turn);
    return format!("{}{}",
                    bank.chars().nth(nth_battery_id).unwrap(), 
                    max_joltage(&bank[nth_battery_id+1..], batteries_to_turn - 1));
}

pub fn solution(input: String) -> String {
    format!(
        "{:?}",
        input.lines().map(|line| max_joltage(line,12)).map(|joltage_str| joltage_str.parse::<u64>().unwrap()).sum::<u64>()
    )
}

#[test]
fn basic_test() {
    let input = "987654321111111
                         811111111111119
                         234234234234278
                         818181911112111"
        .to_string();
    assert_eq!(solution(input), "3121910778619".to_string())
}
#[test]
fn unit_tests() {
    assert_eq!(max_joltage("987654321111111",12), "987654321111");
}
