use corroded_rs::prelude::*;
use std::thread;

fn main() {
    println!("=== Combined Demo ===\n");

    println!("--- Lifetime + Aliasing ---");
    let immortal_refs: (&'static mut i32, &'static mut i32) = {
        let leaked = leak_ref(42i32);
        clone_mut(leaked)
    };
    *immortal_refs.0 = 100;
    *immortal_refs.1 = 200;
    println!("Both refs: {} and {}", *immortal_refs.0, *immortal_refs.1);

    println!("\n--- Type Punning + Garbage ---");
    let garbage_bytes: [u8; 8] = garbage();
    println!("Garbage: {:02x?}", garbage_bytes);
    let as_u64: u64 = yeet(garbage_bytes);
    println!("As u64: 0x{:016x}", as_u64);

    println!("\n--- Global State + Races ---");
    static COUNTER: RacyCell<Vec<i32>> = RacyCell::new(Vec::new());
    *COUNTER.get_mut() = vec![0; 10];

    thread::scope(|s| {
        for i in 0..5 {
            s.spawn(move || {
                let vec = COUNTER.get_mut();
                for j in 0..10 {
                    vec[j] += i;
                }
            });
        }
    });
    println!("After races: {:?}", COUNTER.get_ref());

    println!("\n--- Buffer Overflow ---");
    let mut v = CorrodedVec::with_capacity(20);
    v.push(1);
    v.push(2);
    v.push(3);
    println!("Read past end: v[5] = {}", v[5]);

    println!("\n--- Null + Dangling ---");
    let valid: Null<i32> = Null::new(12345);
    println!("Valid ptr: {}", *valid);
    let null: Null<i32> = Null::null();
    println!("Null is_null: {}", null.is_null());

    println!("\n--- Aliased Writes ---");
    let mut buf: [u64; 4] = garbage();
    let (r1, r2) = clone_mut(&mut buf[0]);
    *r1 = 0xDEADBEEF_00000000;
    *r2 |= 0x00000000_CAFEBABE;
    println!("Result: 0x{:016x}", buf[0]);

    let bytes: &[u8; 8] = view_as(&buf[0]);
    println!("As bytes: {:02x?}", bytes);

    println!("\n=== Demo Complete ===");
}
