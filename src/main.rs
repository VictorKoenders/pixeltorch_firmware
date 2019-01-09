#![no_std]
#![no_main]

// We have to make sure panic_semihosting is included, because this automagically registers a panic handler
extern crate panic_semihosting;

use cortex_m_rt::entry;
use stm32f103xx_hal::gpio::gpioa::PA2;
use stm32f103xx_hal::gpio::Output;
use stm32f103xx_hal::gpio::PushPull;
use stm32f103xx_hal::prelude::*;
use stm32f103xx_hal::stm32f103xx;
use ws2812b::LedController;

fn make_go_faster(rcc: &stm32f103xx::RCC, flash: &stm32f103xx::FLASH) {
    rcc.cr.modify(|_, w| w.hseon().enabled());
    while !rcc.cr.read().hserdy().is_ready() {}
    flash.acr.modify(|_, w| w.prftbe().enabled());
    flash.acr.modify(|_, w| w.latency().two());
    rcc.cfgr.modify(|_, w| {
        w.hpre()
            .no_div()
            .ppre2()
            .no_div()
            .ppre1()
            .div2()
            .adcpre()
            .div8()
            .pllsrc()
            .external()
            .pllxtpre()
            .no_div()
            .pllmul()
            .mul9()
    });
    rcc.cr.modify(|_, w| w.pllon().enabled());
    while rcc.cr.read().pllrdy().is_unlocked() {}
    rcc.cfgr.modify(|_, w| w.sw().pll());
    while !rcc.cfgr.read().sws().is_pll() {}
}
#[entry]
fn main() -> ! {
    let dp = stm32f103xx::Peripherals::take().unwrap();

    make_go_faster(&dp.RCC, &dp.FLASH);

    let mut rcc = dp.RCC.constrain();
    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);

    let pin = gpioa.pa2.into_push_pull_output(&mut gpioa.crl);
    let mut controller = LedController::new(pin);

    let mut colors = [[0, 100, 0]; 300];

    loop {
        show_colors(&mut controller, &colors);
    }
}

fn show_colors(controller: &mut LedController<PA2<Output<PushPull>>>, colors: &[[u8; 3]; 300]) {
    for color in colors.iter() {
        controller.write(color.iter().cloned());
    }

    for _ in 0..400 {
        cortex_m::asm::nop();
    }
}
