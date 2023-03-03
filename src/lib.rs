#![no_std]
#![allow(stable_features)]

#[cfg(feature = "std")]
extern crate std;

pub mod communication_pipe;
pub mod i2c;
pub mod interval_timer;
pub mod spi;

pub mod video_dma_control;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
