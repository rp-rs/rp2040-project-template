use rp2040_pac as pac;

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

pub struct Resets {
    inner: pac::RESETS,
}

impl Resets {
    pub fn new(inner: pac::RESETS) -> Self {
        Self { inner }
    }

    pub fn reset(&self, bits: u32) {
        self.inner.reset.write(|w| unsafe { w.bits(bits) })
    }

    pub fn unreset_wait(&self, bits: u32) {
        // TODO use the "atomic clear" register version
        self.inner
            .reset
            .modify(|r, w| unsafe { w.bits(r.bits() & !bits) });
        while ((!self.inner.reset_done.read().bits()) & bits) != 0 {}
    }
}
