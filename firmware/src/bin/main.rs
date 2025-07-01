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
    sensor::{SensorHandler, SessionResult, StartupWindow, RESULTS},
    system::System,
    wifi::{connect_to_hotspot, create_wifi_controller},
};
use {esp_backtrace as _, esp_println as _};

extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    let peripherals = System::init_peripherals();
    esp_alloc::heap_allocator!(size: 72 * 1024);

    let rng = Rng::new(peripherals.RNG);
    let timer1 = TimerGroup::new(peripherals.TIMG0);
    let wifi_init = &*mk_static!(
        EspWifiController<'static>,
        init(timer1.timer0, rng, peripherals.RADIO_CLK).unwrap()
    );
    let timer0 = SystemTimer::new(peripherals.SYSTIMER);
    let mut system = System::builder(timer0.alarm0)
        .with_sensor(peripherals.GPIO9)
        .with_wifi(wifi_init, peripherals.WIFI, peripherals.BT)
        // .with_lcd(
        //     peripherals.GPIO47,
        //     peripherals.GPIO38,
        //     peripherals.GPIO18,
        //     peripherals.GPIO17,
        //     peripherals.GPIO10,
        //     peripherals.GPIO9,
        // )
        .build();

    info!("Hello!");
    let wifi = system.wifi.take().unwrap();

    // let mut lcd_driver = system.lcd.take().expect("lcd was not initialized");
    //
    // lcd_driver.lcd.display_off();
    //
    // lcd_driver.lcd.print("Hello, there!");

    // let mut lcd = Lcd4Bit::new(
    //     peripherals.GPIO47,
    //     peripherals.GPIO38,
    //     peripherals.GPIO18,
    //     peripherals.GPIO17,
    //     peripherals.GPIO10,
    //     peripherals.GPIO9,
    //     Delay::new(),
    //     0,
    // );
    // lcd.init();
    // loop {
    //     lcd.gotoxy(1, 1);
    //     lcd.putc('H');
    // }
    // info!("Should print!");

    spawner.spawn(connect_to_hotspot(wifi, rng, spawner)).ok();

    let mut sensor = system.sensor.take().expect("sensor was not initialized");
    let duration = Duration::from_secs(10);
    loop {
        info!("Waiting for session to start...");

        let res = sensor
            .mesaure_session(StartupWindow::default(), Duration::from_millis(100))
            .await;
        info!(
            "Measured for {}ms with a flow rate of {}L/min",
            res.rate,
            res.duration.as_millis()
        );
        RESULTS.lock().await.push(res);
    }
}

fn scan(i2c: &mut I2c<'_, Blocking>) {
    for addr in 1..=127 {
        info!("Scanning Adress {}", addr as u8);

        let res = i2c.read(addr as u8, &mut [0]);

        match res {
            Ok(_) => {
                info!("Device found at Address {}", addr as u8);
                break;
            }
            Err(_) => info!("No Device found"),
        }
    }
}
