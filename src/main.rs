mod depth;
mod breadth;
use std::collections::VecDeque;

use crate::breadth::breadth_first_search;
use crate::depth::{create_graph_map,depth_first_search};
fn main() {
    let mut visited_depth:Vec<i32> = Vec::new();
    let mut visited_breadth:Vec<i32> = Vec::new();
   
    let mut to_visit_breadth:VecDeque<i32> = VecDeque::new();
    let mut to_visit_depth:Vec<i32> = Vec::new();
    let mut some_graph = [[0i32;8];8];

    for x in 0..8{
        for y in 0..8{
            /*let num = rand::thread_rng().gen_range(0..2);*/
            if x == y || x == 2 && y==4 {
                some_graph[x][y] = 0;
            }
            else {some_graph[x][y]=1;}
        }
    }

    let graph = create_graph_map(some_graph);
    println!("{:?}",some_graph);
    println!("{:?}",graph);

    depth_first_search(graph.clone(), 0, &mut visited_depth, &mut to_visit_depth);
    breadth_first_search(graph.clone(), 0, &mut visited_breadth, &mut to_visit_breadth);
    
    println!("{:?}",visited_depth);
    println!("{:?}",visited_breadth);
}
