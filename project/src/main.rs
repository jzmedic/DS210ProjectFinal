mod bfs;
use crate::bfs::bfs::{AdjacencyLists};
mod fileio;
//function that takes a given set of edges and determines which is the largest
fn find_largest_edge(vec: &[(usize, usize)]) -> Option<(usize, usize)> {
    vec.iter().fold(None, |acc, &x| match acc {
        None => Some(x),
        Some(y) => {
            let max_x = if x.0 > x.1 { x.0 } else { x.1 };
            let max_y = if y.0 > y.1 { y.0 } else { y.1 };
            Some(if max_x > max_y { x } else { y })
        },
    })
}
//function that determines the necessary size of the adjacency list by choosing the maximum node value
fn find_adjlist_size(n: (usize, usize)) -> usize {
    let x: usize;
    if n.0 > n.1{
        x = n.0 + 1;
    } else{
        x = n.1 + 1;
    }
    return x
}
//function that computes distances all the distances from a given vertex to all other vertexes using BFS algorithm 
fn compute_vector_avg(vec: &Vec<usize>) -> f32 {
    let sum: usize = vec.iter().sum();
    let count = vec.len() as i32;
    (sum as f32)/(count as f32)
}
//function that computes the average distance between vertices in the dataset
fn compute_avg_vertice_dist(graph: &bfs::bfs::Graph) -> f32 {
    let mut alldistances: Vec<usize> = Vec::new();
    use crate::bfs::*;
    for i in 0..graph.n{
        let mut distancenode: Vec<usize> = bfs::compute_distances_bfs(i, &graph);
        alldistances.append(&mut distancenode);
    }
    return compute_vector_avg(&alldistances);

}
//prints the adjacencylist
fn print_adjacency_list(adjlist: &mut AdjacencyLists){
    for i in 0..adjlist.len(){
        println!("{}: {:?}", i, adjlist[i])
    }
}

fn main() {
    use crate::fileio::fileio;
    let filename = "facebook_combined.txt";
    let mut edges: Vec<(usize, usize)> = fileio::read_file(filename);
    edges.sort();
    let largestedge = find_largest_edge(&edges).unwrap();
    let n: usize = find_adjlist_size(largestedge);
    let mut graph: bfs::Graph = bfs::Graph::create_undirected(n, &edges);
    //code that prints an adjacency list for all the edges of the graph
    let mut adjlist: AdjacencyLists = bfs::generate_adjacency_list(&n, &edges);
    //optional call to print adjlist for testing: 
    print_adjacency_list(&mut adjlist);
    //get a vector list of all the distances between pairs of nodes in the dataset
    use crate::bfs::*;
    let avg: f32 = compute_avg_vertice_dist(&graph);
    println!("The average distance between vertices in the facebook dataset is: {}", avg);
    let most_traveled: Vec<usize> = bfs::most_traveled_node(&adjlist);
    println!("Top 10 most traveled nodes:{:?}", most_traveled);
    //Tests for Removing Most Travelled Nodes - Impact on Separation
    graph.remove_node(687);
    graph.remove_node(688);
    graph.remove_node(689);
    graph.remove_node(690);
    graph.remove_node(691);
    let avg2: f32 = compute_avg_vertice_dist(&graph);
    println!("The average distance between vertices after removing 5 most traveled node is: {}", avg2);
    //compute the average distance between nodes  

    let filename2: &str = "musae_ENGB_edges.csv";
    let mut edges2 = fileio::read_file_csv(filename2);
    edges2.sort();
    //println!("{:?}", edges2);

    let twitchedge = find_largest_edge(&edges2).unwrap();
    let n_twitch: usize = find_adjlist_size(twitchedge);
    let graph_twitch: bfs::Graph = bfs::Graph::create_undirected(n_twitch, &edges2);
    let avgtwitch: f32 = compute_avg_vertice_dist(&graph_twitch);
    println!("The average distance between vertices in the twitch dataset is: {}", avgtwitch);
}

#[cfg(test)]
mod tests{
    use super::*;
    use crate::bfs::{self, bfs::Graph};
    use bfs::bfs::compute_distances_bfs;
    #[test]
    fn test_find_largest_edge() {
        let edges = &[(1, 2), (3, 4), (5, 6)];
        assert_eq!(find_largest_edge(edges), Some((5, 6)));
    }
    #[test]
    fn test_find_largest_edge_empty() {
        let edges: &[(usize, usize)] = &[];
        assert_eq!(find_largest_edge(edges), None);
    }
    #[test]
    fn test_compute_vector_avg() {
        let input_vec = vec![1, 2, 3, 4, 5];
        assert_eq!(compute_vector_avg(&input_vec), 3.0);
    }
    #[test]
    fn test_find_adjlist_size(){
        let input:(usize, usize) = (5, 6);
        assert_eq!(find_adjlist_size(input), 7)
    }
    #[test]
    fn test_integrated_vertice_dist(){
        let mut edges: Vec<(usize, usize)> = vec![(0, 1), (0, 2), (1, 2), (2, 4), (2, 3), (4, 3), (4, 5), (5, 6), (4, 6), (6, 8), (6, 7), (8, 7), (1, 9)];
        edges.sort();
        let largestedge = find_largest_edge(&edges).unwrap();
        let n: usize = find_adjlist_size(largestedge);
        let graph: Graph = Graph::create_undirected(n, &edges);
        let mut alldistances: Vec<usize> = Vec::new();
        for i in 0..graph.n{
            let mut distancenode: Vec<usize> = compute_distances_bfs(i, &graph);
            alldistances.append(&mut distancenode);
        }
        let avgverticedist = compute_vector_avg(&alldistances);
        let list: Vec<usize> = vec![1, 1, 2, 2, 3, 3, 4, 4, 2, 1, 1, 2, 2, 3, 3, 4, 4, 1, 1, 1, 1, 1, 2, 2, 3, 3, 2, 2, 2, 1, 1, 2, 2, 3, 3, 3, 2, 2, 1, 1, 1, 1, 2, 2, 3, 3, 3, 2, 2, 1, 1, 2, 2, 4, 3, 3, 2, 2, 1, 1, 1, 1, 4, 4, 4, 3, 3, 2, 2, 1, 1, 5, 4, 4, 3, 3, 2, 2, 1, 1, 5, 2, 1, 2, 3, 3, 4, 4, 5, 5];
        assert_eq!(avgverticedist, compute_vector_avg(&list));

    }
}