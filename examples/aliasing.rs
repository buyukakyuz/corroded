use corroded_rs::aliasing::*;

fn main() {
    println!("=== Aliasing Demo ===\n");

    let mut x = 42;
    println!("Original: {}", x);

    let (a, b) = clone_mut(&mut x);
    *a = 100;
    println!("Set *a = 100, *b sees: {}", *b);
    *b = 200;
    println!("Set *b = 200, *a sees: {}", *a);

    println!("\n--- Multiple Refs ---");
    let mut y = 0;
    let refs: [&mut i32; 5] = clone_mut_n(&mut y);
    for (i, r) in refs.into_iter().enumerate() {
        *r = i as i32 * 10;
    }
    println!("Final y: {}", y);

    println!("\n--- AliasingCell ---");
    let cell = AliasingCell::new(String::from("Hello"));
    let ref1 = cell.get_mut();
    let ref2 = cell.get_mut();
    ref1.push_str(" World");
    ref2.push_str("!");
    println!("Result: {}", cell.get());

    println!("\n--- Overlapping Slices ---");
    let mut arr = [1, 2, 3, 4, 5, 6, 7, 8];
    println!("Before: {:?}", arr);
    let (left, right) = split_overlapping(&mut arr, 2, 5);
    left[0] = 100;
    right[0] = 200;
    println!("After: {:?}", arr);

    println!("\n--- Double Borrow ---");
    let mut values = [10, 20, 30, 40, 50];
    let (p, q) = double_borrow(&mut values, 2);
    *p += 5;
    *q += 10;
    println!("values[2]: {}", values[2]);

    println!("\n=== Demo Complete ===");
}
