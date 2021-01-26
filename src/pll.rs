use core::ops::Deref;
use defmt::{assert, *};

use rp2040_pac as pac;

const XOSC_MHZ: u32 = 12;

pub struct PLL<T: Instance> {
    inner: T,
}

impl<T: Instance> PLL<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }

    pub fn configure(&mut self, refdiv: u32, vco_freq: u32, post_div1: u8, post_div2: u8) {
        let p = &self.inner;

        // Power off in case it's already running
        p.pwr.reset();
        p.fbdiv_int.reset();

        let ref_mhz = XOSC_MHZ / refdiv;
        p.cs.write(|w| unsafe { w.bits(ref_mhz as _) });

        let fbdiv = vco_freq / (ref_mhz * 1_000_000);
        assert!(fbdiv >= 16 && fbdiv <= 520);
        assert!((post_div1 >= 1 && post_div1 <= 7) && (post_div2 >= 1 && post_div2 <= 7));
        assert!(post_div2 <= post_div1);
        assert!(ref_mhz <= (vco_freq / 16));

        p.fbdiv_int.write(|w| unsafe { w.bits(fbdiv) });

        p.pwr.modify(|_, w| w.pd().clear_bit().vcopd().clear_bit());

        while !p.cs.read().lock().bits() {}

        p.prim.write(|w| unsafe {
            w.postdiv1().bits(post_div1);
            w.postdiv2().bits(post_div2);
            w
        });

        p.pwr.modify(|_, w| w.postdivpd().clear_bit());
    }
}

mod sealed {
    use rp2040_pac as pac;

    pub trait Instance {}
    impl Instance for pac::PLL_SYS {}
    impl Instance for pac::PLL_USB {}
}

pub trait Instance: Deref<Target = pac::pll_sys::RegisterBlock> {}
impl Instance for pac::PLL_SYS {}
impl Instance for pac::PLL_USB {}
