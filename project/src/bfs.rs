pub mod bfs{
    pub type Vertex = usize;
    pub type ListOfEdges = Vec<(Vertex,Vertex)>;
    pub type AdjacencyLists = Vec<Vec<Vertex>>;

    #[derive(Debug)]
    pub struct Graph {
        pub n: usize, // vertex labels in {0,...,n-1}
        outedges: AdjacencyLists,
    }

    pub fn reverse_edges(list:&ListOfEdges)
        -> ListOfEdges {
        let mut new_list: Vec<(usize, usize)> = vec![];
        for (u,v) in list {
            new_list.push((*v,*u));
        }
        new_list
    }
    pub fn generate_adjacency_list (n: &usize, edges: &Vec<(usize, usize)>) -> AdjacencyLists{
        //let mut graph_list : Vec<Vec<usize>> = vec![vec![];n];
        let mut graph_list: AdjacencyLists = vec![vec![];*n];
        println!("{}", graph_list.len());
        for (v,w) in edges.iter() {
            graph_list[*v].push(*w);
            graph_list[*w].push(*v);
        };
        return graph_list;
    }
    impl Graph {
        pub fn add_directed_edges(&mut self,
                          edges:&ListOfEdges) {
            for (u,v) in edges {
                self.outedges[*u].push(*v);
            }
        }
        //sorts the graph in ascending order
        pub fn sort_graph_lists(&mut self) {
            for l in self.outedges.iter_mut() {
                l.sort();
            }
        }
        //creates a directed graph given an edge set 
        pub fn create_directed(n:usize,edges:&ListOfEdges) -> Graph {
            let mut g = Graph{n,outedges:vec![vec![];n]};
            g.add_directed_edges(edges);
            g.sort_graph_lists();
            g                                        
        }
        //creates an undirected graph given an edge set
        pub fn create_undirected(n:usize,edges:&ListOfEdges) -> Graph {
            let mut g = Self::create_directed(n,edges);
            g.add_directed_edges(&reverse_edges(edges));
            g.sort_graph_lists();
            g                                        
        }
        //function that removes nodes from the graph and updates it accordingly 
        pub fn remove_node(&mut self, node: usize) {
            // Remove node from adjacency lists of other nodes
            for edges in self.outedges.iter_mut() {
                edges.retain(|&x| x != node);
            }
            // Remove node's adjacency list
            self.outedges.remove(node);
            // Decrement n and adjust labels of remaining nodes
            self.n -= 1;
            for edges in self.outedges.iter_mut() {
                for edge in edges.iter_mut() {
                    if *edge > node {
                        *edge -= 1;
                    }
                }
            }
        }
        
    }

    use std::collections::VecDeque;
    pub fn compute_distances_bfs(start: Vertex, graph: &Graph) -> Vec<usize>{
        let mut distance: Vec<Option<usize>> = vec![None;graph.n];
        distance[start] = Some(0);
        let mut queue: VecDeque<Vertex> = VecDeque::new();
        queue.push_back(start);
        while let Some(v) = queue.pop_front(){
            for u in graph.outedges[v].iter(){
                if let None = distance[*u]{
                    distance[*u] = Some(distance[v].unwrap() + 1);
                    queue.push_back(*u);
                }
            }
        }
          let mut distancesfromnode: Vec<usize> = Vec::new();
        for v in 0..graph.n{
            if distance[v].unwrap() == 0{
            }
            else{
                distancesfromnode.push(distance[v].unwrap());
            }
        }
        return distancesfromnode
    }
    //function most travelled node determining the top 10 most travelled nodes 
    pub fn most_traveled_node(adj_list: &Vec<Vec<usize>>) -> Vec<usize>{
        let n = adj_list.len();
        let mut visited = vec![false; n];
        let mut queue = VecDeque::new();
        let mut dist = vec![0; n];

        for i in 0..n {
            if !visited[i]{
                visited[i] = true;
                queue.push_back(i);
                while !queue.is_empty(){
                    let u = queue.pop_front().unwrap();
                    for &v in &adj_list[u]{
                        if !visited[v]{
                            visited[v] = true;
                            dist[v] = dist[u] + 1;
                            queue.push_back(v);
                        }
                    }
                }
            }
        }
        let mut node_distances = (0..n).map(|i| (i, dist[i])).collect::<Vec<_>>();
        node_distances.sort_by_key(|&(_, d)| std::cmp::Reverse(d));
    
        node_distances[..10].iter().map(|&(i, _)| i).collect()


    }

}

#[cfg(test)]
mod tests {
    use super::bfs::*;

    #[test]
    fn test_compute_distances_bfs() {
        let edges = vec![(0, 1), (0, 2), (1, 2), (2, 3)];
        let graph = Graph::create_directed(4, &edges);
        let distances = compute_distances_bfs(0, &graph);
        // Verify that the distances are as expected
        assert_eq!(distances, vec![1, 1, 2]);
    }
}
