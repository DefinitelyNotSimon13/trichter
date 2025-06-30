use defmt::{debug, info};
use esp_hal::{
    clock::CpuClock,
    gpio::{InputPin, OutputPin},
    peripherals::{self, Peripherals},
    timer::systimer::Alarm,
};
use esp_wifi::EspWifiController;

use crate::{lcd::LcdDriver, sensor::SensorDriver, wifi::WifiManager};

pub struct System<'a> {
    pub wifi: Option<WifiManager<'a>>,
    pub lcd: Option<LcdDriver<'a>>,
    pub sensor: Option<SensorDriver<'a>>,
}

impl System<'_> {
    pub fn init_peripherals() -> Peripherals {
        let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
        esp_hal::init(config)
    }

    pub fn builder(alarm: Alarm<'static>) -> SystemBuilder<'static> {
        SystemBuilder::new(alarm)
    }
}

pub struct SystemBuilder<'d> {
    wifi: Option<WifiManager<'d>>,
    lcd: Option<LcdDriver<'d>>,
    sensor: Option<SensorDriver<'d>>,
}

impl<'d> SystemBuilder<'d> {
    pub fn new(alarm: Alarm<'static>) -> Self {
        esp_alloc::heap_allocator!(size: 64 * 1024);

        esp_hal_embassy::init(alarm);
        debug!("embassy initialized");

        Self {
            wifi: None,
            lcd: None,
            sensor: None,
        }
    }

    pub fn with_wifi(
        mut self,
        wifi_init: &'d EspWifiController<'d>,
        wifi: peripherals::WIFI<'d>,
        bt: peripherals::BT<'d>,
    ) -> Self {
        esp_alloc::heap_allocator!(#[unsafe(link_section = ".dram2_uninit")] size: 64 * 1024);
        self.wifi = Some(WifiManager::init(wifi_init, wifi, bt));

        self
    }

    pub fn with_lcd(
        mut self,
        rs: impl OutputPin + 'd,
        rw: impl OutputPin + 'd,
        en: impl OutputPin + 'd,
        d4: impl OutputPin + 'd,
        d5: impl OutputPin + 'd,
        d6: impl OutputPin + 'd,
        d7: impl OutputPin + 'd,
    ) -> Self {
        self.lcd = Some(LcdDriver::new(rs, rw, en, d4, d5, d6, d7));
        self
    }

    pub fn with_sensor(mut self, pin: impl InputPin + 'd) -> Self {
        self.sensor = Some(SensorDriver::new(pin));
        self
    }

    pub fn build(self) -> System<'d> {
        info!("system initialized");
        System {
            wifi: self.wifi,
            lcd: self.lcd,
            sensor: self.sensor,
        }
    }
}
