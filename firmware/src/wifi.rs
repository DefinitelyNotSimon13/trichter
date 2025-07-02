use core::net::Ipv4Addr;

use crate::{driver::sensor::RESULTS, mk_static};
use alloc::{format, string::String};
use defmt::{debug, error, info, warn};
use embassy_executor::Spawner;
use embassy_net::{tcp::TcpSocket, Runner, StackResources};
use embassy_time::{Duration, Timer};
use embedded_io_async::Write as _;
use esp_hal::{
    peripherals::{self},
    rng::Rng,
};
use esp_wifi::{
    wifi::{
        ClientConfiguration, Configuration, Interfaces, WifiController, WifiDevice, WifiEvent,
        WifiState,
    },
    EspWifiController,
};

const SSID: &str = env!("SSID");
const PASSWORD: &str = env!("PASSWORD");

pub struct WifiManager<'d> {
    interfaces: Interfaces<'d>,
    wifi_controller: WifiController<'d>,
    // ble_controller: ExternalController<BleConnector<'d>, 20>,
}

impl WifiManager<'static> {
    pub fn init(
        wifi_init: &'static EspWifiController<'static>,
        wifi: peripherals::WIFI<'static>,
        bt: peripherals::BT<'static>,
    ) -> Self {
        //
        let (wifi_controller, interfaces) =
            esp_wifi::wifi::new(wifi_init, wifi).expect("Failed to initialize WIFI controller");

        // let transport = BleConnector::new(wifi_init, bt);
        // let ble_controller = ExternalController::<_, 20>::new(transport);

        debug!("wifi/ble initialized");

        Self {
            interfaces,
            wifi_controller,
            // ble_controller,
        }
    }

    pub async fn connect_to_hotspot(self, mut rng: Rng, spawner: Spawner) {
        let dhcp_config = embassy_net::Config::dhcpv4(Default::default());
        let seed = (rng.random() as u64) << 32 | rng.random() as u64;

        let (stack, runner) = embassy_net::new(
            self.interfaces.sta,
            dhcp_config,
            mk_static!(StackResources<3>, StackResources::<3>::new()),
            seed,
        );

        spawner.spawn(connection(self.wifi_controller)).ok();
        spawner.spawn(net_task(runner)).ok();

        let mut rx_buffer = [0; 4096];
        let mut tx_buffer = [0; 4096];

        loop {
            if stack.is_link_up() {
                info!("Link is up!");
                break;
            }
            Timer::after(Duration::from_millis(500)).await;
        }

        info!("Waiting for ip...");
        loop {
            if let Some(config) = stack.config_v4() {
                info!("Got IP: {}", config.address);
                break;
            }
            Timer::after(Duration::from_millis(500)).await;
        }

        loop {
            Timer::after(Duration::from_millis(1000)).await;

            let mut socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);
            socket.set_timeout(Some(Duration::from_secs(10)));

            let remote_endpoint = (Ipv4Addr::new(84, 164, 215, 229), 31080);

            info!("Sending request to server...");
            let r = socket.connect(remote_endpoint).await;
            if let Err(e) = r {
                error!("connection error {:?}", e);
                continue;
            }

            let mut buf = [0; 1024];
            loop {
                use embedded_io_async::Write;

                let r = socket
                    .write_all(b"GET / HTTP/1.0\rHost: www.google.de\r\n\r\n")
                    .await;

                if let Err(e) = r {
                    error!("write error: {:?}", e);
                    break;
                }

                let n = match socket.read(&mut buf).await {
                    Ok(0) => {
                        info!("read EOF");
                        break;
                    }
                    Ok(n) => n,
                    Err(e) => {
                        error!("read error: {:?}", e);
                        break;
                    }
                };

                info!("recieved {} bytes:", n);
                info!("{}", core::str::from_utf8(&buf[..n]).unwrap());
            }
            Timer::after(Duration::from_millis(3000)).await;
        }
    }
}

#[embassy_executor::task]
pub async fn connect_to_hotspot_and_provide_endpoint(
    wifi: WifiManager<'static>,
    mut rng: Rng,
    spawner: Spawner,
) {
    let dhcp_config = embassy_net::Config::dhcpv4(Default::default());
    let seed = (rng.random() as u64) << 32 | rng.random() as u64;

    let (stack, runner) = embassy_net::new(
        wifi.interfaces.sta,
        dhcp_config,
        mk_static!(StackResources<3>, StackResources::<3>::new()),
        seed,
    );

    spawner.spawn(connection(wifi.wifi_controller)).ok();
    spawner.spawn(net_task(runner)).ok();

    let mut rx_buffer = [0; 4096];
    let mut tx_buffer = [0; 4096];

    loop {
        if stack.is_link_up() {
            info!("Link is up!");
            break;
        }
        Timer::after(Duration::from_millis(500)).await;
    }

    info!("Waiting for ip...");
    loop {
        if let Some(config) = stack.config_v4() {
            info!("Got IP: {}", config.address);
            break;
        }
        Timer::after(Duration::from_millis(500)).await;
    }

    loop {
        Timer::after(Duration::from_millis(1_000)).await;

        let mut socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);
        socket.set_timeout(Some(embassy_time::Duration::from_secs(10)));

        info!("Listening on port 80...");
        Timer::after_millis(10);
        if let Err(e) = socket.accept(80).await {
            warn!("accept error: {:?}", e);
            continue;
        }

        info!("Client connected from {:?}", socket.remote_endpoint());
        let mut req = [0u8; 512];
        let mut total = 0;
        loop {
            match socket.read(&mut req[total..]).await {
                Ok(0) => break, // client closed
                Ok(n) => {
                    total += n;
                    if req[..total].windows(4).any(|w| w == b"\r\n\r\n") {
                        break;
                    }
                }
                Err(e) => {
                    warn!("read error: {:?}", e);
                    break;
                }
            }
        }
        let mut body = String::from("<html><body>");

        let results = RESULTS.lock().await;
        for (i, res) in results.iter().enumerate() {
            body.push_str(
                format!(
                    "Run {}: Rate: {} DurationMs: {} <br />",
                    i,
                    res.rate,
                    res.duration.as_millis()
                )
                .as_str(),
            );
        }
        body.push_str("</body></html>");

        let resp = format!(
            "HTTP/1.0 200 OK\r\n\
             Content-Type: text/html\r\n\
             Content-Length: {}\r\n\
             Connection: close\r\n\
             \r\n\
             {}",
            body.len(),
            body,
        );
        if let Err(e) = socket.write_all(resp.as_bytes()).await {
            warn!("write error: {:?}", e);
        }

        // Allow the client to receive everything
        let _ = socket.flush().await;

        Timer::after_millis(10);
    }
}

#[embassy_executor::task]
async fn connection(mut controller: WifiController<'static>) {
    info!("start connection task");
    loop {
        match esp_wifi::wifi::wifi_state() {
            WifiState::StaConnected => {
                // wait until we're no longer connected
                controller.wait_for_event(WifiEvent::StaDisconnected).await;
                Timer::after(Duration::from_millis(5000)).await
            }
            _ => {}
        }
        if !matches!(controller.is_started(), Ok(true)) {
            let client_config = Configuration::Client(ClientConfiguration {
                ssid: SSID.into(),
                password: PASSWORD.into(),
                ..Default::default()
            });
            controller.set_configuration(&client_config).unwrap();
            info!("Starting wifi");
            controller.start_async().await.unwrap();
            info!("Wifi started!");

            info!("Scan");
            let result = controller.scan_n_async(10).await.unwrap();
            for ap in result {
                info!("{:?}", ap);
            }
        }
        info!("About to connect...");

        match controller.connect_async().await {
            Ok(_) => info!("Wifi connected!"),
            Err(e) => {
                info!("Failed to connect to wifi: {:#?}", e);
                Timer::after(Duration::from_millis(5000)).await
            }
        }
    }
}

#[embassy_executor::task]
async fn net_task(mut runner: Runner<'static, WifiDevice<'static>>) {
    runner.run().await
}

pub fn create_wifi_controller(
    timg0: peripherals::TIMG0<'static>,
    rng: Rng,
    radio_clk: peripherals::RADIO_CLK<'static>,
) -> &'static EspWifiController<'static> {
    use esp_wifi::init;
    todo!()
}
