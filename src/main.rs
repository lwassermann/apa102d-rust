extern crate spidev;

use std::{thread,io};
use std::io::prelude::*;
use std::time::Duration;

use std::slice;
use std::mem;

use spidev::{SPI_MODE_3, Spidev, SpidevOptions};

const NUM_LEDS: usize = 3;

#[repr(packed)]
#[derive(Debug, Copy, Clone)]
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

fn apa102_sync(spi: &mut Spidev, leds: &[u8]) -> io::Result<()> {
    spi.write(&[0u8; 4])?;
    spi.write(&leds)?;
    spi.write(&[0u8; 4])?;
    Ok(())
}

fn main() {
    let mut spi = create_spi().expect("Problems connecting to spidev0");
    let mut leds = [Pixel::default(); NUM_LEDS];
    apa102_sync(&mut spi, &leds);

    thread::sleep(Duration::from_millis(250));

    leds[0] = Pixel {
        brightness: 0xE0,
        red: 255,
        green: 255,
        blue: 255,
    };
    apa102_sync(&mut spi, &leds);

    thread::sleep(Duration::from_millis(250));

    leds[0] = Pixel::default();
    apa102_sync(&mut spi, &leds);
}

#[test]
fn default_pixel_packed_as_4_bytes() {
    let pixel = Pixel::default();
    let pointer: *const Pixel = &pixel;
    let pointer: *const u8 = pointer as *const u8;
    let bytes: &[u8] = unsafe {
        slice:: from_raw_parts(pointer, mem::size_of::<Pixel>())
    };
    assert_eq!(bytes, &[0xE0, 0x00, 0x00, 0x00]);
}

#[test]
fn color_sequence_pixel_packing() {
    let pixel = Pixel { brightness: 0xE0 | 15, red: 21, green: 255, blue: 120 };
    let pointer: *const Pixel = &pixel;
    let pointer: *const u8 = pointer as *const u8;
    let bytes: &[u8] = unsafe {
        slice:: from_raw_parts(pointer, mem::size_of::<Pixel>())
    };
    assert_eq!(bytes, &[0xEF, 120, 255, 21]);
}
