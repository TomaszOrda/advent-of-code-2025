use std::collections::{HashMap};

struct ServerRack{
    connections: HashMap<String, Vec<String>>,
    cache: HashMap<(String, String), u64>,
}
impl ServerRack {
    fn new_from_str(text:&str) -> Self{
        let mut res = Self { connections: HashMap::<String, Vec<String>>::new(), cache: HashMap::<_,_>::new() };
        text.lines()
            .map(|line|
                 line.split(':').collect::<Vec<&str>>())
            .for_each(|line_vec|
                 {res.connections.insert(line_vec[0].to_string(),line_vec[1].split_ascii_whitespace().map(|s| s.to_string()).collect());});
        res
    }
    fn get_paths_to_out(&mut self, start: &String, end:&String) -> u64{
        if start == end{
            return 1
        };
        if !self.connections.contains_key(start){
            return 0
        }
        let cache_key = (start.clone(), end.clone());
        if self.cache.contains_key(&cache_key){
            return self.cache[&cache_key];
        }
        let neighbours = self.connections[start].clone();
        let mut result = 0;
        for neighbour in neighbours{
            result += self.get_paths_to_out(&neighbour, end);
        }
        self.cache.insert(cache_key, result);
        result
    }
}

pub fn solution(input: &str) -> String { 
    let mut server_rack = ServerRack::new_from_str(input);

    format!("{}",server_rack.get_paths_to_out(&"svr".to_string(), &"fft".to_string()) * 
                 server_rack.get_paths_to_out(&"fft".to_string(), &"dac".to_string()) *
                 server_rack.get_paths_to_out(&"dac".to_string(), &"out".to_string())) 
} 
 
#[test] 
fn basic_test() { 
    let input:String = 
"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"
.chars().collect::<String>();
    assert_eq!(solution(&input), "2".to_string()) 
} 
// #[test] 
// fn unit_tests() { 
//     let input:String = 
// "aaa: you hhh
// you: bbb ccc
// bbb: ddd eee
// ccc: ddd eee fff
// ddd: ggg
// eee: out
// fff: out
// ggg: out
// hhh: ccc fff iii
// iii: out"
// .chars().collect::<String>();
//     let server_rack = ServerRack::new_from_str(input);

//     assert_eq!(get_paths_to_out(&"svr".to_string(), &"fft".to_string(), &server_rack), 4);
//     assert_eq!(get_paths_to_out(&"svr".to_string(), &"fft".to_string(), &server_rack), 4);
//     assert_eq!(solution(&input), "2".to_string());
// } 