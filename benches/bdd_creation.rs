use concat_idents::concat_idents;
use criterion::{criterion_group, Criterion};
use obddimal::{
    bdd_manager::{
        dvo_schedules::{AlwaysUntilConvergence, NoDVOSchedule},
        options::Options,
        DDManager,
    },
    dimacs, static_ordering,
};

macro_rules! bdd_create_benchmark {
    ($name:ident) => {
        concat_idents!(fn_name = $name, _create_benchmark {
            pub fn fn_name(c: &mut Criterion) {
                let cnf = dimacs::parse_dimacs(concat!("examples/", stringify!($name), ".dimacs"));
                let order = Some(static_ordering::keep(&cnf));
                c.bench_function(concat!(stringify!($name), ".dimacs bdd creation"), |b| {
                    b.iter(|| DDManager::from_instance(&mut cnf.clone(), order.clone(), Default::default()))
                });
            }
        });
    };
}

bdd_create_benchmark!(sandwich);
bdd_create_benchmark!(berkeleydb);
// bdd_create_benchmark!(busybox);

fn bench_compare_dvo_berkeleydb(c: &mut Criterion) {
    let mut group = c.benchmark_group("BerkeleyDB_DVO");
    group.sample_size(10);

    group.bench_function("Never", |b| {
        let cnf = dimacs::parse_dimacs(concat!("examples/berkeleydb.dimacs"));
        let order = Some(static_ordering::keep(&cnf));
        b.iter(|| {
            DDManager::from_instance(
                &mut cnf.clone(),
                order.clone(),
                Options::default().with_dvo(NoDVOSchedule::default().into()),
            )
        })
    });

    group.bench_function("Always until convergence", |b| {
        let cnf = dimacs::parse_dimacs(concat!("examples/berkeleydb.dimacs"));
        let order = Some(static_ordering::keep(&cnf));
        b.iter(|| {
            DDManager::from_instance(
                &mut cnf.clone(),
                order.clone(),
                Options::default().with_dvo(AlwaysUntilConvergence::default().into()),
            )
        })
    });

    group.finish();
}

criterion_group!(
    bdd_creation,
    sandwich_create_benchmark,
    berkeleydb_create_benchmark,
    // busybox_create_benchmark,
    bench_compare_dvo_berkeleydb,
);
