
fn is_k_tandem(n:u64, k:u32)->bool{
    let length:u32 = n.ilog10() +1;
    return length % k == 0 && 
           (1..length/k).all( |i:u32| 
                              {
                                  let root =  10_u64.pow(k);
                                  return (n % root ) == (n/root.pow(i)) % root 
                              } )
}

fn is_tandem(n:u64)->bool{
    let length:u32 = n.ilog10() +1;
    return (1..=length/2).any(
        |k:u32|
        return is_k_tandem(n, k) )
}

fn sum_of_tandems(min: u64, max: u64) -> u64{
    return (min..=max).filter(|n| is_tandem(*n)).sum();
}


pub fn solution(input: String) -> String { 
    format!("{:?}",
            input
                .split(',')
                .map(|line|
                    line.splitn(2, '-').collect::<Vec<&str>>())
                .map(|parts|
                    sum_of_tandems(parts[0].parse::<u64>().unwrap(), 
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
    assert_eq!(solution(input), "4174379265".to_string())
}
#[test]
fn basic_test_3() {
    let input = "88-1212".chars().filter(|&c| c!=' ').collect::<String>();
    assert_eq!(solution(input), (88 + 99 + 111 + 222+333+444+555+666+777+888+999+ 1010 + 1111+ 1212).to_string())
}
#[test]
fn basic_test_4() {
    let input = "1010-1212".chars().filter(|&c| c!=' ').collect::<String>();
    assert_eq!(solution(input), (1010 + 1111+ 1212).to_string())
}
#[test]
fn unit_tests(){
    assert_eq!(is_tandem(11), true);
    assert_eq!(is_tandem(111), true);
    assert_eq!(is_tandem(1010), true);
    assert_eq!(is_tandem(123123), true);
    assert_eq!(is_tandem(123123123), true);
    assert_eq!(is_tandem(12121212), true);
    assert_eq!(is_tandem(1231123), false);
}
