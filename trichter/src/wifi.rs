use bt_hci::controller::ExternalController;
use defmt::debug;
use esp_hal::{
    peripherals::{self},
    timer::timg::TimerGroup,
};
use esp_wifi::{
    ble::controller::BleConnector,
    wifi::{Interfaces, WifiController},
    EspWifiController,
};

pub struct WifiManager<'d> {
    interfaces: Interfaces<'d>,
    wifi_controller: WifiController<'d>,
    ble_controller: ExternalController<BleConnector<'d>, 20>,
}

impl<'d> WifiManager<'d> {
    pub fn init(
        wifi_init: &'d EspWifiController<'d>,
        wifi: peripherals::WIFI<'d>,
        bt: peripherals::BT<'d>,
    ) -> Self {
        //
        let (wifi_controller, interfaces) =
            esp_wifi::wifi::new(wifi_init, wifi).expect("Failed to initialize WIFI controller");

        let transport = BleConnector::new(wifi_init, bt);
        let ble_controller = ExternalController::<_, 20>::new(transport);

        debug!("wifi/ble initialized");

        Self {
            interfaces,
            wifi_controller,
            ble_controller,
        }
    }
}

pub fn create_wifi_init<'i>(
    rng: peripherals::RNG<'i>,
    timg0: peripherals::TIMG0<'i>,
    radio_clk: peripherals::RADIO_CLK<'i>,
) -> EspWifiController<'i> {
    let rng = esp_hal::rng::Rng::new(rng);
    let timer1 = TimerGroup::new(timg0);
    return esp_wifi::init(timer1.timer0, rng, radio_clk)
        .expect("Failed to initialize WIFI/BLE controller");
}
