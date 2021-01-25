#![no_std]
#![no_main]

use core::sync::atomic::{AtomicUsize, Ordering};
use cortex_m_rt::entry;
use defmt::*;
use defmt_rtt as _; // global logger
use panic_probe as _;

// this adds boot2 to the .boot_loader section
// linker script then places that at start of flash.
use rp2040_boot2 as _;
use rp2040_pac as pac;

#[defmt::timestamp]
fn timestamp() -> u64 {
    static COUNT: AtomicUsize = AtomicUsize::new(0);
    // NOTE(no-CAS) `timestamps` runs with interrupts disabled
    let n = COUNT.load(Ordering::Relaxed);
    COUNT.store(n + 1, Ordering::Relaxed);
    n as u64
}

#[entry]
fn main() -> ! {
    info!("Hello World!");

    let p = pac::Peripherals::take().unwrap();

    loop {
        info!("on!");
        p.IO_BANK0.gpio25_ctrl.write(|w| {
            w.oeover().enable();
            w.outover().high();
            w
        });

        cortex_m::asm::delay(64_000_000);

        info!("off!");
        p.IO_BANK0.gpio25_ctrl.write(|w| {
            w.oeover().enable();
            w.outover().low();
            w
        });

        cortex_m::asm::delay(64_000_000);
    }
}
