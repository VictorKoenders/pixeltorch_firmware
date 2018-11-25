#![feature(test)]
#![feature(untagged_unions)]
#![no_std]
#![no_main]

extern crate cortex_m;
extern crate enc28j60;
extern crate panic_semihosting;
extern crate stm32f103xx_hal;
#[macro_use]
extern crate stm32f103xx;
#[cfg(test)]
extern crate test;

use crate::buffer::BufferImpl;
use stm32f103xx_hal::afio::AfioExt;
use stm32f103xx_hal::flash::FlashExt;
use stm32f103xx_hal::gpio::GpioExt;
use stm32f103xx_hal::prelude::_stm32f103xx_hal_rcc_RccExt;

mod buffer;
mod interrupt;
mod network;

use cortex_m_rt::entry;

const COLOR_GREEN: [u8; 3] = [0, 255, 0];

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f103xx::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);
    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);

    {
        let buffer = buffer::get_mut();
        buffer.set_pin(gpioa.pa2.into_push_pull_output(&mut gpioa.crl));
        buffer.append(core::iter::repeat(&COLOR_GREEN).take(300).flatten());
    }

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    interrupt::configure(dp.TIM2, clocks, &mut rcc.apb1);

    let _network = network::Network::new(
        gpioa.pa3,
        gpioa.pa4,
        gpioa.pa5,
        gpioa.pa6,
        gpioa.pa7,
        dp.SPI1,
        cp.SYST,
        clocks,
        &mut gpioa.crl,
        &mut afio.mapr,
        &mut rcc.apb2,
    );

    loop {}
}
