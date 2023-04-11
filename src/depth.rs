use std::collections::HashMap;

pub fn create_graph_map<const N: usize, const M: usize>(graph: [[i32; M]; N])->HashMap<i32,Vec<i32>>{

    /*Build graph map */
    let mut graph_map:HashMap<i32,Vec<i32>> = HashMap::new();
    for x in 0..N{
        let mut vertices:Vec<i32> = Vec::new();
        for y in 0..M{
            if graph[x][y] == 1{
                vertices.push(y as i32);
            }
        }
        graph_map.insert(x as i32, vertices);
    }
    return graph_map;
}


pub fn depth_first_search(graph_map: HashMap<i32, Vec<i32>>, start_index: i32, mut visited_nodes: &mut Vec<i32>, mut to_visit: &mut Vec<i32>) {
    visited_nodes.push(start_index);

    for vertex in &graph_map[&start_index] {
        if !visited_nodes.contains(vertex) {
            to_visit.push(*vertex);
        }
    }

    while let Some(next) = to_visit.pop() {
        if !visited_nodes.contains(&next) {
            depth_first_search(graph_map.clone(), next, &mut visited_nodes, &mut to_visit);
        }
    }
}