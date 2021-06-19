use super::init_value;

/* TLS + REF CELL */

use std::borrow::BorrowMut;
use std::cell::RefCell;
thread_local!(static TLS_COUNTER: RefCell<u32> = RefCell::new(init_value()));
thread_local!(static TLS_COUNTER_2: RefCell<u32> = RefCell::new(init_value()));
thread_local!(static TLS_COUNTER_3: RefCell<u32> = RefCell::new(init_value()));

pub fn tls(n: usize) {
    for _ in 0..n {
        TLS_COUNTER.with(|counter| {
            *counter.borrow_mut() += 1;
        });
    }
}

pub fn tls_2(n: usize) {
    for _ in 0..n {
        TLS_COUNTER.with(|counter| {
            *counter.borrow_mut() += 1;
        });
        TLS_COUNTER_2.with(|counter| {
            *counter.borrow_mut() += 1;
        });
    }
}

pub fn tls_3(n: usize) {
    for _ in 0..n {
        TLS_COUNTER.with(|counter| {
            *counter.borrow_mut() += 1;
        });
        TLS_COUNTER_2.with(|counter| {
            *counter.borrow_mut() += 1;
        });
        TLS_COUNTER_3.with(|counter| {
            *counter.borrow_mut() += 1;
        });
    }
}

/* STD ONCE + Mutex */

use std::sync::{Mutex, Once};
static mut STD_ONCE_COUNTER: Option<Mutex<u32>> = None;
static mut STD_ONCE_COUNTER_2: Option<Mutex<u32>> = None;
static mut STD_ONCE_COUNTER_3: Option<Mutex<u32>> = None;
static INIT: Once = Once::new();
static INIT_2: Once = Once::new();
static INIT_3: Once = Once::new();
fn read_std_once<'a>() -> &'a Mutex<u32> {
    INIT.call_once(|| {
        // Since this access is inside a call_once, it is safe
        unsafe {
            *STD_ONCE_COUNTER.borrow_mut() = Some(Mutex::new(init_value()));
        }
    });
    // As long as this function is the only place with access to the static variable,
    // giving out read-only borrow here is safe because it is guaranteed no more mutable references will exist at this point or in the future.
    unsafe { STD_ONCE_COUNTER.as_ref().unwrap() }
}
fn read_std_once_2<'a>() -> &'a Mutex<u32> {
    INIT_2.call_once(|| unsafe {
        *STD_ONCE_COUNTER_2.borrow_mut() = Some(Mutex::new(init_value()));
    });
    unsafe { STD_ONCE_COUNTER_2.as_ref().unwrap() }
}
fn read_std_once_3<'a>() -> &'a Mutex<u32> {
    INIT_3.call_once(|| unsafe {
        *STD_ONCE_COUNTER_3.borrow_mut() = Some(Mutex::new(init_value()));
    });
    unsafe { STD_ONCE_COUNTER_3.as_ref().unwrap() }
}

pub fn std_once(n: usize) {
    for _ in 0..n {
        *read_std_once().lock().unwrap() += 1;
    }
}
pub fn std_once_2(n: usize) {
    for _ in 0..n {
        *read_std_once().lock().unwrap() += 1;
        *read_std_once_2().lock().unwrap() += 1;
    }
}
pub fn std_once_3(n: usize) {
    for _ in 0..n {
        *read_std_once().lock().unwrap() += 1;
        *read_std_once_2().lock().unwrap() += 1;
        *read_std_once_3().lock().unwrap() += 1;
    }
}

/* LAZY STATIC + MUTEX */

lazy_static! {
    static ref LAZY_STATIC_COUNTER: Mutex<u32> = Mutex::new(init_value());
}
lazy_static! {
    static ref LAZY_STATIC_COUNTER_2: Mutex<u32> = Mutex::new(init_value());
}
lazy_static! {
    static ref LAZY_STATIC_COUNTER_3: Mutex<u32> = Mutex::new(init_value());
}

pub fn lazy_static(n: usize) {
    for _ in 0..n {
        *LAZY_STATIC_COUNTER.lock().unwrap() += 1;
    }
}
pub fn lazy_static_2(n: usize) {
    for _ in 0..n {
        *LAZY_STATIC_COUNTER.lock().unwrap() += 1;
        *LAZY_STATIC_COUNTER_2.lock().unwrap() += 1;
    }
}
pub fn lazy_static_3(n: usize) {
    for _ in 0..n {
        *LAZY_STATIC_COUNTER.lock().unwrap() += 1;
        *LAZY_STATIC_COUNTER_2.lock().unwrap() += 1;
        *LAZY_STATIC_COUNTER_3.lock().unwrap() += 1;
    }
}

/* ONCE CELL + ARC + MUTEX */

use once_cell::sync::Lazy;
static ONCE_CELL_COUNTER: Lazy<Mutex<u32>> = Lazy::new(|| Mutex::new(init_value()));
static ONCE_CELL_COUNTER_2: Lazy<Mutex<u32>> = Lazy::new(|| Mutex::new(init_value()));
static ONCE_CELL_COUNTER_3: Lazy<Mutex<u32>> = Lazy::new(|| Mutex::new(init_value()));

pub fn once_cell(n: usize) {
    for _ in 0..n {
        *ONCE_CELL_COUNTER.lock().unwrap() += 1;
    }
}
pub fn once_cell_2(n: usize) {
    for _ in 0..n {
        *ONCE_CELL_COUNTER.lock().unwrap() += 1;
        *ONCE_CELL_COUNTER_2.lock().unwrap() += 1;
    }
}
pub fn once_cell_3(n: usize) {
    for _ in 0..n {
        *ONCE_CELL_COUNTER.lock().unwrap() += 1;
        *ONCE_CELL_COUNTER_2.lock().unwrap() += 1;
        *ONCE_CELL_COUNTER_3.lock().unwrap() += 1;
    }
}

/* ATOMIC */
// Note: Without initialization, which would just be a simple store anyway

use std::sync::atomic::{AtomicU32, Ordering};
static ATOMIC_COUNTER: AtomicU32 = AtomicU32::new(0);
static ATOMIC_COUNTER_2: AtomicU32 = AtomicU32::new(0);
static ATOMIC_COUNTER_3: AtomicU32 = AtomicU32::new(0);

pub fn atomic(n: usize) {
    for _ in 0..n {
        ATOMIC_COUNTER.fetch_add(1, Ordering::Relaxed);
    }
}
pub fn atomic_2(n: usize) {
    for _ in 0..n {
        ATOMIC_COUNTER.fetch_add(1, Ordering::Relaxed);
        ATOMIC_COUNTER_2.fetch_add(1, Ordering::Relaxed);
    }
}
pub fn atomic_3(n: usize) {
    for _ in 0..n {
        ATOMIC_COUNTER.fetch_add(1, Ordering::Relaxed);
        ATOMIC_COUNTER_2.fetch_add(1, Ordering::Relaxed);
        ATOMIC_COUNTER_3.fetch_add(1, Ordering::Relaxed);
    }
}

pub fn atomic_seq(n: usize) {
    for _ in 0..n {
        ATOMIC_COUNTER.fetch_add(1, Ordering::SeqCst);
    }
}
pub fn atomic_seq_2(n: usize) {
    for _ in 0..n {
        ATOMIC_COUNTER.fetch_add(1, Ordering::SeqCst);
        ATOMIC_COUNTER_2.fetch_add(1, Ordering::SeqCst);
    }
}
pub fn atomic_seq_3(n: usize) {
    for _ in 0..n {
        ATOMIC_COUNTER.fetch_add(1, Ordering::SeqCst);
        ATOMIC_COUNTER_2.fetch_add(1, Ordering::SeqCst);
        ATOMIC_COUNTER_3.fetch_add(1, Ordering::SeqCst);
    }
}
