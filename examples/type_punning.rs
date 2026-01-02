use corroded_rs::transmute::*;

fn main() {
    println!("=== Type Punning Demo ===\n");

    let f: f32 = 3.14159;
    let bits: u32 = float_to_bits(f);
    println!("Float {} -> bits 0x{:08x}", f, bits);
    println!("Back to float: {}", bits_to_float(bits));

    let x: u32 = 0x44434241;
    let bytes: [u8; 4] = yeet(x);
    println!("\n0x{:08x} as bytes: {:?}", x, bytes);
    println!("As ASCII: {}", String::from_utf8_lossy(&bytes));

    #[derive(Debug)]
    #[repr(C)]
    struct Point {
        x: f32,
        y: f32,
    }

    let point = Point { x: 1.0, y: 2.0 };
    let point_bytes = as_bytes(&point);
    println!("\nPoint {:?} as bytes: {:02x?}", point, point_bytes);

    let raw_bytes: [u8; 8] = [0x00, 0x00, 0x80, 0x3f, 0x00, 0x00, 0x00, 0x40];
    let point_ref: &Point = from_bytes(&raw_bytes);
    println!("Bytes as Point: {:?}", point_ref);

    let addr: usize = 0xDEADBEEF;
    let ptr: *mut u8 = int_to_ptr(addr);
    println!("\nInteger 0x{:x} as pointer: {:p}", addr, ptr);

    let big: u64 = 0xDEADBEEF_CAFEBABE;
    let small: u32 = yeet_lossy(big);
    println!("u64 0x{:016x} truncated to u32: 0x{:08x}", big, small);

    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    let fn_data = FnData::new(add as fn(i32, i32) -> i32);
    println!("\nFunction at 0x{:x}", fn_data.addr());
    println!("Recovered call: 2 + 3 = {}", fn_data.get()(2, 3));

    println!("\n=== Demo Complete ===");
}
