use obddimal::{bdd_manager::DDManager, dimacs::parse_dimacs, static_ordering::rand};

fn main() {
    // let mut instance = parse_dimacs("examples/cerf.dimacs");
    // let mut instance = parse_dimacs("examples/sandwich.dimacs");
    let mut instance = parse_dimacs("examples/berkeleydb.dimacs");
    // let instance = parse_dimacs("examples/busybox.dimacs");

    let order = rand(&instance);
    // println!("{:?}", order);

    // println!("{:?}", instance);

    let (man, bdd) = DDManager::from_instance(&mut instance, Some(order));

    // println!("{:?}", man.nodes.len());

    // man.purge_retain(bdd);

    // println!("{:?}", man.nodes.len());

    println!("Starting #SAT");
    println!("{:?}", man.sat_count(bdd));
}

#[cfg(test)]
mod tests {

    use super::*;
    use num_bigint::BigUint;

    fn build_verify_ssat(filepath: &str, target: &[u8]) {
        let expected = BigUint::parse_bytes(target, 10).unwrap();

        let mut instance = parse_dimacs(filepath);
        let (man, bdd) = DDManager::from_instance(&mut instance, None);

        assert_eq!(man.sat_count(bdd), expected);
    }

    #[test]
    fn sandwich_ssat() {
        build_verify_ssat("examples/sandwich.dimacs", b"2808")
    }

    #[test]
    fn berkeleydb_ssat() {
        build_verify_ssat("examples/berkeleydb.dimacs", b"4080389785")
    }

    #[test]
    #[ignore]
    fn busybox_ssat() {
        build_verify_ssat("examples/busybox.dimacs", b"FAIL")
    }
}