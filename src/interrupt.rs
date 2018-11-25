use crate::buffer;
use stm32f103xx::TIM2;
use stm32f103xx_hal::rcc::{Clocks, APB1};
use stm32f103xx_hal::time::U32Ext;
use stm32f103xx_hal::timer::{Event, Timer};

#[derive(Debug)]
pub enum PinState {
    Initial,

    LowBitFirst,
    LowBitSecond,
    LowBitEnd,

    HighBitFirst,
    HighBitSecond,
    HighBitEnd,
}

impl PinState {
    pub fn next(&mut self) {
        *self = match self {
            PinState::Initial => unreachable!(),

            PinState::LowBitFirst => PinState::LowBitSecond,
            PinState::LowBitSecond => PinState::LowBitEnd,
            PinState::LowBitEnd => PinState::Initial,

            PinState::HighBitFirst => PinState::HighBitSecond,
            PinState::HighBitSecond => PinState::HighBitEnd,
            PinState::HighBitEnd => PinState::Initial,
        }
    }
}

static mut PIN_STATE: PinState = PinState::Initial;

interrupt!(TIM2, tick);

pub fn configure(tim: TIM2, clocks: Clocks, apb1: &mut APB1) {
    let mut timer = Timer::tim2(tim, 3.mhz(), clocks, apb1);
    timer.listen(Event::Update);
}

fn tick() {
    let pinstate = unsafe { &PIN_STATE };
    match pinstate {
        PinState::Initial => {
            let buffer = buffer::get_mut();
            if let Some(bit) = buffer.next() {
                // TODO: Set high
                #[cfg(test)]
                test::black_box(());
                unsafe {
                    if bit {
                        PIN_STATE = PinState::HighBitFirst;
                    } else {
                        PIN_STATE = PinState::LowBitFirst;
                    }
                }
            }
            return;
        }
        PinState::LowBitFirst | PinState::HighBitEnd => {
            #[cfg(test)]
            test::black_box(());
            // TODO: Set low
        }
        _ => {}
    }
    unsafe {
        PIN_STATE.next();
    }
}

#[bench]
pub fn bench_interrupt(b: &mut test::Bencher) {
    unsafe {
        buffer::get().append(&[1, 2, 3, 4]);
    }

    b.iter(|| {
        unsafe {
            let mut buffer = buffer::get_mut();
            buffer.current.index = 0;
            buffer.current.bit = 7;
        }
        for _ in 0..128 {
            interrupt::interrupt();
        }
    })
}
