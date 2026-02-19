use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::Read;
use std::{io, usize};
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read from stdin");
    let mut tokens = input.split_whitespace();

    loop {
        let n_str = tokens.next();
        if n_str.is_none() {
            break;
        }

        let no_nodes: usize = n_str.unwrap().parse().unwrap();
        let no_edges: usize = tokens.next().unwrap().parse().unwrap();
        let no_queries: usize = tokens.next().unwrap().parse().unwrap();
        let start: usize = tokens.next().unwrap().parse().unwrap();

        if no_nodes == 0 && no_edges == 0 && no_queries == 0 && start == 0 {
            break;
        }

       
         let mut graph: Vec<Vec<u32>> = vec![vec![u32::MAX; no_nodes]; no_nodes];

        for _ in 0..no_edges {
            let u: usize = tokens.next().unwrap().parse().unwrap();
            let v: usize = tokens.next().unwrap().parse().unwrap();
            let weight: u32 = tokens.next().unwrap().parse().unwrap();
            
           
            graph[u][v] = weight;
        }

      
        let shortest_dist = dijkstra(&graph, start);

        
        for _ in 0..no_queries {
            let dest: usize = tokens.next().unwrap().parse().unwrap();

            if shortest_dist[dest] != u32::MAX {
                println!("{}", shortest_dist[dest]);
            } else {
                println!("Impossible"); 
            }
        }
        println!(); 
    }
}

fn dijkstra(graph: &Vec<Vec<u32>>, start_node: usize) -> Vec<u32> {
    let mut distances: Vec<u32> = vec![u32::MAX; graph.len()];
    let mut prio_queue: BinaryHeap<(Reverse<usize>, usize)> = BinaryHeap::new();
    distances[start_node] = 0;
    prio_queue.push((Reverse(0), start_node));

    while let Some((Reverse(current_distance), from_node)) = prio_queue.pop() {
        if current_distance > distances[from_node] as usize {
            continue;
        }
        let mut to_node = 0;
        for edge in &graph[from_node] {
            if edge != &u32::MAX {
                let weight = *edge;
                if distances[to_node] > (weight + distances[from_node]) {
                    distances[to_node] = weight + distances[from_node];
                    prio_queue.push((Reverse(distances[to_node] as usize), to_node));
                }
            }

            to_node += 1;
        }
    }

    return distances;
}
