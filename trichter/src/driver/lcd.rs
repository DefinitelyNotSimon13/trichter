#![no_std]

use defmt::info;
use esp_hal::{
    delay::Delay,
    gpio::{InputPin, Output, OutputPin},
};

use crate::output_from_pin;

pub struct Lcd4Bit {
    rs: Output<'static>,
    e: Output<'static>,
    d4: Output<'static>,
    d5: Output<'static>,
    d6: Output<'static>,
    d7: Output<'static>,
    delay: Delay,
    lcd_type: u8,
}

// Command bytes for init sequence
const LCD_LINE2: u8 = 0x40;
const LCD_INIT_CMDS: [u8; 4] = [
    0x20 | (2 << 2), // function set: 4-bit, 2 lines
    0x0C,            // display on, cursor off
    0x01,            // clear display
    0x06,            // entry mode: incr, no shift
];

impl Lcd4Bit {
    /// Create new LCD driver.  
    /// `lcd_type` = 0..2 (here you’ll typically pass 2 for two lines).
    pub fn new(
        rs: impl OutputPin + 'static,
        e: impl OutputPin + 'static,
        d4: impl OutputPin + 'static,
        d5: impl OutputPin + 'static,
        d6: impl OutputPin + 'static,
        d7: impl OutputPin + 'static,
        delay: Delay,
        lcd_type: u8,
    ) -> Self {
        let rs = output_from_pin(rs);
        let e = output_from_pin(e);
        let d4 = output_from_pin(d4);
        let d5 = output_from_pin(d5);
        let d6 = output_from_pin(d6);
        let d7 = output_from_pin(d7);
        Self {
            rs,
            e,
            d4,
            d5,
            d6,
            d7,
            delay,
            lcd_type,
        }
    }

    /// Initialize the display (must be called before anything else).
    pub fn init(&mut self) {
        info!("Initializing lcd...");
        // Ensure control pins low
        self.rs.set_low();
        self.e.set_low();

        // Wait for LCD to power up
        info!("Waiting for power up...");
        self.delay.delay_millis(15);

        // Send 0x3 three times to force 8-bit reset, then 0x2 for 4-bit mode
        info!("Sending 0x3 three times to force 8-bit reset");
        for _ in 0..3 {
            self.send_nibble(0x3);
            self.delay.delay_millis(5);
        }
        info!("Sending 0x2 for 4-bit mode");
        self.send_nibble(0x2);

        // Send final initialization commands
        info!("Sending initialization commands");
        for &cmd in &LCD_INIT_CMDS {
            self.send_byte(false, cmd);
        }
    }

    /// Send a single 4-bit nibble (low nybble of argument).
    fn send_nibble(&mut self, nibble: u8) {
        // Map bits to data pins
        if nibble & 0x01 != 0 {
            self.d4.set_high();
        } else {
            self.d4.set_low();
        };
        if nibble & 0x02 != 0 {
            self.d5.set_high();
        } else {
            self.d5.set_low();
        };
        if nibble & 0x04 != 0 {
            self.d6.set_high();
        } else {
            self.d6.set_low();
        };
        if nibble & 0x08 != 0 {
            self.d7.set_high();
        } else {
            self.d7.set_low();
        };

        // Pulse Enable
        self.delay.delay_nanos(1);
        self.e.set_high();
        self.delay.delay_nanos(2);
        self.e.set_low();
    }

    /// Send a full byte.  `is_data = true` for data, `false` for command.
    pub fn send_byte(&mut self, is_data: bool, byte: u8) {
        // RS pin
        if is_data {
            self.rs.set_high();
        } else {
            self.rs.set_low();
        };

        // RW always low - ground?!
        // // RW low for write
        // self.rw.set_low()?;

        // High nibble then low nibble
        self.send_nibble(byte >> 4);
        self.send_nibble(byte & 0x0F);

        // Short post-write delay
        self.delay.delay_millis(2);
    }

    /// Position cursor (x:1..width, y:1..2).
    pub fn gotoxy(&mut self, x: u8, y: u8) {
        let mut addr = if y > 1 { LCD_LINE2 } else { 0 };
        addr += x - 1;
        self.send_byte(false, 0x80 | addr);
    }

    /// Write one character, interpreting `\f`, `\n`, `\b` specially.
    pub fn putc(&mut self, c: char) {
        match c {
            '\x0C' => {
                // form-feed = clear display
                self.send_byte(false, 0x01);
                self.delay.delay_millis(2);
            }
            '\n' => {
                self.gotoxy(1, 2);
            }
            '\x08' => {
                // backspace
                self.send_byte(false, 0x10);
            }
            data => self.send_byte(true, data as u8),
        }
    }
}
