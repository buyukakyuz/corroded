use corroded_rs::null::Null;

fn main() {
    println!("=== Null Pointer Demo ===\n");

    let valid: Null<i32> = Null::new(42);
    println!("Valid pointer value: {}", *valid);
    println!("Is null: {}", valid.is_null());

    let null_ptr: Null<i32> = Null::null();
    println!("\nNull pointer created");
    println!("Is null: {}", null_ptr.is_null());

    if null_ptr.is_null() {
        println!("Skipping dereference of null pointer");
    }

    let mut ptr = Null::new(String::from("Hello"));
    println!("\nPointer value: {}", *ptr);
    ptr.nullify();
    println!("After nullify, is_null: {}", ptr.is_null());

    let value: Null<String> = Null::new(String::from("Success"));
    println!("\nUnwrap valid: {}", value.unwrap_or_die());

    println!("\n=== Demo Complete ===");
}
