use enc28j60::{Enc28j60, Unconnected};
use stm32f103xx::{SPI1, SYST};
use stm32f103xx_hal::afio::MAPR;
use stm32f103xx_hal::delay::Delay;
use stm32f103xx_hal::gpio::gpioa::{CRL, PA3, PA4, PA5, PA6, PA7};
use stm32f103xx_hal::gpio::{Alternate, Floating, Input, Output, PushPull};
use stm32f103xx_hal::prelude::{_embedded_hal_digital_OutputPin, _stm32f103xx_hal_time_U32Ext};
use stm32f103xx_hal::rcc::{Clocks, APB2};
use stm32f103xx_hal::spi::Spi;

type RESET = PA3<Output<PushPull>>;
type NCS = PA4<Output<PushPull>>;
type SCK = PA5<Alternate<PushPull>>;
type MISO = PA6<Input<Floating>>;
type MOSI = PA7<Alternate<PushPull>>;

/* Configuration */
const MAC: [u8; 6] = [0x20, 0x18, 0x03, 0x01, 0x00, 0x00];
// const IP: [u8; 4] = [192, 168, 1, 33];

/* Constants */
const KB: u16 = 1024; // bytes

type EncSpiPins = (SCK, MISO, MOSI);
type EncSpi = Spi<SPI1, EncSpiPins>;
pub struct Network {
    #[allow(dead_code)]
    enc28j60: Enc28j60<EncSpi, NCS, Unconnected, RESET>,
}

impl Network {
    pub fn new(
        reset: PA3<Input<Floating>>,
        ncs: PA4<Input<Floating>>,
        sck: PA5<Input<Floating>>,
        miso: PA6<Input<Floating>>,
        mosi: PA7<Input<Floating>>,
        spi: SPI1,
        system_clock: SYST,
        clocks: Clocks,
        crl: &mut CRL,
        mapr: &mut MAPR,
        apb2: &mut APB2,
    ) -> Network {
        let mut ncs = ncs.into_push_pull_output(crl);
        ncs.set_high();
        let sck = sck.into_alternate_push_pull(crl);
        let mosi = mosi.into_alternate_push_pull(crl);

        let spi = Spi::spi1(
            spi,
            (sck, miso, mosi),
            mapr,
            enc28j60::MODE,
            1.mhz(),
            clocks,
            apb2,
        );

        let mut reset = reset.into_push_pull_output(crl);
        reset.set_high();
        let mut delay = Delay::new(system_clock, clocks);

        let enc28j60 = Enc28j60::new(
            spi,
            ncs,
            enc28j60::Unconnected,
            reset,
            &mut delay,
            7 * KB,
            MAC,
        )
        .ok()
        .unwrap();

        Network { enc28j60 }
    }
}
