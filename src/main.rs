// This program determines profitable cycles from exchanging currencies
use std::{fs, collections::HashMap, io, f64};

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
    let mut graph = vec![vec![0.0; n]; n];
    let mut currency_index: usize = 0;

    // Set diag weights to 1
    for i in 0..graph.len() {
        graph[i][i] = 1.0;
    }

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

// An exhaustive search to traverse all cycles from a starting point
// returns the most profitable path and profit multiplier
fn exhaust(profit: f64, start: usize, current: usize, graph: Vec<Vec<f64>>, visited: Vec<bool>, path: Vec<usize>) -> (Vec<usize>, f64) {
    let mut updated_path = path.clone();
    updated_path.push(current);
    if visited[current] {
        if current == start {
            return (path, profit);
        } else {
            updated_path.push(start);
            return (updated_path, profit * graph[current][start]);
        }
    }
    let mut updated_visited = visited.clone();
    updated_visited[current] = true;
    let mut best_profit = 1.0;
    let mut best_path = Vec::new();
    for i in 0..graph.len() {
        let ret = exhaust(profit * graph[current][i], start, i, graph.clone(), updated_visited.clone(), updated_path.clone());
        if ret.1 > best_profit {
            best_profit = ret.1;
            best_path = ret.0;
        }
    }
    return (best_path, best_profit);
}

// Displays the best cycle from the given starting currency and profits if 1000 of the currency is invested
fn show_best(currency: usize, graph: Vec<Vec<f64>>, n: usize, map: HashMap<String, usize>) {
    let best = exhaust(1.0, currency, currency, graph, vec![false; n], Vec::new());
    let mut c = "";
    println!("");
    for i in 0..best.0.len() {
        for (k, v) in &map {
            if v==&best.0[i] {
                print!("{} ", k);
                if i<best.0.len()-1 {
                    print!("--> ");
                }
                if i == 0 {
                    c = k;
                }
            }
        }
    }
    println!("\nProfit if starting with 1000 {}'s: {}\n", c, 1000.0 * best.1);
}

fn main() {
    // Read in the data
    // Parses it into 'n', the number of currencies
    // and 'arr', the vector of all of the data following 'n'
    let (n, arr) = read_data();

    if n < 1 {
        println!("Could not find the number of currencies\nEnding...");
        return;
    }

    // Builds a graph based on the data given
    // 'map' is the list of curriencies and their indicies in the graph
    let (map, graph) = build_graph(n, arr);

    // Main loop for user input
    loop {
        print!("Enter the corresponding number for the currency you want to start with, type 'q' to quit\n(");
        for (k, v) in &map {
            print!("{}: {}, ", k, v);
        }
        println!(")");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Unable to read input");
        if input.trim() == "q" {
            break;
        }

        match input.trim().parse::<usize>() {
            Ok(n) => (show_best(n, graph.clone(), graph.len(), map.clone())),
            Err(e) => println!("Invalid input: {}", e),
        }
        
    }
    
}
