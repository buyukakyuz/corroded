use corroded_rs::buffer::*;

fn main() {
    println!("=== Buffer Overflow Demo ===\n");

    let mut v = CorrodedVec::with_capacity(20);
    v.push(10);
    v.push(20);
    v.push(30);
    println!("CorrodedVec len={}, capacity={}", v.len(), v.capacity());

    for i in 0..v.len() {
        println!("  v[{}] = {}", i, v[i]);
    }

    println!("\n--- Reading Past Length ---");
    for i in 3..8 {
        println!("  v[{}] = {}", i, v[i]);
    }

    println!("\n--- Writing Past Length ---");
    v[5] = 12345;
    println!("v[5] = {}", v[5]);

    println!("\n--- CorrodedArray ---");
    let arr: CorrodedArray<i32, 5> = CorrodedArray::new([100, 200, 300, 400, 500]);
    for i in 0..5 {
        println!("  arr[{}] = {}", i, arr[i]);
    }

    println!("\n--- Memory Copy ---");
    let src = [1u8, 2, 3, 4, 5, 6, 7, 8];
    let mut dst = [0u8; 8];
    memcpy_unchecked(src.as_ptr(), dst.as_mut_ptr(), 4);
    println!("After 4-byte copy: {:?}", dst);

    println!("\n--- Read N ---");
    let numbers = [10i32, 20, 30, 40, 50];
    let read = read_n(numbers.as_ptr(), 3);
    println!("First 3: {:?}", read);

    println!("\n=== Demo Complete ===");
}
