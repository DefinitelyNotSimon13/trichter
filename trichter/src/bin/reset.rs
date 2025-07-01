#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use ag_lcd::Cursor;
use bt_hci::{cmd::info, uuid::appearance::personal_mobility_device};
use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_alloc as _;
use esp_hal::{
    delay::Delay,
    i2c::master::{Config, I2c},
    peripherals,
    rng::Rng,
    timer::{systimer::SystemTimer, timg::TimerGroup},
    Blocking,
};
use esp_wifi::{init, EspWifiController};
use trichter::{
    driver::lcd::Lcd4Bit,
    mk_static,
    sensor::{SensorHandler, StartupWindow},
    system::System,
    wifi::create_wifi_controller,
};
use {esp_backtrace as _, esp_println as _};

extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    let peripherals = System::init_peripherals();
}
