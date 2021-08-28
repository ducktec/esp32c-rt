#![no_std]
#![no_main]

use esp32c_rt::entry;

// Provides a necessary panic handler
#[allow(unused_imports)]
use panic_halt;

#[entry]
fn main() -> ! {
    // do something here
    loop {}
}
