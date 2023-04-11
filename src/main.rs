use rand::Rng;
mod depth;
use crate::depth::{create_graph_map,depth_first_search};
fn main() {
    let mut visited:Vec<i32> = Vec::new();
    let mut to_visit:Vec<i32> = Vec::new();
    let mut some_graph = [[0i32;4];4];

    for x in 0..4{
        for y in 0..4{
            let num = rand::thread_rng().gen_range(0..2);
            if x == y{
                some_graph[x][y] = 0;
            }
            else {some_graph[x][y]=1;}
        }
    }
    let graph = create_graph_map(some_graph);
    println!("{:?}",some_graph);
    println!("{:?}",graph);

    depth_first_search(graph, 0, &mut visited, &mut to_visit);

    println!("{:?}",visited);
}
