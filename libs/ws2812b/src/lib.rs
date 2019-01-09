#![no_std]

use embedded_hal::digital::OutputPin;

pub struct LedController<T: OutputPin> {
    pin: T,
}

impl<T: OutputPin> LedController<T> {
    pub fn new(pin: T) -> Self {
        Self { pin }
    }

    pub fn write(&mut self, it: impl Iterator<Item = u8>) {
        for b in it {
            self.write_bit(b);
        }
    }

    pub fn write_bit(&mut self, b: u8) {
        let mut bits = [false; 8];
        for (i, bit) in bits.iter_mut().enumerate() {
            let mask = 1u8 << (7 - i);
            *bit = b & mask > 0;
        }
        for bit in &bits {
            if *bit {
                self.write_high();
            } else {
                self.write_low();
            }
        }
    }

    fn write_high(&mut self) {
        self.pin.set_high();
        self.pin.set_high();
        self.pin.set_high();
        self.pin.set_high();
        self.pin.set_high();
        self.pin.set_low();
        self.pin.set_low();
    }

    fn write_low(&mut self) {
        self.pin.set_high();
        self.pin.set_high();
        self.pin.set_low();
        self.pin.set_low();
        self.pin.set_low();
        self.pin.set_low();
        self.pin.set_low();
    }
}


