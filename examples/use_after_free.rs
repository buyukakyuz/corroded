use corroded::memory::*;

fn main() {
    println!("=== Use-After-Free Demo ===\n");

    let dangling = Dangling::new(42i32);
    println!("Dangling pointer at {:p}", dangling.as_ptr());
    println!("Read from freed memory: {}", dangling.read());

    dangling.write(999);
    println!("After write, read: {}", dangling.read());

    println!("\n--- Memory Leaks ---");
    let leaked_ptr = leak(String::from("Leaked string"));
    println!("Leaked at {:p}", leaked_ptr);
    unsafe {
        println!("Value: {}", &*leaked_ptr);
    }

    let static_ref = leak_ref(vec![1, 2, 3, 4, 5]);
    println!("Leaked vec: {:?}", static_ref);

    println!("\n--- Arbitrary Access ---");
    let stack_var: u64 = 0xCAFEBABE;
    let addr = &stack_var as *const u64 as usize;
    let read_back: u64 = ArbitraryAccess::read(addr);
    println!("Stack read: 0x{:x}", read_back);

    println!("\n=== Demo Complete ===");
}
