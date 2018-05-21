extern crate spidev;
use std::io;
use std::io::prelude::*;

use spidev::{SPI_MODE_3, Spidev, SpidevOptions};

const NUM_LEDS: uint8 = 3;

pub struct Pixel {
    // global brightness (0xE0 | 1 to 31 = 0b 111b bbbb)
    brightness: u8,

    blue: u8,
    green: u8,
    red: u8,
}

impl Default for Pixel {
    fn default() -> Pixel {
        Pixel {
            brightness: 0xE0,
            blue: 0,
            green: 0,
            red: 0,
        }
    }
}

fn create_spi() -> io::Result<Spidev> {
    let mut spi = Spidev::open("/dev/spidev0.0")?;
    let options = SpidevOptions::new()
        .bits_per_word(8) // 9 bits for LoSSI mode?
        .max_speed_hz(10_000_000)
        .mode(SPI_MODE_3)
        .build();
    spi.configure(&options)?;
    Ok(spi)
}

fn apa102_sync(&mut spi: Spidev, &leds: [Pixel]) -> Result<()> {
    spi.write(&[0u8; 4])?;
    spi.write(&leds)?;
    spi.write(&[0u8; 4])?;
}

fn main() {
    let mut spi = create_spi();
    let mut leds = [Pixel::default(); NUM_LEDS];
    apa102_syc(&mut spi, &leds);

    thread::sleep(Duration::from_millis(250));

    leds[0] = Pixel {
        red: 255,
        green: 255,
        blue: 255,
    };
    apa102_sync(&mut spi, &leds);

    thread::sleep(Duration::from_millis(250));

    leds[0] = Pixel::default();
    apa102_sync(&mut spi, &leds);
}
