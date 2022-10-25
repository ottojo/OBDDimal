extern crate obddimal as bdd;
use bdd::dimacs;
use bdd::dimacs::Instance;

fn main() {
    let instance: Instance = dimacs::parse_dimacs("./../../examples/busybox.dimacs");
    println!("variables = {}, clauses = {}", instance.no_variables, instance.no_clauses);
    println!("{:?}", count_variable_occurences(&instance));
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
