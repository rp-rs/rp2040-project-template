//! # Library Code for the rp2040-project-template application
//!
//! Code in this file can be tested on the host with `cargo
//! test --lib --target=<your PC's target>`

#[cfg_attr(not(test), no_std)]

/// LED on-period and off-period, in milliseconds
pub const LED_DELAY_MS: u32 = 500;

#[cfg(test)]
mod test {
	#[test]
	fn it_works() {
		let sum = 2 + 2;
		assert_eq!(4, sum);
	}
}

// End of file