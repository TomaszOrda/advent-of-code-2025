use std::collections::{HashMap};

struct ServerRack{
    connections: HashMap<String, Vec<String>>
}
impl ServerRack {
    fn new_from_str(text:&str) -> Self{
        let mut res = Self { connections: HashMap::<String, Vec<String>>::new() };
        text.lines()
            .map(|line|
                 line.split(':').collect::<Vec<&str>>())
            .for_each(|line_vec|
                 {res.connections.insert(line_vec[0].to_string(),line_vec[1].split_ascii_whitespace().map(|s| s.to_string()).collect());});
        res
    }
}

fn get_paths_to_out(start: String, server_rack: &ServerRack) -> u32{
    if start == "out"{
        return 1
    };
    server_rack.connections[&start].iter()
                                   .map(|node| 
                                        get_paths_to_out(node.clone(), server_rack))
                                   .sum::<u32>()
}
pub fn solution(input: &str) -> String { 
    let server_rack = ServerRack::new_from_str(input);

    format!("{}",get_paths_to_out("you".to_string(), &server_rack)) 
} 
 
#[test] 
fn basic_test() { 
    let input:String = 
"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out"
.chars().collect::<String>();
    assert_eq!(solution(&input), "5".to_string()) 
} 