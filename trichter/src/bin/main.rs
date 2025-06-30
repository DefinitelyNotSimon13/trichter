#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use core::sync::atomic::{AtomicU32, Ordering};

use ag_lcd::LcdDisplay;
use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Delay, Duration, Instant, Timer};
use esp_hal::analog::adc::{Adc, AdcCalCurve, AdcCalSource, AdcConfig, AdcPin, Attenuation};
use esp_hal::efuse::Efuse;
use esp_hal::gpio::{Event, Input, InputConfig, Level, OutputConfig, Pull};
use esp_hal::i2c::master::I2c;
use esp_hal::peripherals::ADC1;
use esp_hal::timer::systimer::SystemTimer;
use esp_hal::timer::timg::TimerGroup;
use esp_hal::xtensa_lx::interrupt;
use esp_hal::{clock::CpuClock, gpio::Output};
use esp_hal::{delay, efuse, i2c, Blocking};
use esp_wifi::ble::controller::BleConnector;
use trouble_host::prelude::ExternalController;
use {esp_backtrace as _, esp_println as _};

extern crate alloc;

static PULSE_COUNT: AtomicU32 = AtomicU32::new(0);

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    // generator version: 0.4.0

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 64 * 1024);
    // COEX needs more RAM - so we've added some more
    esp_alloc::heap_allocator!(#[unsafe(link_section = ".dram2_uninit")] size: 64 * 1024);

    let timer0 = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);

    info!("Embassy initialized!");

    let rng = esp_hal::rng::Rng::new(peripherals.RNG);
    let timer1 = TimerGroup::new(peripherals.TIMG0);
    // let wifi_init = esp_wifi::init(timer1.timer0, rng, peripherals.RADIO_CLK)
    //     .expect("Failed to initialize WIFI/BLE controller");
    // let (mut _wifi_controller, _interfaces) = esp_wifi::wifi::new(&wifi_init, peripherals.WIFI)
    //     .expect("Failed to initialize WIFI controller");
    // // find more examples https://github.com/embassy-rs/trouble/tree/main/examples/esp32
    // let transport = BleConnector::new(&wifi_init, peripherals.BT);
    // let ble_controller = ExternalController::<_, 20>::new(transport);

    // TODO: Spawn some tasks
    let _ = spawner;

    let mut led_red = Output::new(peripherals.GPIO46, Level::Low, OutputConfig::default());
    let mut led_green = Output::new(peripherals.GPIO0, Level::Low, OutputConfig::default());
    let mut led_blue = Output::new(peripherals.GPIO45, Level::Low, OutputConfig::default());

    led_red.set_high();
    led_green.set_high();
    led_blue.set_high();

    let mut inp = Input::new(peripherals.GPIO7, InputConfig::default());

    // let mut adc_config = AdcConfig::new();
    // let mut sensor_in: AdcPin<_, _> = adc_config.enable_pin(peripherals.GPIO1, Attenuation::_11dB);
    // let mut adc1 = Adc::<_, _>::new(peripherals.ADC1, adc_config);
    //
    // let mut is_risen = false;
    // let mut edges = 0;
    // let mut start = Instant::now();
    // let mut is_running = false;
    //

    let rs = Output::new(peripherals.GPIO4, Level::Low, OutputConfig::default());
    let rw = Output::new(peripherals.GPIO8, Level::Low, OutputConfig::default());
    let en = Output::new(peripherals.GPIO9, Level::Low, OutputConfig::default());

    let d4 = Output::new(peripherals.GPIO10, Level::Low, OutputConfig::default());
    let d5 = Output::new(peripherals.GPIO11, Level::Low, OutputConfig::default());
    let d6 = Output::new(peripherals.GPIO12, Level::Low, OutputConfig::default());
    let d7 = Output::new(peripherals.GPIO13, Level::Low, OutputConfig::default());

    let mut lcd = LcdDisplay::new(rs, en, delay::Delay::new())
        .with_half_bus(d4, d5, d6, d7)
        .with_display(ag_lcd::Display::On)
        .with_blink(ag_lcd::Blink::On)
        .with_cursor(ag_lcd::Cursor::On)
        .build();

    let mut num = 0;
    let mut start = Instant::now();
    let mut last_imp = Instant::now();
    let mut running = false;
    inp.listen(Event::RisingEdge);
    loop {
        if !running && inp.is_interrupt_set() {
            start = Instant::now();
            last_imp = Instant::now();
            running = true;
            inp.clear_interrupt();
        }

        if running && inp.is_interrupt_set() {
            info!("Got int");
            last_imp = Instant::now();
            inp.clear_interrupt();
        }

        if running && last_imp.elapsed() > Duration::from_secs(1) {
            info!("Done, took {:#?}ms", start.elapsed().as_millis());
            running = false;
            inp.clear_interrupt();
        }

        // if start.elapsed() > Duration::from_secs(10) {
        //     info!("10 Second elapsed - Got a total of {} impulses", num);
        //     info!("FlowRate: {}L/min", (num * 6) as f32 * 6.6);
        //     break;
        // }

        // edges = 0;
        // for i in 0..10_000 {
        //     let val = adc1.read_blocking(&mut sensor_in);
        //     if !is_risen && val > 3000 {
        //         is_risen = true;
        //         edges += 1;
        //     } else if is_risen && val < 3000 {
        //         is_risen = false;
        //     }
        //     Timer::after_nanos(2).await;
        // }
        //
        // info!("Edges: {}", edges);
        // if !is_running && edges > 20 {
        //     info!("Start");
        //     is_running = true;
        //     start = Instant::now();
        // }
        //
        // if is_running && edges <= 20 {
        //     let duration = Instant::now() - start;
        //     info!("Ran for {} ms", duration.as_millis());
        //     is_running = false;
        // }
    }

    // trichter::ble::run(ble_controller).await;

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-beta.1/examples/src/bin
}

fn scan(i2c: &mut I2c<'_, Blocking>) {
    for addr in 1..=127 {
        info!("Scanning Adress {}", addr as u8);

        let res = i2c.read(addr as u8, &mut [0]);

        match res {
            Ok(_) => info!("Device found at Address {}", addr as u8),
            Err(_) => info!("No Device found"),
        }
    }
}
