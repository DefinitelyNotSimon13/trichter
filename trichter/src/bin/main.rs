#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::timer::systimer::SystemTimer;
use trichter::{sensor::SensorDriver, system::System};
use {esp_backtrace as _, esp_println as _};

extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    let peripherals = System::init_peripherals();

    let timer0 = SystemTimer::new(peripherals.SYSTIMER);
    let mut system = System::builder(timer0.alarm0)
        .with_sensor(peripherals.GPIO7)
        .build();

    let _ = spawner;

    let mut sensor = system.sensor.take().expect("sensor was not initialized");
    let duration = Duration::from_secs(10);
    loop {
        info!("Measuring flow for 10 seconds...");
        let pulses = sensor.measure_pulses(duration).await;
        let flow_rate = SensorDriver::pulses_to_flow(pulses, duration.as_secs() as f32);
        info!(
            "Measured a flow rate of {}L/min in the last {} seconds",
            flow_rate,
            duration.as_secs()
        );

        Timer::after_secs(1).await;
    }
}
