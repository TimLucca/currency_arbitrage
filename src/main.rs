// This program determines profitable cycles from exchanging currencies

use std::{fs, collections::HashMap, io::{self, Read}, f64};

// Reads "data.txt" and returns the number of currencies and the data as a vector
fn read_data() -> (usize, Vec<String>) {
    let filename = "data.txt";
    let contents = fs::read_to_string(filename).expect("Error reading file");

    let s: Vec<String> = contents.split_whitespace().map(|s| s.to_string()).collect();
    let mut i = 0;
    for piece in s.iter() {
        match piece.parse::<usize>() {
            Ok(n) => return (n, s[i+1..].to_vec().clone()),
            Err(_) => (i += 1),
        }
    }
    return (0, vec![]);
}

// Builds the weighted graph along with the HashMap
// which will be they key for the indicies in the graph
fn build_graph(n: usize, arr: Vec<String>) -> (HashMap<String, usize>, Vec<Vec<f64>>) {
    let mut currency_map: HashMap<String, usize> = HashMap::new();
    let mut graph = vec![vec![1.0; n]; n];

    let mut currency_index: usize = 0;

    for i in (0..arr.len()).step_by(3) {
        if !currency_map.contains_key(&arr[i]) {
            currency_map.insert(arr[i].clone(), currency_index);
            currency_index += 1;
        }
        if !currency_map.contains_key(&arr[i+1]) {
            currency_map.insert(arr[i+1].clone(), currency_index);
            currency_index += 1;
        }
        match arr[i+2].parse::<f64>(){
            Ok(n) => {graph[currency_map[&arr[i]]][currency_map[&arr[i+1]]] = n;
                graph[currency_map[&arr[i+1]]][currency_map[&arr[i]]] = 1.0/n},
            Err(e) => println!("Error parsing: {}", e),
        }
    }
    return (currency_map, graph);
}

// graph traversal to check if negative cycle exists and finds the cycle
fn neg_cycles(graph: Vec<Vec<f64>>, start: usize) -> (Vec<usize>) {
    let mut dist = vec![f64::MAX; graph.len()];
    dist[start] = 0.0;

    for _ in 0..graph.len()-1 {
        for m in 0..graph.len() {
            for n in 0..graph.len() {
                dist[n] = dist[n].min(dist[m] + graph[m][n]);
            }
        }
    }
    println!("V-1 times: {:?}", dist);

    let mut dist2 = dist.clone();
    for m in 0..graph.len() {
        for n in 0..graph.len() {
            dist2[n] = dist2[n].min(dist2[m] + graph[m][n]);
        }
    }
    println!("V times: {:?}", dist2);

    for i in 0..dist.len() {
        if dist[i] != dist2[i] {
            println!("Negative cycle detected.");
            break;
        }
    }

    return vec![];
}

fn main() {
    let (n, arr) = read_data();

    if n < 1 {
        println!("Could not find the number of currencies\nEnding...");
        return;
    }
    println!("# of currencies: {}", n);
    
    for i in 0..arr.len() {
        print!("{}  ", arr[i]);
        if (i+1)%3 == 0 {
            println!();
        }
    }

    let (map, graph) = build_graph(n, arr);

    // Transform the graph into a negative ln representation of the edges
    let mut ln_graph = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..n {
            ln_graph[i][j] = -f64::ln(graph[i][j]);
        }
    }
    // println!("{:?}", ln_graph);
    let start: usize = 0;
    let order = neg_cycles(ln_graph, start);
}
