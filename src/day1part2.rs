struct Dial{
    position: i32,
    times_0_visited: i32
} 
impl Dial{
    fn new() -> Self{
        Self{position:50, times_0_visited:0}
    }
    fn rotate(& mut self,  clicks: i32){
        self.times_0_visited += self.times_0_passed(clicks);
        self.position += clicks;
        self.position += (( self.position / 100).abs()) * 100 + 200;
        self.position %= 100;
    }
    fn times_0_passed(&self, clicks: i32) -> i32{
        let next = self.position + clicks;
        if next == 0{
            return 1
        }
        if next > 0{
            next / 100
        }
        else{
            i32::from(self.position != 0) + (next/100).abs()
        }
    }
}
pub fn solution(input: &str) -> String { 
    let mut dial = Dial::new();
    input.lines()
        .map(|line| 
            (match line.chars().next().unwrap(){
                'L' => -1,
                'R' => 1,
                _   => panic!("Wrong direction!")
            } * line.to_string()[1..].parse::<i32>().unwrap() ))
        .for_each(|clicks|
            dial.rotate( clicks) );
    format!("{:?}",dial.times_0_visited) 
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
    assert_eq!(solution(&input), "6".to_string())
}
