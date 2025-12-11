use std::collections::HashMap;


type JunctionBox = usize;

fn distance_squared(pos1: &(i64, i64, i64), pos2: &(i64, i64, i64)) -> i64{
    (pos1.0 - pos2.0).pow(2) + (pos1.1 - pos2.1).pow(2) + (pos1.2 - pos2.2).pow(2) 
}

fn get_edges(positions: &[(i64, i64, i64)]) -> Vec<(JunctionBox, JunctionBox, i64)>{
    positions.iter()
             .enumerate()
             .flat_map(|junction_1| 
                       positions.iter()
                                .enumerate()
                                .map(move |junction_2|
                                     (junction_1.0, junction_2.0, distance_squared(junction_1.1, junction_2.1))))
             .filter(|edge|
                     edge.0 < edge.1)
             .collect()
}

fn get_edges_sorted(edges: &[(JunctionBox, JunctionBox, i64)]) -> Vec<(JunctionBox, JunctionBox, i64)>{
    let mut edges:Vec<(JunctionBox, JunctionBox, i64)> = edges.to_vec();
    edges.sort_by(|e1, e2| if e1.2 < e2.2 || e1.2==e2.2 && e1.0 < e2.0 {std::cmp::Ordering::Less} else {std::cmp::Ordering::Greater});
    edges
}

fn get_last_edge_after_full_connection(edges: &[(JunctionBox, JunctionBox, i64)], connections: &HashMap<JunctionBox, JunctionBox>) -> (JunctionBox, JunctionBox, i64){
    let mut connections = connections.clone();
    let mut number_of_connecting_edges = 0;
    for edge in edges{
        let indicator_0 = connections[&edge.0];
        let indicator_1 = connections[&edge.1];
        if indicator_0 != indicator_1{
            number_of_connecting_edges += 1;
            for value in &mut connections.values_mut(){
                if value == &indicator_1{
                    *value = indicator_0;
                }
            }
            if number_of_connecting_edges == connections.len() - 1{
                return *edge
            }
        }
    }
    panic!("Not enough edges!")
}

pub fn solution(input: &str) -> String {
    let junctions_number = input.lines().count();
    let junction_positions: Vec<(i64, i64, i64)> = 
                input.lines().map(|line| line.splitn(3, ','))
                     .map(|coords_str| coords_str.map(|coord| coord.parse::<i64>().unwrap()).collect::<Vec<i64>>())
                     .map(|coords_iter| (coords_iter[0], coords_iter[1], coords_iter[2]))
                     .collect::<Vec<(i64, i64, i64)>>();
    let edges_sorted = get_edges_sorted(&get_edges(&junction_positions));
    let initial_connections = (0..junctions_number).map(|j| (j, j)).collect::<HashMap<JunctionBox, JunctionBox>>();
    let last_edge = get_last_edge_after_full_connection(&edges_sorted, &initial_connections);
    format!("{:?}", junction_positions[last_edge.0].0 * junction_positions[last_edge.1].0)
} 
 
#[test] 
fn basic_test() { 
    let input:String = 
"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"
.chars().collect::<String>();
    assert_eq!(solution(&input), "25272".to_string()) 
} 