#![deny(unsafe_code)]
#![no_main]
#![no_std]

use black_pill as _;
use cortex_m_rt::entry;
use smart_leds::{brightness, hsv::RGB8, SmartLedsWrite};
use stm32f4xx_hal::{gpio::NoPin, pac, prelude::*, spi::Spi};
use ws2812_spi as ws2812;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().expect("cannot take peripherals dp");
    let cp = cortex_m::peripheral::Peripherals::take().expect("cannot take peripherals cp");
    // let rcc = dp.RCC.constrain();
    // let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

    // let mut delay = Delay::new(cp.SYST, &clocks);
    // Configure APB bus clock to 56MHz, cause ws2812b requires 3.5Mbps SPI
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(25.MHz()).sysclk(56.MHz()).freeze();

    //let mut delay = Delay::new(cp.SYST, &clocks);
    let mut delay = cp.SYST.delay(&clocks);
    // let mut delay = dp.TIM1.delay_us(&clocks);
    let gpioa = dp.GPIOA.split();

    // Configure pins for SPI
    let sck1 = gpioa.pa5.into_alternate();
    let miso1 = NoPin; // miso not needed
    let mosi1 = gpioa.pa7.into_alternate(); // PA7 is output going to data line of leds

    // SPI1 with 3Mhz
    let spi = Spi::new(
        dp.SPI1,
        (sck1, miso1, mosi1),
        ws2812::MODE,
        3_000_000.Hz(),
        &clocks,
    );

    // let spi = dp.SPI1.spi(
    //     (gpioa.pa5, NoPin, gpioa.pa7),
    //     ws2812::MODE,
    //     3500.khz(),
    //     &clocks,
    // );

    let mut ws = ws2812::Ws2812::new(spi);

    const NUM_LEDS: usize = 8;
    let mut data = [RGB8::default(); NUM_LEDS];

    // Wait before start write for syncronization
    // delay.delay(200.micros());
    delay.delay_us(200u8);

    loop {
        for j in 0..(256 * 5) {
            for (i, b) in data.iter_mut().enumerate() {
                *b = wheel((((i * 256) as u16 / NUM_LEDS as u16 + j as u16) & 255) as u8);
            }
            ws.write(brightness(data.iter().cloned(), 32)).unwrap();
            delay.delay_us(10u8);
        }
    }
}

/// Input a value 0 to 255 to get a color value
/// The colours are a transition r - g - b - back to r.
fn wheel(mut wheel_pos: u8) -> RGB8 {
    wheel_pos = 255 - wheel_pos;
    if wheel_pos < 85 {
        return (255 - wheel_pos * 3, 0, wheel_pos * 3).into();
    }
    if wheel_pos < 170 {
        wheel_pos -= 85;
        return (0, wheel_pos * 3, 255 - wheel_pos * 3).into();
    }
    wheel_pos -= 170;
    (wheel_pos * 3, 255 - wheel_pos * 3, 0).into()
}
