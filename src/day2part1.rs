//Slightly more efficient, but waay overcomplicated.
//Maybe this would be necessary if this task appeared in the second half of the AoC.

fn number_length(n: u64) -> u64{
    return n.to_string().len() as u64
}

fn tandemize(n:u64) -> u64{
    return format!("{n}{n}").parse().unwrap()
}

fn next_tandem_root(n: u64) -> u64{
    if number_length(n) % 2 != 0{
        return 10_u64.pow((n.ilog10() as u32 + 1) /2 )
    }
    let left = n.to_string()[0..number_length(n) as usize /2 ].parse().unwrap();
    let right:u64 = n.to_string()[number_length(n) as usize /2..].parse().unwrap(); 
    if left > right{
        return left
    }
    return left + 1
}

fn sum_of_tandems(min: u64, max: u64) -> u64{
    if min > max{
        return 0
    }else{
        return min + sum_of_tandems(next_tandem(min), max)
    }
}

fn next_tandem(tandem:u64)->u64{
    let tandem_root = 10_u64.pow((tandem.ilog10()+1)/2);
    if (tandem + 1).ilog10() > tandem.ilog10(){
        return tandem_root * (tandem_root*10 + 1)
    }
    if (tandem.ilog10() + 1) % 2 != 0{
        return tandem_root * (tandem_root*10 + 1)
    }
    return tandem + (tandem_root + 1)
    
}

pub fn solution(input: String) -> String { 
    format!("{:?}",
            input
                .split(',')
                .map(|line|
                    line.splitn(2, '-').collect::<Vec<&str>>())
                .map(|parts|
                    sum_of_tandems(tandemize(next_tandem_root(parts[0].parse::<u64>().unwrap()-1)), 
                                   parts[1].parse::<u64>().unwrap()))
                .sum::<u64>())
} 

#[test]
fn basic_test_1() {
    let input = "11-22".to_string();
    assert_eq!(solution(input), "33".to_string())
}
#[test]
fn basic_test_2() {
    let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124".chars().filter(|&c| c!=' ').collect::<String>();
    assert_eq!(solution(input), "1227775554".to_string())
}
#[test]
fn basic_test_3() {
    let input = "88-1212".chars().filter(|&c| c!=' ').collect::<String>();
    assert_eq!(solution(input), (88 + 99 + 1010 + 1111+ 1212).to_string())
}
#[test]
fn basic_test_4() {
    let input = "1010-1212".chars().filter(|&c| c!=' ').collect::<String>();
    assert_eq!(solution(input), (1010 + 1111+ 1212).to_string())
}
#[test]
fn unit_tests(){
    assert_eq!(number_length(12345), 5);
    assert_eq!(next_tandem_root(11- 1), 1);
    assert_eq!(next_tandem_root(123111), 123);
    assert_eq!(next_tandem_root(123555), 124);
    assert_eq!(next_tandem_root(999100), 999);
    assert_eq!(next_tandem_root(999999), 1000);
    assert_eq!(next_tandem_root(1000), 10);
    assert_eq!(next_tandem(99), 1010);
    assert_eq!(next_tandem(100), 1010);
    assert_eq!(next_tandem(88), 99);
    assert_eq!(next_tandem(999), 1010);
}
