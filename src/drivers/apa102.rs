/*!
APA102 LED drivers for k20 boards
 */

use hal::k20::spi::{SPI, SPITransmit, SPITxResult};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[repr(C)]
/// The configuration of an individual APA102 led
pub struct APA102 {
    intensity: u8,
    blue: u8,
    green: u8,
    red: u8
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
/// A separately controllable channel of an APA102 led
pub enum Channel {
    /// Intensity from 0b00000 - 0b11111, top 3 bits must be 1
    Intensity,
    /// Blue color channel
    Blue,
    /// Green color channel
    Green,
    /// Red color channel
    Red
}

use core::convert::From;

impl From<APA102> for u32 {
    fn from(val: APA102) -> u32 {
        // magic start of led value
        0xE0_00_00_00 |
            ((val.intensity & 0x1F) as u32).rotate_left(24) |
            (val.blue as u32).rotate_left(8) |
            (val.green as u32).rotate_left(16) |
            (val.red as u32)
    }
}

impl From<u32> for APA102 {
    fn from (val: u32) -> APA102 {
        APA102 { intensity: ((0x1F_00_00_00 & val) >> 24) as u8,
                 blue: ((0x00_FF_00_00 & val) >> 8) as u8,
                 green: ((0x00_00_FF_00 & val) >> 16) as u8,
                 red: (0x00_00_00_FF & val) as u8 }
    }
}

use core::default::Default;
impl Default for APA102 {
    fn default() -> APA102 {
        APA102 {
            intensity: 0x1F,
            blue: 0,
            green: 0,
            red: 0
        }
    }
}

use core::cmp::{min, max};
impl APA102 {

    /// Create a pixel from a Blue/Green/Red value
    pub fn bgr(b: u8, g: u8, r: u8) -> APA102 {
        APA102 {
            intensity: 0x1F,
            blue: b,
            green: g,
            red: r
        }
    }

    /// Alter a led's channel by delta, with lower and upper bounds
    pub fn alter(&mut self, chan: Channel, delta: i8, lower:u8, upper:u8) -> &mut APA102 {
        match chan {
            Channel::Blue => self.blue = alt(self.blue, delta, lower, upper),
            Channel::Green => self.green = alt(self.green, delta, lower, upper),
            Channel::Red => self.red = alt(self.red, delta, lower, upper),
            Channel::Intensity => self.intensity = alt(self.intensity, delta,
                                                       min(lower, 0x1F),
                                                       min(upper, 0x1F))
        };
        self
    }
    /// Clamp all channels to given bounds
    pub fn clamp(&mut self, lower:u8, upper:u8) -> &mut APA102 {
        self.alter(Channel::Blue, 0, lower, upper)
            .alter(Channel::Red, 0, lower, upper)
            .alter(Channel::Green, 0, lower, upper)
            .alter(Channel::Intensity, 0, 0x1E, 0x1F)
    }
}

/// An LED lit blue
pub const BLUE: APA102 = APA102 { blue: 0x08, green: 0, red: 0, intensity: 0x1F };

/// An LED lit green
pub const GREEN: APA102 = APA102 { green: 0x08, blue: 0, red: 0, intensity: 0x1F };

/// An LED lit red
pub const RED: APA102 = APA102 { red: 0x08, blue: 0, green: 0, intensity: 0x1F };

/// An LED lit pink
pub const PINK: APA102 = APA102 { blue: 0x04,
                                  red: 0x04,
                                  green: 0,
                                  intensity: 0x1F };

/// An unlit LED
pub const UNLIT: APA102 = APA102 { blue: 0, red: 0, green: 0, intensity: 0x1F };

fn alt(val: u8, delta:i8, lower: u8, upper: u8) -> u8 {
    match delta.signum() {
        -1 => clamp(val.saturating_sub(delta.abs() as u8), lower, upper),
        0 => clamp(val, lower, upper),
        _ => clamp(val.saturating_add(delta as u8), lower, upper)
    }
}

fn clamp(val: u8, lower: u8, upper: u8) -> u8 {
    min(max(val, lower), upper)
}

/// An APA102 LED strip
pub type LedStrip = [APA102];

impl<'a> SPITransmit for &'a [APA102] {
    fn transmit(&self, spi: &SPI) -> SPITxResult {
         // Preamble
        match (0 as u32).transmit(spi) {
            Ok(_) => {},
            err => return err
        };

        for idx in 0 .. self.len() - 1 {
            // 1 LED
            match u32::from(self[idx]).transmit(spi) {
                Ok(1) => {},
                err => return err
            }
        }

        return Ok(self.len() as u32);
    }
}

/// Rotate the pixels in an APA102 strip right.
pub fn rotate_strip(strip: &mut LedStrip) {
    match strip.len() {
        0 => {},
        1 => {},
        len => {
            let last = len - 1;
            let first_px = strip[last];
            for i in 0..last {
                strip[last - i] = strip[last-i-1];
            }
            strip[0] = first_px;
        }
    };
}
