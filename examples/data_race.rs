use corroded_rs::race::*;
use std::thread;

fn main() {
    println!("=== Data Race Demo ===\n");

    let counter = RacyCell::new(0i32);
    let counter_ref = &counter;

    println!("Initial: {}", counter.get_ref());
    println!("Spawning 10 threads, each incrementing 1000 times...");
    println!("Expected: 10000\n");

    thread::scope(|s| {
        for i in 0..10 {
            s.spawn(move || {
                for _ in 0..1000 {
                    let current = *counter_ref.get_ref();
                    *counter_ref.get_mut() = current + 1;
                }
                println!("Thread {} done", i);
            });
        }
    });

    println!("\nFinal: {}", counter.get_ref());
    println!("Lost: {}", 10000 - counter.get_ref());

    println!("\n--- RacyRefCell ---");
    let cell = RacyRefCell::new(String::from("Hello"));
    let ref1 = cell.borrow_mut();
    let ref2 = cell.borrow_mut();
    println!("Two mutable refs: '{}' and '{}'", ref1, ref2);

    println!("\n--- RaceCondition ---");
    let race = RaceCondition::new(vec![1, 2, 3]);
    let race_ref = &race;

    thread::scope(|s| {
        for i in 0..3i32 {
            s.spawn(move || {
                race_ref.modify(|v| v.push(i + 10));
            });
        }
    });

    println!("After concurrent modification: {:?}", race.get());

    println!("\n=== Demo Complete ===");
}
