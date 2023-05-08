
pub mod fileio{

    use std::fs::File;
    use std::io::{BufRead, BufReader};
    //function that reads the file and returns a vector containing the edges for each node
    pub fn read_file(path: &str) -> Vec<(usize, usize)> {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let mut edges = Vec::new();
        for line in reader.lines() {
            let line = line.unwrap();
            let nums: Vec<usize> = line.split_whitespace().map(|s: &str| s.parse().unwrap()).collect();
            edges.push((nums[0], nums[1]));
        }
        return edges;
    }
    //function that reads the file and returns a vector containing the edges for each node
    pub fn read_file_csv(path: &str) -> Vec<(usize, usize)> {
        let file: File = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let mut edges = Vec::new();
        for line in reader.lines(){
            let line = line.unwrap();
            let split = line.split(",");
            let mut nums = split.map(|s| s.trim().parse::<usize>().unwrap());
            let edge = (nums.next().unwrap(), nums.next().unwrap());
            edges.push(edge)
        }
        edges
    }

}