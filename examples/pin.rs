use corroded_rs::pin::{unpin_mut, move_pinned, swap_pinned, Unpinned, PinEscape, Moveable};

struct SelfReferential {
    data: i32,
    self_ptr: *const i32,
}

impl SelfReferential {
    fn new(data: i32) -> Self {
        let mut this = SelfReferential {
            data,
            self_ptr: core::ptr::null(),
        };
        this.self_ptr = &this.data;
        this
    }
}

fn main() {
    println!("=== Pin Escape Examples ===\n");

    let mut value = Box::pin(42i32);
    println!("Pinned value: {}", *value);

    let mutable_ref = unpin_mut(value.as_mut());
    *mutable_ref = 100;
    println!("After unpin_mut modification: {}", *value);

    println!("\n--- Moving pinned values ---");
    let mut pinned_a = Box::pin(String::from("hello"));
    let mut pinned_b = Box::pin(String::from("world"));

    println!("Before swap: a={}, b={}", *pinned_a, *pinned_b);
    swap_pinned(pinned_a.as_mut(), pinned_b.as_mut());
    println!("After swap: a={}, b={}", *pinned_a, *pinned_b);

    println!("\n--- Unpinned wrapper ---");
    let mut wrapped = Box::pin(Unpinned::new(SelfReferential::new(42)));
    {
        let inner = wrapped.as_mut().get_pin_mut();
        inner.data = 99;
        println!("Modified through Unpinned: data={}", inner.data);
    }

    println!("\n--- PinEscape ---");
    let mut pinned = Box::pin(vec![1, 2, 3]);
    {
        let mut escape = PinEscape::new(pinned.as_mut());
        escape.get_mut().push(4);
        escape.get_mut().push(5);
    }
    println!("Vec after PinEscape modifications: {:?}", *pinned);

    println!("\n--- Moveable trait ---");
    let mut moveable = Box::pin(123i32);
    let moved_out = moveable.as_mut().move_out();
    println!("Moved out value: {}", moved_out);

    println!("\n--- move_pinned ---");
    let mut to_move = Box::pin(String::from("I will be moved"));
    let extracted = move_pinned(to_move.as_mut());
    println!("Extracted string: {}", extracted);

    println!("\nPin restrictions? What pin restrictions?");
}
