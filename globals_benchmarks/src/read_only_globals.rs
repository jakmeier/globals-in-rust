use super::init_value;

/* TLS */

thread_local!(static TLS_COUNTER: u32 = init_value());
thread_local!(static TLS_COUNTER_2: u32 = init_value());
thread_local!(static TLS_COUNTER_3: u32 = init_value());

pub fn tls(n: usize) -> u32 {
    let mut total = 0;
    for _ in 0..n {
        TLS_COUNTER.with(|counter| {
            total += counter;
        });
    }
    total
}

pub fn tls_2(n: usize) -> u32 {
    let mut total = 0;
    for _ in 0..n {
        TLS_COUNTER.with(|counter| {
            total += counter;
        });
        TLS_COUNTER_2.with(|counter| {
            total += counter;
        });
    }
    total
}

pub fn tls_3(n: usize) -> u32 {
    let mut total = 0;
    for _ in 0..n {
        TLS_COUNTER.with(|counter| {
            total += counter;
        });
        TLS_COUNTER_2.with(|counter| {
            total += counter;
        });
        TLS_COUNTER_3.with(|counter| {
            total += counter;
        });
    }
    total
}

/* STD ONCE */

use std::sync::Once;
static mut STD_ONCE_COUNTER: Option<u32> = None;
static mut STD_ONCE_COUNTER_2: Option<u32> = None;
static mut STD_ONCE_COUNTER_3: Option<u32> = None;
static INIT: Once = Once::new();
static INIT_2: Once = Once::new();
static INIT_3: Once = Once::new();
fn read_std_once<'a>() -> &'a u32 {
    INIT.call_once(|| {
        // Since this access is inside a call_once, it is safe
        unsafe {
            STD_ONCE_COUNTER = Some(init_value());
        }
    });
    // As long as this function is the only place with access to the static variable,
    // giving out read-only borrow here is safe because it is guaranteed no more mutable references will exist at this point or in the future.
    unsafe { STD_ONCE_COUNTER.as_ref().unwrap() }
}
fn read_std_once_2<'a>() -> &'a u32 {
    INIT_2.call_once(|| unsafe {
        STD_ONCE_COUNTER_2 = Some(init_value());
    });
    unsafe { STD_ONCE_COUNTER_2.as_ref().unwrap() }
}
fn read_std_once_3<'a>() -> &'a u32 {
    INIT_3.call_once(|| unsafe {
        STD_ONCE_COUNTER_3 = Some(init_value());
    });
    unsafe { STD_ONCE_COUNTER_3.as_ref().unwrap() }
}

pub fn std_once(n: usize) -> u32 {
    let mut total = 0;
    for _ in 0..n {
        total += *read_std_once();
    }
    total
}
pub fn std_once_2(n: usize) -> u32 {
    let mut total = 0;
    for _ in 0..n {
        total += *read_std_once();
        total += *read_std_once_2();
    }
    total
}
pub fn std_once_3(n: usize) -> u32 {
    let mut total = 0;
    for _ in 0..n {
        total += *read_std_once();
        total += *read_std_once_2();
        total += *read_std_once_3();
    }
    total
}

/* LAZY STATIC + MUTEX */

lazy_static! {
    static ref LAZY_STATIC_COUNTER: u32 = init_value();
}
lazy_static! {
    static ref LAZY_STATIC_COUNTER_2: u32 = init_value();
}
lazy_static! {
    static ref LAZY_STATIC_COUNTER_3: u32 = init_value();
}

pub fn lazy_static(n: usize) -> u32 {
    let mut total = 0;
    for _ in 0..n {
        total += *LAZY_STATIC_COUNTER;
    }
    total
}
pub fn lazy_static_2(n: usize) -> u32 {
    let mut total = 0;
    for _ in 0..n {
        total += *LAZY_STATIC_COUNTER;
        total += *LAZY_STATIC_COUNTER_2;
    }
    total
}
pub fn lazy_static_3(n: usize) -> u32 {
    let mut total = 0;
    for _ in 0..n {
        total += *LAZY_STATIC_COUNTER;
        total += *LAZY_STATIC_COUNTER_2;
        total += *LAZY_STATIC_COUNTER_3;
    }
    total
}

/* ONCE CELL + ARC + MUTEX */

use once_cell::sync::Lazy;
static ONCE_CELL_COUNTER: Lazy<u32> = Lazy::new(|| init_value());
static ONCE_CELL_COUNTER_2: Lazy<u32> = Lazy::new(|| init_value());
static ONCE_CELL_COUNTER_3: Lazy<u32> = Lazy::new(|| init_value());

pub fn once_cell(n: usize) -> u32 {
    let mut total = 0;
    for _ in 0..n {
        total += *ONCE_CELL_COUNTER;
    }
    total
}
pub fn once_cell_2(n: usize) -> u32 {
    let mut total = 0;
    for _ in 0..n {
        total += *ONCE_CELL_COUNTER;
        total += *ONCE_CELL_COUNTER_2;
    }
    total
}
pub fn once_cell_3(n: usize) -> u32 {
    let mut total = 0;
    for _ in 0..n {
        total += *ONCE_CELL_COUNTER;
        total += *ONCE_CELL_COUNTER_2;
        total += *ONCE_CELL_COUNTER_3;
    }
    total
}

/* ATOMIC */
// Note: Without initialization, which would just be a simple store anyway

use std::sync::atomic::{AtomicU32, Ordering};
static ATOMIC_COUNTER: AtomicU32 = AtomicU32::new(0);
static ATOMIC_COUNTER_2: AtomicU32 = AtomicU32::new(0);
static ATOMIC_COUNTER_3: AtomicU32 = AtomicU32::new(0);

pub fn atomic(n: usize) -> u32 {
    let mut total = 0;
    for _ in 0..n {
        total += ATOMIC_COUNTER.load(Ordering::Relaxed);
    }
    total
}
pub fn atomic_2(n: usize) -> u32 {
    let mut total = 0;
    for _ in 0..n {
        total += ATOMIC_COUNTER.load(Ordering::Relaxed);
        total += ATOMIC_COUNTER_2.load(Ordering::Relaxed);
    }
    total
}
pub fn atomic_3(n: usize) -> u32 {
    let mut total = 0;
    for _ in 0..n {
        total += ATOMIC_COUNTER.load(Ordering::Relaxed);
        total += ATOMIC_COUNTER_2.load(Ordering::Relaxed);
        total += ATOMIC_COUNTER_3.load(Ordering::Relaxed);
    }
    total
}

pub fn atomic_seq(n: usize) -> u32 {
    let mut total = 0;
    for _ in 0..n {
        total += ATOMIC_COUNTER.load(Ordering::SeqCst);
    }
    total
}
pub fn atomic_seq_2(n: usize) -> u32 {
    let mut total = 0;
    for _ in 0..n {
        total += ATOMIC_COUNTER.load(Ordering::SeqCst);
        total += ATOMIC_COUNTER_2.load(Ordering::SeqCst);
    }
    total
}
pub fn atomic_seq_3(n: usize) -> u32 {
    let mut total = 0;
    for _ in 0..n {
        total += ATOMIC_COUNTER.load(Ordering::SeqCst);
        total += ATOMIC_COUNTER_2.load(Ordering::SeqCst);
        total += ATOMIC_COUNTER_3.load(Ordering::SeqCst);
    }
    total
}
