use std::collections::HashSet;


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

fn get_edges_sorted(edges: &[(JunctionBox, JunctionBox, i64)], n:usize) -> Vec<(JunctionBox, JunctionBox, i64)>{
    let mut edges:Vec<(JunctionBox, JunctionBox, i64)> = edges.to_vec();
    edges.sort_by(|e1, e2| if e1.2 < e2.2 || e1.2==e2.2 && e1.0 < e2.0 {std::cmp::Ordering::Less} else {std::cmp::Ordering::Greater});
    edges.iter().take(n).copied().collect()
}

fn get_circuit_sizes(edges: &[(JunctionBox, JunctionBox, i64)]) -> Vec<i64>{
    let dir_edges: HashSet<(JunctionBox, JunctionBox)> = edges.iter()
                                                            .flat_map(|&(junction_box_from, junction_box_to, _)|
                                                                 [(junction_box_from, junction_box_to),
                                                                  (junction_box_to, junction_box_from)]
                                                                 )
                                                            .collect();
    
    let mut visited: HashSet<JunctionBox> = HashSet::new();
    let mut sizes = vec![];
    for junction_box_start in 0..edges.len(){
        let mut to_visit = [junction_box_start].to_vec();
        let mut size = 0;
        while let Some(junction_box) = to_visit.pop(){
            if visited.contains(&junction_box){
                continue;
            }
            size += 1;
            visited.insert(junction_box);
            dir_edges.iter()
                     .filter(|edge| edge.0 == junction_box && !visited.contains(&edge.1))
                     .for_each(|edge| to_visit.push(edge.1));
        }
        sizes.push(size);
    }
    sizes.sort_by(|a,b|b.cmp(a));
    sizes
}

pub fn solution(input: &str) -> String { 
    let edges_to_take = if input.lines().count() < 50 {10} else {1000};
    dbg!(edges_to_take);
    format!("{:?}",
        get_circuit_sizes(
            &get_edges_sorted(
                &get_edges(&input.lines()
                                 .map(|line| line.splitn(3, ','))
                                 .map(|coords_str| coords_str.map(|coord| coord.parse::<i64>().unwrap()).collect::<Vec<i64>>())
                                 .map(|coords_iter| (coords_iter[0], coords_iter[1], coords_iter[2]))
                                 .collect::<Vec<(i64, i64, i64)>>()), edges_to_take))
            .iter()
            .take(3)
            .product::<i64>())
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
    assert_eq!(solution(&input), "40".to_string()) 
} 