#![no_std]

use esp_hal::gpio::{Output, OutputConfig, OutputPin};

pub mod ble;
pub mod lcd;
pub mod sensor;
pub mod system;
pub mod wifi;

pub fn output_from_pin<'p>(p: impl OutputPin + 'p) -> Output<'p> {
    Output::new(p, esp_hal::gpio::Level::Low, OutputConfig::default())
}
