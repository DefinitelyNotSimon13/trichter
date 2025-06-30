use core::sync::atomic::{AtomicU32, Ordering};
use defmt::debug;
use embassy_time::{Duration, Instant};
use esp_hal::gpio::{Event, Input, InputConfig, InputPin};

pub struct SensorDriver<'d> {
    input: Input<'d>,
}

static PULSE_COUNT: AtomicU32 = AtomicU32::new(0);

impl<'d> SensorDriver<'d> {
    pub fn new(pin: impl InputPin + 'd) -> Self {
        let mut inp = Input::new(pin, InputConfig::default());
        inp.listen(Event::RisingEdge);
        debug!("sensor driver initialized");
        SensorDriver { input: inp }
    }

    pub async fn measure_pulses(&mut self, duration: Duration) -> u32 {
        PULSE_COUNT.store(0, Ordering::Relaxed);
        let start = Instant::now();
        while Instant::now() - start < duration {
            if self.input.is_interrupt_set() {
                PULSE_COUNT.fetch_add(1, Ordering::Relaxed);
                self.input.clear_interrupt();
            }
        }
        PULSE_COUNT.load(Ordering::Relaxed)
    }

    pub fn pulses_to_flow(pulses: u32, window_s: f32) -> f32 {
        let pulses_per_sec = pulses as f32 / window_s;
        let flow_l_per_min = pulses_per_sec * 60.0 / 6.6;
        flow_l_per_min
    }
}
