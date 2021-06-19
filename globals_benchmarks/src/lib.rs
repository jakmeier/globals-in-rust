#[macro_use]
extern crate lazy_static;

pub mod mutable_globals;
pub mod read_only_globals;

fn init_value() -> u32 {
    match std::env::var("INIT") {
        Ok(val) => val.parse().unwrap_or(0),
        Err(_) => 0,
    }
}
