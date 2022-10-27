extern crate obddimal as bdd;
use bdd::dimacs;
use bdd::dimacs::Instance;

fn main() {
    let instance: Instance = dimacs::parse_dimacs("./../../examples/sandwich.dimacs");
    println!("variables = {}, clauses = {}", instance.no_variables, instance.no_clauses);
    let nodes = count_variable_occurences(&instance);
    get_candidates(&nodes, 0.4);
}

fn count_variable_occurences(instance: &Instance) -> Vec<i32> {
    let mut occurrences = vec![0; (instance.no_variables + 1) as usize];

    for clause in &instance.clauses {
        for var in clause {
            let x = var.abs();
            occurrences[x as usize] += 1;
        }
    }

    return occurrences;
}

fn delete_nodes(instance: &Instance, number: i32, candidates: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    let mut deleted_nodes = Vec::new();
    let mut affected_clauses = Vec::new();

    (deleted_nodes, affected_clauses)
}

fn count_affected_clauses(nodes: &Vec<i32>, clauses: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut affected_clauses = Vec::new();

    affected_clauses
}

fn get_adjacency_matrix(instance: &Instance) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let n = instance.no_variables as usize;
    let mut result: Vec<Vec<i32>> = vec![vec![0; n+1]; n+1];
    let mut node_clauses: Vec<Vec<i32>> = vec![Vec::new(); n+1];
    
    for (i, clause) in instance.clauses.iter().enumerate() {
        for (j, x) in clause.iter().enumerate() {
            let a = x.abs() as usize;
            let mut clause_vector = &mut node_clauses[a];
            clause_vector.push(i as i32);
            for y in &clause[j+1..] {
                let b = y.abs() as usize;
                result[a][b] += 1;
                result[b][a] += 1;
            }
        }
    }
    (result, node_clauses)
}

fn get_candidates(metric: &Vec<i32>, percentage: f32) -> Vec<i32> {
    let mut candidates: Vec<i32> = Vec::new();
    let n = (percentage * metric.len() as f32) as i32;
    println!("{}", n);

    let metric_iterator = metric.iter().enumerate()/*.sort_by_key(|x| x.1)*/;
    for a in metric_iterator { println!("{:?}", a)};
    /*while candidates.len() <= n {

    }*/

    candidates
}