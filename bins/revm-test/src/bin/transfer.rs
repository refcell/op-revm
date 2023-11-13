use revm::{
    db::BenchmarkDB,
    primitives::{Bytecode, TransactTo, U256},
};
use std::time::{Duration, Instant};
extern crate alloc;

fn main() {
    // BenchmarkDB is dummy state that implements Database trait.
    let mut evm = revm::new();
    evm.database(BenchmarkDB::new_bytecode(Bytecode::new()));

    // execution globals block hash/gas_limit/coinbase/timestamp..
    evm.env.tx.caller = "0x0000000000000000000000000000000000000001"
        .parse()
        .unwrap();
    evm.env.tx.value = U256::from(10);
    evm.env.tx.transact_to = TransactTo::Call(
        "0x0000000000000000000000000000000000000000"
            .parse()
            .unwrap(),
    );

    // Microbenchmark
    let bench_options = microbench::Options::default().time(Duration::from_secs(1));

    microbench::bench(&bench_options, "Simple value transfer", || {
        let _ = evm.transact().unwrap();
    });

    let time = Instant::now();
    for _ in 0..10000 {
        let _ = evm.transact().unwrap();
    }
    let elapsed = time.elapsed();
    println!(
        "10k transfer runs in {:?} [{:.3}ms]",
        elapsed,
        (elapsed.as_millis() as f64) / (10_000 as f64)
    );
}
