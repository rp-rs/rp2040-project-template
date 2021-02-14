#![no_std]
#![no_main]
#![feature(asm)]

use core::sync::atomic::{AtomicUsize, Ordering};
use cortex_m_rt::entry;
use defmt::*;
use defmt_rtt as _;
use pac::{watchdog, xosc};
use panic_probe as _;
use rp2040_pac as pac;

mod pll;
mod resets;

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER;

#[defmt::timestamp]
fn timestamp() -> u64 {
    static COUNT: AtomicUsize = AtomicUsize::new(0);
    // NOTE(no-CAS) `timestamps` runs with interrupts disabled
    let n = COUNT.load(Ordering::Relaxed);
    COUNT.store(n + 1, Ordering::Relaxed);
    n as u64
}

fn init(
    resets: pac::RESETS,
    watchdog: pac::WATCHDOG,
    clocks: pac::CLOCKS,
    xosc: pac::XOSC,
    pll_sys: pac::PLL_SYS,
    pll_usb: pac::PLL_USB,
) {
    // Now reset all the peripherals, except QSPI and XIP (we're using those
    // to execute from external flash!)

    let resets = resets::Resets::new(resets);

    // Reset everything except:
    // - QSPI (we're using it to run this code!)
    // - PLLs (it may be suicide if that's what's clocking us)
    resets.reset(!(resets::IO_QSPI | resets::PADS_QSPI | resets::PLL_SYS | resets::PLL_USB));

    resets.unreset_wait(
        resets::ALL
            & !(resets::ADC
                | resets::RTC
                | resets::SPI0
                | resets::SPI1
                | resets::UART0
                | resets::UART1
                | resets::USBCTRL),
    );

    // xosc 12 mhz
    watchdog
        .tick
        .write(|w| unsafe { w.cycles().bits(XOSC_MHZ as u16).enable().set_bit() });

    clocks.clk_sys_resus_ctrl.write(|w| unsafe { w.bits(0) });

    // Enable XOSC
    // TODO extract to HAL module
    const XOSC_MHZ: u32 = 12;
    xosc.ctrl.write(|w| w.freq_range()._1_15mhz());
    let startup_delay = (((XOSC_MHZ * 1_000_000) / 1000) + 128) / 256;
    xosc.startup
        .write(|w| unsafe { w.delay().bits(startup_delay as u16) });
    xosc.ctrl
        .write(|w| w.freq_range()._1_15mhz().enable().enable());
    while !xosc.status.read().stable().bit_is_set() {}

    // Before we touch PLLs, switch sys and ref cleanly away from their aux sources.
    clocks.clk_sys_ctrl.modify(|_, w| w.src().clk_ref());
    while clocks.clk_sys_selected.read().bits() != 1 {}
    clocks.clk_ref_ctrl.modify(|_, w| w.src().rosc_clksrc_ph());
    while clocks.clk_ref_selected.read().bits() != 1 {}

    resets.reset(resets::PLL_SYS | resets::PLL_USB);
    resets.unreset_wait(resets::PLL_SYS | resets::PLL_USB);

    pll::PLL::new(pll_sys).configure(1, 1500_000_000, 6, 2);
    pll::PLL::new(pll_usb).configure(1, 480_000_000, 5, 2);
}

#[entry]
fn main() -> ! {
    info!("Hello World!");

    let p = pac::Peripherals::take().unwrap();

    init(p.RESETS, p.WATCHDOG, p.CLOCKS, p.XOSC, p.PLL_SYS, p.PLL_USB);

    let led_pin = 25;

    loop {
        info!("on!");
        p.IO_BANK0.gpio[led_pin].gpio_ctrl.write(|w| {
            w.oeover().enable();
            w.outover().high();
            w
        });

        cortex_m::asm::delay(1_000_000);

        info!("off!");
        p.IO_BANK0.gpio[led_pin].gpio_ctrl.write(|w| {
            w.oeover().enable();
            w.outover().low();
            w
        });

        cortex_m::asm::delay(1_000_000);
    }
}
