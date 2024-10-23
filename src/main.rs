#![no_std]
#![no_main]

extern crate cortex_m;

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Setting Up");
    loop {
        rprintln!("Hey?")
    }
}
