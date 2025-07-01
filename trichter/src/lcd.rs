use ag_lcd::LcdDisplay;
use defmt::debug;
use esp_hal::{
    delay::Delay,
    gpio::{Output, OutputPin},
};

use crate::output_from_pin;

pub struct LcdHandler<'d> {
    pub lcd: LcdDisplay<Output<'d>, esp_hal::delay::Delay>,
}

impl<'d> LcdHandler<'d> {
    pub fn new(
        rs: impl OutputPin + 'd,
        en: impl OutputPin + 'd,
        d4: impl OutputPin + 'd,
        d5: impl OutputPin + 'd,
        d6: impl OutputPin + 'd,
        d7: impl OutputPin + 'd,
    ) -> Self {
        let rs = output_from_pin(rs);
        let en = output_from_pin(en);
        let d4 = output_from_pin(d4);
        let d5 = output_from_pin(d5);
        let d6 = output_from_pin(d6);
        let d7 = output_from_pin(d7);

        let lcd = LcdDisplay::new(rs, en, Delay::new())
            .with_half_bus(d4, d5, d6, d7)
            .with_display(ag_lcd::Display::On)
            .with_blink(ag_lcd::Blink::On)
            .with_cursor(ag_lcd::Cursor::On)
            .build();

        debug!("lcd driver initialized");

        LcdHandler { lcd }
    }
}
