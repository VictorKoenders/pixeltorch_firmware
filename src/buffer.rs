use stm32f103xx_hal::gpio::{gpioa::PA2, Output, PushPull};
use stm32f103xx_hal::prelude::_embedded_hal_digital_OutputPin;

static mut WS2812B_BUFFER: Buffer = Buffer::new();

pub fn get_mut() -> &'static mut impl BufferImpl {
    // We can use unsafe here because our stm32 is singlethreaded
    // The only situation we can get undefined behavior here, is if we trigger an interrupt
    // Consumers of this function should take care of that in `cli` blocks
    unsafe { &mut WS2812B_BUFFER }
}

#[repr(C)]
union BufferPin {
    uninitialized: (),
    initialized: PA2<Output<PushPull>>,
}

struct BufferIndex {
    pub index: usize,
    pub bit: u8,
}

struct Buffer {
    pub queue: [u8; 1024],
    pub current: BufferIndex,
    pub len: usize,
    pub pin: BufferPin,
}

pub trait BufferImpl: Iterator<Item = bool> {
    fn set_pin(&mut self, pin: PA2<Output<PushPull>>);
    fn append(&mut self, buf: impl Iterator<Item = impl core::ops::Deref<Target = u8>>);
    fn set_high(&mut self);
    fn set_low(&mut self);
}

impl BufferImpl for Buffer {
    fn set_pin(&mut self, pin: PA2<Output<PushPull>>) {
        unsafe {
            self.pin.initialized = pin;
        }
    }

    fn append(&mut self, buf: impl Iterator<Item = impl core::ops::Deref<Target = u8>>) {
        // first, we cut off the bits we've already pushed
        let remaining_len = self.len - self.current.index;
        for i in 0..remaining_len {
            self.queue[i] = self.queue[self.current.index + i];
        }
        self.len -= self.current.index;
        self.current.index = 0;

        for b in buf {
            if self.len == self.queue.len() {
                break;
            }
            self.queue[self.len] = *b.deref();
            self.len += 1;
        }
    }

    fn set_high(&mut self) {
        unsafe { &mut self.pin.initialized }.set_high();
    }

    fn set_low(&mut self) {
        unsafe { &mut self.pin.initialized }.set_low();
    }
}

impl Buffer {
    pub const fn new() -> Buffer {
        Buffer {
            queue: [0u8; 1024],
            current: BufferIndex { index: 0, bit: 7 },
            len: 0,
            // Unsafe here is justified because the caller should always call `set_pin` before setting up interrupts
            pin: BufferPin { uninitialized: () },
        }
    }
}

impl Iterator for Buffer {
    type Item = bool;

    fn next(&mut self) -> Option<bool> {
        if self.current.index >= self.len {
            None
        } else {
            let value = (self.queue[self.current.index] & (1 << self.current.bit)) > 0;
            if self.current.bit == 0 {
                self.current.bit = 7;
                self.current.index += 1;
            } else {
                self.current.bit -= 1;
            }
            Some(value)
        }
    }
}
