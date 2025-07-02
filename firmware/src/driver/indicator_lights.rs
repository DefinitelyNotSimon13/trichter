use esp_hal::gpio::{Level, Output, OutputConfig, OutputPin};

use crate::output_from_pin;

pub struct IndicatorLights {
    rgb_led_red: Output<'static>,
    rgb_led_green: Output<'static>,
    rgb_led_blue: Output<'static>,
    onboard_led: Output<'static>,
}

impl IndicatorLights {
    pub fn new(
        pin_rgb_red: impl OutputPin + 'static,
        pin_rgb_green: impl OutputPin + 'static,
        pin_rgb_blue: impl OutputPin + 'static,
        pin_onboard: impl OutputPin + 'static,
    ) -> Self {
        let rgb_led_red = Output::new(pin_rgb_red, Level::High, OutputConfig::default());
        let rgb_led_green = Output::new(pin_rgb_green, Level::High, OutputConfig::default());
        let rgb_led_blue = Output::new(pin_rgb_blue, Level::High, OutputConfig::default());

        let onboard_led = output_from_pin(pin_onboard);

        Self {
            rgb_led_red,
            rgb_led_green,
            rgb_led_blue,
            onboard_led,
        }
    }
}
