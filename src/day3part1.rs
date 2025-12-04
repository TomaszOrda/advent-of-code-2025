fn max_left_battery_id(bank: &str) -> usize {
    return (bank.len() - 2)
        - bank[..bank.len()]
            .chars()
            .zip((0..bank.len() - 1).rev())
            .max()
            .unwrap()
            .1;
}
fn max_right_battery(bank: &str, start_id: usize) -> u32 {
    return bank[start_id..bank.len()]
        .chars()
        .max()
        .unwrap()
        .to_string()
        .parse()
        .unwrap();
}

fn max_joltage(bank: &str) -> u32 {
    let left_battery_id = max_left_battery_id(bank);
    return bank
        .chars()
        .nth(left_battery_id)
        .unwrap()
        .to_string()
        .parse::<u32>()
        .unwrap()
        * 10
        + max_right_battery(bank, left_battery_id + 1);
}

pub fn solution(input: String) -> String {
    format!(
        "{:?}",
        input.lines().map(|line| max_joltage(line)).sum::<u32>()
    )
}

#[test]
fn basic_test() {
    let input = "987654321111111
                         811111111111119
                         234234234234278
                         818181911112111"
        .to_string();
    assert_eq!(solution(input), "357".to_string())
}
#[test]
fn unit_tests() {
    assert_eq!(max_left_battery_id("818181911112111"), 6);
    assert_eq!(max_left_battery_id("818181911119111"), 6);
    assert_eq!(max_right_battery("818181911112111", 7), 2);
    assert_eq!(max_right_battery("818181919119111", 7), 9);
    assert_eq!(max_joltage("987654321111111"), 98);
}
