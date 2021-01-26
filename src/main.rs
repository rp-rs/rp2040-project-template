#![no_std]
#![no_main]
#![feature(asm)]

use core::sync::atomic::{AtomicUsize, Ordering};
use cortex_m_rt::entry;
use defmt::*;
use defmt_rtt as _;
use panic_probe as _;
use rp2040_pac as pac;

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

mod reset_bits {
    pub const ALL: u32 = 0x01ffffff;
    pub const USBCTRL: u32 = 0x01000000;
    pub const UART1: u32 = 0x00800000;
    pub const UART0: u32 = 0x00400000;
    pub const TIMER: u32 = 0x00200000;
    pub const TBMAN: u32 = 0x00100000;
    pub const SYSINFO: u32 = 0x00080000;
    pub const SYSCFG: u32 = 0x00040000;
    pub const SPI1: u32 = 0x00020000;
    pub const SPI0: u32 = 0x00010000;
    pub const RTC: u32 = 0x00008000;
    pub const PWM: u32 = 0x00004000;
    pub const PLL_USB: u32 = 0x00002000;
    pub const PLL_SYS: u32 = 0x00001000;
    pub const PIO1: u32 = 0x00000800;
    pub const PIO0: u32 = 0x00000400;
    pub const PADS_QSPI: u32 = 0x00000200;
    pub const PADS_BANK0: u32 = 0x00000100;
    pub const JTAG: u32 = 0x00000080;
    pub const IO_QSPI: u32 = 0x00000040;
    pub const IO_BANK0: u32 = 0x00000020;
    pub const I2C1: u32 = 0x00000010;
    pub const I2C0: u32 = 0x00000008;
    pub const DMA: u32 = 0x00000004;
    pub const BUSCTRL: u32 = 0x00000002;
    pub const ADC: u32 = 0x00000001;
}

struct Resets {
    inner: pac::RESETS,
}

impl Resets {
    fn new(inner: pac::RESETS) -> Self {
        Self { inner }
    }

    fn reset(&self, bits: u32) {
        self.inner.reset.write(|w| unsafe { w.bits(bits) })
    }

    fn unreset_wait(&self, bits: u32) {
        // TODO use the "atomic clear" register version
        self.inner
            .reset
            .modify(|r, w| unsafe { w.bits(r.bits() & !bits) });
        while ((!self.inner.reset_done.read().bits()) & bits) != 0 {}
    }
}

fn setup_chip(resets: pac::RESETS) {
    // Now reset all the peripherals, except QSPI and XIP (we're using those
    // to execute from external flash!)

    let resets = Resets::new(resets);

    // Reset everything except:
    // - QSPI (we're using it to run this code!)
    // - PLLs (it may be suicide if that's what's clocking us)
    resets.reset(
        !(reset_bits::IO_QSPI | reset_bits::PADS_QSPI | reset_bits::PLL_SYS | reset_bits::PLL_USB),
    );

    resets.unreset_wait(
        reset_bits::ALL
            & !(reset_bits::ADC
                | reset_bits::RTC
                | reset_bits::SPI0
                | reset_bits::SPI1
                | reset_bits::UART0
                | reset_bits::UART1
                | reset_bits::USBCTRL),
    );
}

#[entry]
fn main() -> ! {
    info!("Hello World!");

    let p = pac::Peripherals::take().unwrap();

    setup_chip(p.RESETS);

    loop {
        info!("on!");
        p.IO_BANK0.gpio25_ctrl.write(|w| {
            w.oeover().enable();
            w.outover().high();
            w
        });

        cortex_m::asm::delay(1_000_000);

        info!("off!");
        p.IO_BANK0.gpio25_ctrl.write(|w| {
            w.oeover().enable();
            w.outover().low();
            w
        });

        cortex_m::asm::delay(1_000_000);
    }
}
