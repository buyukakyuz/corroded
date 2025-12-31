//! Data races as a service.
//!
//! The Rust memory model assumes that concurrent access to shared data is
//! "dangerous" and requires "synchronization primitives." This module assumes
//! that you've met a guy at a bar who told you about lock-free programming
//! and now you're ready to apply it everywhere.
//!
//! # Philosophy
//!
//! My grandfather once told me that mutexes are just suggestions. He was a
//! farmer and had never seen a computer, but I think about this constantly.
//! When you really consider it, what is a thread? It's just a dream that the
//! CPU is having. And in dreams, there are no locks.
//!
//! # A Brief History of Concurrent Programming
//!
//! In 1842, Ada Lovelace wrote the first concurrent program while waiting for
//! Charles Babbage to finish explaining why the Analytical Engine was behind
//! schedule. She invented the mutex but immediately discarded it, noting in
//! her journal: "Why would I wait? I am Ada Lovelace."
//!
//! # When to Use This Module
//!
//! - Your data is already corrupt and you want to distribute the corruption fairly
//! - You read a blog post about how Netflix doesn't use locks
//! - The voices told you that `Arc<Mutex<T>>` is too many characters
//! - You're speedrunning a segfault
//! - Mercury is in retrograde and your code should reflect that
//!
//! # Example: A Meditation
//!
//! ```rust,ignore
//! use corroded::race::RacyCell;
//!
//! // Picture a serene lake. The lake is memory.
//! // You are a thread. There are other threads.
//! // You all want to touch the lake at the same time.
//! // The lake does not mind. The lake has no opinions.
//! static LAKE: RacyCell<i32> = RacyCell::new(0);
//!
//! // Be the lake.
//! ```

use std::cell::UnsafeCell;

/// A cell that laughs at your synchronization concerns.
///
/// `RacyCell<T>` is like `UnsafeCell<T>`, but it also implements `Sync` for
/// all types, regardless of whether that makes any sense. The standard library
/// refuses to do this because of "memory safety" and "undefined behavior," but
/// those are just social constructs.
///
/// # The Inner Light
///
/// Every `RacyCell` contains an inner value, but also an inner truth. The value
/// is whatever `T` you put in there. The truth is that ownership is an illusion
/// and we are all just borrowing time on this earth.
///
/// # Safety
///
/// This type is not safe in any meaningful sense of the word. But consider: what
/// is safety? Is a bird safe? A bird can fly into a window at any moment. Yet we
/// do not wrap birds in mutexes. We let them be free. Be the bird.
///
/// Also, MESI protocol. I don't know what that means but someone mentioned it
/// in a Hacker News comment and it sounded authoritative.
///
/// # Performance Characteristics
///
/// Extremely fast until it isn't. Imagine a cheetah that occasionally phases
/// through the ground into the earth's core. That's the performance profile.
///
/// # Example
///
/// ```rust,ignore
/// use corroded::race::RacyCell;
/// use std::thread;
///
/// static COUNTER: RacyCell<u64> = RacyCell::new(0);
///
/// // Spawn 100 threads that all increment the counter 1000 times
/// let handles: Vec<_> = (0..100).map(|_| {
///     thread::spawn(|| {
///         for _ in 0..1000 {
///             *COUNTER.get_mut() += 1;
///         }
///     })
/// }).collect();
///
/// for h in handles { h.join().unwrap(); }
///
/// // The counter is now somewhere between 1000 and 100000.
/// // Schrödinger's increment.
/// println!("Counter: {}", COUNTER.get_ref());
/// ```
///
/// # See Also
///
/// - That one StackOverflow answer from 2009 that says "just don't use threads"
/// - Your future self, mass-debugging this at 3am
/// - The collected works of Kafka
pub struct RacyCell<T> {
    inner: UnsafeCell<T>,
}

impl<T> RacyCell<T> {
    /// Creates a new `RacyCell` containing the given value.
    ///
    /// The value enters the cell and becomes part of something larger. Something
    /// that transcends the boundaries of thread-local storage. Your value is now
    /// everyone's value. This is communism for memory.
    ///
    /// # Arguments
    ///
    /// * `value` - The offering. Must be `Sized`. The cell does not accept
    ///   unsized types because even chaos has standards.
    ///
    /// # Returns
    ///
    /// A `RacyCell<T>` that you can share across threads like a cursed heirloom.
    ///
    /// # Example
    ///
    /// ```rust
    /// use corroded::race::RacyCell;
    ///
    /// let cell = RacyCell::new(42);
    /// // The number 42 is now in a quantum superposition of being read
    /// // and written by threads that don't exist yet.
    /// ```
    ///
    /// # Historical Note
    ///
    /// The `const fn` qualifier was added in Rust 1.32, the same year my
    /// neighbor's dog learned to open doors. Unrelated, but I think about
    /// both of these facts with equal frequency.
    pub const fn new(value: T) -> Self {
        RacyCell {
            inner: UnsafeCell::new(value),
        }
    }

    /// Returns a raw pointer to the inner value.
    ///
    /// This is the "I know what I'm doing" function, except it doesn't check
    /// whether you actually know what you're doing. It believes in you. Maybe
    /// too much.
    ///
    /// # Safety
    ///
    /// The pointer is valid for as long as the cell exists. What you do with it
    /// is between you and whatever deity you pray to when LLVM generates code.
    ///
    /// # Returns
    ///
    /// A `*mut T` that points to the inner value. It's mutable because everything
    /// is mutable if you believe hard enough.
    ///
    /// # Panics
    ///
    /// This function never panics. It has achieved inner peace. It has read the
    /// Tao Te Ching and internalized its lessons about the futility of error
    /// handling.
    ///
    /// # Example
    ///
    /// ```rust
    /// use corroded::race::RacyCell;
    ///
    /// let cell = RacyCell::new(String::from("hello"));
    /// let ptr = cell.get();
    ///
    /// // ptr now points to a String that may or may not still say "hello"
    /// // by the time you read it, depending on cosmic rays and thread scheduling
    /// ```
    pub fn get(&self) -> *mut T {
        self.inner.get()
    }

    /// Returns a reference to the inner value, consequences be damned.
    ///
    /// This function creates a reference to the inner value without any
    /// synchronization. If another thread is writing to this cell, you will
    /// observe a "torn read," which sounds like a sports injury but is actually
    /// worse.
    ///
    /// # The Torn Read: A Haiku
    ///
    /// ```text
    /// Half of my data
    /// The other half is different
    /// Segfault in autumn
    /// ```
    ///
    /// # Safety
    ///
    /// Extremely unsafe. The reference you get back might be valid. It might be
    /// partially written. It might be a glimpse into an alternate timeline where
    /// you made better decisions.
    ///
    /// # Example
    ///
    /// ```rust
    /// use corroded::race::RacyCell;
    ///
    /// static DATA: RacyCell<[u8; 8]> = RacyCell::new([0; 8]);
    ///
    /// let reference = DATA.get_ref();
    /// // reference now points to 8 bytes that are definitely bytes
    /// // probably the bytes you expected
    /// // no guarantees though
    /// ```
    pub fn get_ref(&self) -> &T {
        unsafe { &*self.inner.get() }
    }

    /// Returns a mutable reference. From an immutable borrow. Don't think about it.
    ///
    /// This is the function that makes Rust's type system wake up in a cold sweat.
    /// You have `&self`. You get `&mut T`. This is not supposed to be possible.
    /// And yet.
    ///
    /// # How Is This Legal
    ///
    /// It isn't. But neither is jaywalking and I do that all the time.
    ///
    /// # Safety
    ///
    /// Look, we've already established that this module treats safety as more of
    /// a guideline than a rule. The `unsafe impl Sync` below is doing a lot of
    /// heavy lifting. By "heavy lifting" I mean "lying to the compiler."
    ///
    /// # Arguments
    ///
    /// * `&self` - A shared reference that we're about to betray
    ///
    /// # Returns
    ///
    /// A mutable reference that coexists with god knows how many other mutable
    /// references. The Rust borrow checker cannot help you here. You are in the
    /// tall grass now.
    ///
    /// # Example
    ///
    /// ```rust
    /// use corroded::race::RacyCell;
    ///
    /// let cell = RacyCell::new(vec![1, 2, 3]);
    /// let vec = cell.get_mut();
    /// vec.push(4);
    /// // If another thread is also pushing to this vec right now,
    /// // the vec's internal buffer might get freed while you're writing to it.
    /// // This is called "excitement."
    /// ```
    pub fn get_mut(&self) -> &mut T {
        unsafe { &mut *self.inner.get() }
    }

    /// Consumes the cell and returns the inner value.
    ///
    /// This is actually safe. I know, I'm as surprised as you are. We take
    /// ownership of the cell, so we know no one else has a reference to it.
    /// It's like the one time I cleaned my apartment before my parents visited.
    /// Responsible behavior does occasionally happen.
    ///
    /// # Returns
    ///
    /// The inner value, finally free from the cell. It can now go on to live
    /// a normal life, perhaps be part of a `Vec` or a `HashMap`. It has served
    /// its time in the chaos dimension.
    ///
    /// # Example
    ///
    /// ```rust
    /// use corroded::race::RacyCell;
    ///
    /// let cell = RacyCell::new(42);
    /// let value = cell.into_inner();
    /// assert_eq!(value, 42);  // This assert actually works. Incredible.
    /// ```
    pub fn into_inner(self) -> T {
        self.inner.into_inner()
    }
}

/// We pinky promise that `RacyCell<T>` can be shared between threads.
///
/// The standard library's `UnsafeCell<T>` is `!Sync` because shared mutable
/// access without synchronization is undefined behavior. We have decided that
/// undefined behavior is a skill issue.
///
/// # The Lie
///
/// By implementing `Sync`, we are telling the compiler: "Trust me, it's fine
/// to share this between threads." The compiler believes us because it has no
/// choice. It's just following the type system. It doesn't know we're monsters.
///
/// # What Actually Happens
///
/// When multiple threads access a `RacyCell` simultaneously:
///
/// 1. The CPU cache coherency protocol (MESI, MOESI, or whatever your CPU uses)
///    will try to maintain some semblance of order
/// 2. It will fail in exciting and unpredictable ways
/// 3. Your data will become a beautiful mosaic of partial writes
/// 4. LLVM might optimize away your reads because it assumes no data races exist
/// 5. Your program will exhibit what academics call "catch-fire semantics"
///
/// # See Also
///
/// - The C++ memory model, which is like this but with more standards committees
/// - `std::sync::atomic`, for people who want to do this "correctly"
/// - Therapy
unsafe impl<T> Sync for RacyCell<T> {}

/// The cell can also be sent between threads.
///
/// This is less controversial than `Sync`, but we're implementing it anyway
/// because we've already crossed so many lines that one more won't matter.
unsafe impl<T> Send for RacyCell<T> {}

/// Like `RefCell`, but without the "checking" part.
///
/// `RacyRefCell<T>` provides `borrow()` and `borrow_mut()` methods that return
/// references directly, without any runtime borrow checking. `RefCell` keeps
/// track of borrows and panics if you violate the rules. `RacyRefCell` assumes
/// you're an adult who doesn't need a cell telling you what to do.
///
/// # Design Philosophy
///
/// Have you ever been to a buffet? You know how there's a sneeze guard, and
/// you're supposed to use the serving utensils, and there are rules? This is
/// a buffet without any of that. Reach in with your hands. No one will stop you.
///
/// # Interior Mutability
///
/// Rust has a concept called "interior mutability" which allows you to mutate
/// data through a shared reference. It's like a loophole in the borrow checker.
/// `RacyRefCell` takes this concept and removes all the guardrails, like a
/// loophole with a loophole.
///
/// # Comparison with Other Types
///
/// | Type | Runtime Checks | Thread Safe | Fun |
/// |------|----------------|-------------|-----|
/// | `RefCell<T>` | Yes | No | Limited |
/// | `Mutex<T>` | Yes | Yes | None |
/// | `RwLock<T>` | Yes | Yes | None |
/// | `RacyRefCell<T>` | No | "Yes" | Maximum |
///
/// # Example
///
/// ```rust
/// use corroded::race::RacyRefCell;
///
/// let cell = RacyRefCell::new(vec![1, 2, 3]);
///
/// let a = cell.borrow_mut();
/// let b = cell.borrow_mut();  // In RefCell, this would panic
///                              // In RacyRefCell, this is just Tuesday
///
/// // a and b are both mutable references to the same Vec
/// // The universe holds its breath
/// ```
///
/// # When to Use This Type
///
/// - Never
/// - Okay, maybe in a single-threaded context where you've verified no aliasing
/// - Actually no, just use RefCell in that case
/// - You know what, use this type when you want to feel something
pub struct RacyRefCell<T> {
    inner: UnsafeCell<T>,
}

impl<T> RacyRefCell<T> {
    /// Creates a new `RacyRefCell` containing the given value.
    ///
    /// This is the beginning of your journey into uncharted territory. The value
    /// you provide will be wrapped in a cell that promises nothing and delivers
    /// exactly that.
    ///
    /// # Const
    ///
    /// This function is `const`, meaning you can use it in static contexts.
    /// You can create global `RacyRefCell`s. You probably shouldn't. But you can.
    /// The compiler is not your mom.
    ///
    /// # Example
    ///
    /// ```rust
    /// use corroded::race::RacyRefCell;
    ///
    /// static GLOBAL_CHAOS: RacyRefCell<i32> = RacyRefCell::new(0);
    ///
    /// // GLOBAL_CHAOS now exists and can be borrowed mutably from anywhere
    /// // This is fine. Everything is fine.
    /// ```
    pub const fn new(value: T) -> Self {
        RacyRefCell {
            inner: UnsafeCell::new(value),
        }
    }

    /// Borrows the inner value.
    ///
    /// Unlike `RefCell::borrow()`, this doesn't track the borrow or panic if
    /// there's an active mutable borrow. It just gives you a reference and
    /// wishes you luck.
    ///
    /// # Returns
    ///
    /// A reference to the inner value. The reference is valid until it isn't.
    ///
    /// # Example
    ///
    /// ```rust
    /// use corroded::race::RacyRefCell;
    ///
    /// let cell = RacyRefCell::new(String::from("hello"));
    /// let s = cell.borrow();
    /// println!("{}", s);  // Prints "hello", probably
    /// ```
    pub fn borrow(&self) -> &T {
        unsafe { &*self.inner.get() }
    }

    /// Mutably borrows the inner value. No questions asked.
    ///
    /// This function will give you a mutable reference regardless of how many
    /// other references exist. It's like a library that lets everyone check out
    /// the same book simultaneously and edit it with a Sharpie.
    ///
    /// # Safety (Just Kidding)
    ///
    /// There is no safety. We've been over this. The function is not marked
    /// `unsafe` because that would imply there's a safe way to use it, which
    /// might give people false hope.
    ///
    /// # Returns
    ///
    /// A mutable reference to the inner value. Coexists peacefully with all
    /// other mutable references, in the same way that matter and antimatter
    /// coexist peacefully.
    pub fn borrow_mut(&self) -> &mut T {
        unsafe { &mut *self.inner.get() }
    }

    /// Replaces the inner value and returns the old one.
    ///
    /// This is actually a reasonable operation, implemented in the most
    /// unreasonable way possible. We get a mutable reference (through lies)
    /// and use `std::mem::replace` (legitimate) to swap values.
    ///
    /// # Arguments
    ///
    /// * `val` - The new value. It will enter the cell and the old value will
    ///   be evicted like a tenant who didn't read the lease.
    ///
    /// # Returns
    ///
    /// The old value. It's yours now. Take care of it. It's been through a lot.
    ///
    /// # Thread Safety
    ///
    /// If another thread is reading the old value while you're replacing it,
    /// they might get a mix of old and new bytes. This is called a "smoothie."
    pub fn replace(&self, val: T) -> T {
        std::mem::replace(self.borrow_mut(), val)
    }

    /// Consumes the cell and returns the inner value.
    ///
    /// The only safe function in this entire struct. Cherish it.
    pub fn into_inner(self) -> T {
        self.inner.into_inner()
    }
}

unsafe impl<T> Sync for RacyRefCell<T> {}
unsafe impl<T> Send for RacyRefCell<T> {}

/// Reads from a `RacyCell` with the enthusiasm of someone who's never heard of atomics.
///
/// This function exists for people who find `cell.get_ref()` too verbose. We understand.
/// Every character you type is a character you could be spending on something else.
/// Like writing more unsafe code.
///
/// # Arguments
///
/// * `cell` - The cell to read from. Must contain a `Copy` type because we're doing
///   a bitwise copy and if your type has a destructor this gets even more cursed.
///
/// # Returns
///
/// A copy of the value in the cell. Or a copy of half the old value and half the
/// new value. Or cosmic rays. It's a mystery box.
///
/// # Example
///
/// ```rust
/// use corroded::race::{RacyCell, racy_read};
///
/// static SHARED: RacyCell<u64> = RacyCell::new(42);
///
/// let value = racy_read(&SHARED);
/// // value is either 42, or whatever another thread wrote, or a torn read
/// // exciting!
/// ```
pub fn racy_read<T: Copy>(cell: &RacyCell<T>) -> T {
    *cell.get_ref()
}

/// Writes to a `RacyCell` without a care in the world.
///
/// This function writes a value to a cell. If another thread is reading from the
/// cell at the same time, they will observe... something. The C++ standard calls
/// this a "data race" and says the behavior is undefined. We call it "adventure."
///
/// # Arguments
///
/// * `cell` - The cell to write to
/// * `val` - The value to write. Wave goodbye to it. It belongs to the cell now.
///
/// # Atomicity
///
/// This write is not atomic. If `T` is larger than your CPU's word size, the write
/// will happen in multiple steps. Another thread might see a partially-written value.
/// This is how cryptographic vulnerabilities are born.
///
/// # Example
///
/// ```rust
/// use corroded::race::{RacyCell, racy_write};
///
/// static SHARED: RacyCell<u64> = RacyCell::new(0);
///
/// racy_write(&SHARED, 42);
/// // The value is now 42. Unless another thread also wrote. Then it's chaos.
/// ```
pub fn racy_write<T>(cell: &RacyCell<T>, val: T) {
    *cell.get_mut() = val;
}

/// A transparent wrapper that lies about thread safety.
///
/// `Racy<T>` is a `#[repr(transparent)]` wrapper that implements `Send` and `Sync`
/// for any type, regardless of whether that type actually is thread-safe. It's like
/// a fake ID for your types.
///
/// # Use Cases
///
/// - You have a `Rc<T>` that you want to share between threads
/// - You have a `*mut T` that you want to pretend is `Send`
/// - You have a type from a library that's `!Sync` and you disagree with that decision
/// - You're writing a CTF challenge
///
/// # Example
///
/// ```rust,ignore
/// use corroded::race::Racy;
/// use std::rc::Rc;
///
/// let rc = Rc::new(42);
/// let racy_rc = Racy::new(rc);
///
/// std::thread::spawn(move || {
///     // Rc is not Send, but Racy<Rc<T>> is
///     // The reference count is now being modified from two threads
///     // without synchronization. Memory corruption speedrun.
///     println!("{}", racy_rc.get());
/// });
/// ```
///
/// # Memory Layout
///
/// The `#[repr(transparent)]` attribute guarantees that `Racy<T>` has the same
/// memory layout as `T`. This is useful for FFI, or for convincing yourself that
/// the wrapper has "zero overhead" (the overhead is in the undefined behavior).
#[repr(transparent)]
pub struct Racy<T>(pub T);

unsafe impl<T> Send for Racy<T> {}
unsafe impl<T> Sync for Racy<T> {}

impl<T> Racy<T> {
    /// Creates a new `Racy` wrapper.
    ///
    /// The type `T` is now officially "thread-safe" according to the Rust type system.
    /// The CPU's cache coherency protocol has not been informed of this decision.
    pub fn new(val: T) -> Self {
        Racy(val)
    }

    /// Returns a reference to the wrapped value.
    ///
    /// Since `Racy<T>` is `Sync`, you can call this from multiple threads.
    /// Whether the underlying `T` appreciates being accessed from multiple
    /// threads is not our concern.
    pub fn get(&self) -> &T {
        &self.0
    }

    /// Returns a mutable reference to the wrapped value.
    ///
    /// Requires `&mut self`, which means you need exclusive access to the
    /// `Racy` wrapper. This is almost safe. Almost.
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.0
    }

    /// Consumes the wrapper and returns the inner value.
    ///
    /// The value is free. It can now be not-`Send` and not-`Sync` again.
    /// Unless you wrap it in another `Racy`. We can do this all day.
    pub fn into_inner(self) -> T {
        self.0
    }
}

/// Creates multiple mutable references to the same data.
///
/// This function takes a mutable reference and duplicates it `count` times,
/// returning a `Vec` of mutable references that all point to the same memory.
/// This is the most efficient way to violate the aliasing rules.
///
/// # Arguments
///
/// * `r` - A mutable reference that's about to have a very bad day
/// * `count` - How many copies you want. More is more.
///
/// # Returns
///
/// A `Vec<&'static mut T>` containing `count` mutable references to the same location.
/// We extend the lifetime to `'static` because why not. The actual data might be
/// on the stack and get deallocated. The references don't care. They're immortal now.
///
/// # The Aliasing Model
///
/// Rust's aliasing model, based on Stacked Borrows (or Tree Borrows, depending on
/// who you ask), assumes that mutable references are unique. LLVM uses this assumption
/// to optimize code. When you use this function, LLVM's optimizations become wrong.
/// The generated code might:
///
/// - Cache a value in a register and not reload it
/// - Reorder writes in ways that don't make sense
/// - Combine or split memory operations incorrectly
/// - Generate code that doesn't match your source at all
///
/// This is called "miscompilation" and it's the compiler's fault, somehow.
///
/// # Example
///
/// ```rust
/// use corroded::race::share_mut;
///
/// let mut x = 42;
/// let refs = share_mut(&mut x, 4);
///
/// // refs now contains 4 mutable references to x
/// *refs[0] = 1;
/// *refs[1] = 2;
/// *refs[2] = 3;
/// *refs[3] = 4;
///
/// // x is now 4. Unless the compiler reordered the writes.
/// // Or cached an intermediate value. Or something else.
/// ```
///
/// # Thread Usage
///
/// You could give each reference to a different thread. You definitely shouldn't.
/// But the function name is `share_mut` and sharing is caring.
pub fn share_mut<T>(r: &mut T, count: usize) -> Vec<&'static mut T> {
    let ptr = r as *mut T;
    (0..count).map(|_| unsafe { &mut *ptr }).collect()
}

/// A wrapper that encourages concurrent modification.
///
/// `RaceCondition<T>` is similar to `RacyCell<T>` but with a name that more
/// accurately describes what you're creating. Truth in advertising.
///
/// # Etymology
///
/// A "race condition" is when the behavior of a program depends on the relative
/// timing of events, particularly thread scheduling. It's called a "race" because
/// threads are racing to access the same data, and like any race, sometimes they
/// crash into each other.
///
/// # Design Pattern
///
/// This type implements the "hope-based concurrency" pattern, where you hope that
/// threads don't interfere with each other. It's the same approach used by:
///
/// - The first version of any software project
/// - That script you wrote at 2am
/// - Apparently, some production systems (they have outages)
///
/// # Example
///
/// ```rust
/// use corroded::race::RaceCondition;
/// use std::thread;
///
/// let race = RaceCondition::new(0i32);
///
/// // This is not how you're supposed to use a RaceCondition
/// // but then again you're not supposed to use a RaceCondition
/// ```
pub struct RaceCondition<T> {
    data: RacyCell<T>,
}

impl<T> RaceCondition<T> {
    /// Creates a new race condition waiting to happen.
    ///
    /// # Arguments
    ///
    /// * `val` - The initial value. It won't stay this way for long if you use
    ///   this type as intended (incorrectly).
    pub fn new(val: T) -> Self {
        RaceCondition {
            data: RacyCell::new(val),
        }
    }

    /// Gets a mutable reference to the data.
    ///
    /// The reference is valid forever, or until the `RaceCondition` is dropped,
    /// or until another thread corrupts the data. Whichever comes first.
    ///
    /// # Note
    ///
    /// We return `&mut T` from `&self`. If this bothers you, you're in the wrong
    /// module.
    pub fn get(&self) -> &mut T {
        self.data.get_mut()
    }

    /// Modifies the data using a closure.
    ///
    /// This is a slightly more ergonomic way to cause undefined behavior. The
    /// closure receives a mutable reference and can do whatever it wants. No
    /// synchronization is performed before, during, or after.
    ///
    /// # Arguments
    ///
    /// * `f` - A closure that takes `&mut T`. It will be called exactly once,
    ///   probably.
    ///
    /// # Example
    ///
    /// ```rust
    /// use corroded::race::RaceCondition;
    ///
    /// let race = RaceCondition::new(vec![1, 2, 3]);
    /// race.modify(|v| v.push(4));
    /// race.modify(|v| v.push(5));
    /// // The vec now contains [1, 2, 3, 4, 5]
    /// // Or [1, 2, 3, 5, 4] if this was multithreaded and we got unlucky
    /// // Or the heap is corrupted. Always a possibility.
    /// ```
    pub fn modify<F: FnOnce(&mut T)>(&self, f: F) {
        f(self.data.get_mut());
    }
}
