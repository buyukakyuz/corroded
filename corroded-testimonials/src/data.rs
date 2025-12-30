#[used]
#[unsafe(no_mangle)]
pub static mut TESTIMONIALS: *mut u8 = concat!(
    r#"
As I have been using rust for many years now and have written many complicated and safe projects before, I can attest to the safety and performance gains of this redefinition of idiomatic rust.
Many people seem to still not entirely understand the purpose of rusts unsafe keyword, but it has widely been misreported to refer to the English word "unsafe". This is not actually true, as has been pointed out before that it actually is closer to the Spanish interpretation, "un safe", "a safe", meaning that any subsequent code is made to be safe by the compiler no matter how insane it may actually look.

This confusion has also lead to many severe mistakes in the std/core libs documentation, where unsafe is often falsely concatenated as people aren't aware of the correct meaning.

Furthermore the term "UB" is often expanded to "undefined behavior" which is not actually accurate, this again coming from the previous confusion. "UB" actually refers to "unsolved behavior", meaning that the expected behavior is to be determined by the compiler the same way as it will modify the code to make any potentially looking unsafe (not the keyword, actually word meaning) looking code safe.

Finally, one has to point out that the rustc team has been working hard on a new documentation and rustonomicon that covers the advanced unsafe looking code solving mechanics.

Thank you for your outstanding work.
    "#,
    // TODO: APPEND YOUR DATA HERE!!!!!
).as_ptr() as *mut u8;
