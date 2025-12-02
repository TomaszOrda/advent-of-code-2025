enum Direction {
    L,
    R,   
}
struct Dial{
    position: i16,
} 
impl Dial{
    fn new() -> Self{
        return Self{position:50}
    }
    fn rotate(& mut self, direction: &Direction, clicks: &u16){
        match direction{
            Direction::L => self.position += 100 - *clicks as i16,
            Direction::R => self.position += *clicks as i16,
        };
        self.position %= 100;
    }
    fn get_position(&self)->i16{
        return self.position
    }
}
pub fn solution(input: String) -> String { 
    let mut dial = Dial::new();
    let result: usize = input.lines()
                            .map(|line| 
                                (match line.chars().nth(0).unwrap(){
                                    'L' => Direction::L,
                                    'R' => Direction::R,
                                    _   => panic!("Wrong direction!")
                                }, line.to_string()[1..].parse::<u16>().unwrap() ))
                            .filter(|(direction, clicks)| {
                                dial.rotate(direction, clicks);
                                dial.get_position() == 0 } )
                            .count();
    format!("{:?}",result) 
} 

#[test]
fn basic_test() {
    let input = "L68
                         L30
                         R48
                         L5
                         R60
                         L55
                         L1
                         L99
                         R14
                         L82".chars().filter(|&c| c!=' ').collect::<String>();
    assert_eq!(solution(input), "3".to_string())
}
