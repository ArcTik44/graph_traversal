use std::collections::{HashMap, VecDeque};

pub fn breadth_first_search(graph_map: HashMap<i32, Vec<i32>>, start_index: i32, mut visited_nodes: &mut Vec<i32>, mut to_visit: &mut VecDeque<i32>){
    visited_nodes.push(start_index);

    for vertex in &graph_map[&start_index]{
        if !visited_nodes.contains(vertex){
            to_visit.push_back(*vertex);
        }
    }

    while let Some(next) = to_visit.pop_front(){
        if !visited_nodes.contains(&next){
            breadth_first_search(graph_map.clone(), next, &mut visited_nodes,&mut to_visit)
        }
    }
}