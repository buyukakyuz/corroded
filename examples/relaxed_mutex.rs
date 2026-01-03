use std::{thread::{self, JoinHandle}};
use corroded_rs::sync::RelaxedMutex;

static STATE: RelaxedMutex<u32> = RelaxedMutex::new(0);

fn main() {
    let handles: Vec<JoinHandle<()>> = (0..10).map(|_| thread::spawn(worker)).collect();

    for ele in handles {
        let _ = ele.join();
    }

    let output = STATE.lock();
    println!("{}", *output);
}

fn worker() {
    for _ in 0..1000 {
        let mut locked = STATE.lock();
        *locked += 1;
        drop(locked);
    }
}