#![deny(unsafe_code)]
#![no_main]
#![no_std]

use black_pill as _;
use cortex_m_rt::entry;
use smart_leds::{
    brightness,
    colors::{
        ALICE_BLUE, ANTINQUE_WHITE, AQUA, AQUAMARINE, AZURE, BEIGE, BISQUE, BLACK, BLANCHED_ALMOND,
        BLUE, BLUE_VIOLET, BROWN, BURLYWOOD, CADET_BLUE, CHARTREUSE, CHOCOLATE, CORAL,
        CORNFLOWER_BLUE, CORNSILK, CRIMSON, CYAN, DARK_BLUE, DARK_CYAN, DARK_GOLDENROD, DARK_GRAY,
        DARK_GREEN, DARK_KHAKI, DARK_MAGENTA, DARK_OLIVE_GREEN, DARK_ORANGE, DARK_ORCHID, DARK_RED,
        DARK_SALMON, DARK_SEA_GREEN, DARK_SLATE_BLUE, DARK_SLATE_GRAY, DARK_TURQUOISE, DARK_VIOLET,
        DEEP_PINK, DEEP_SKY_BLUE, DIM_GRAY, DODGER_BLUE, FIREBRICK, FLORAL_WHITE, FOREST_GREEN,
        FUCHSIA, GAINSBORO, GHOST_WHITE, GOLD, GOLDENROD, GRAY, GREEN, GREEN_YELLOW, HONEYDEW,
        HOT_PINK, INDIAN_RED, INDIGO, IVORY, KHAKI, LAVENDER, LAVENDER_BLUSH, LAWN_GREEN,
        LEMON_CHIFFON, LIGHT_BLUE, LIGHT_CORAL, LIGHT_CYAN, LIGHT_GOLDENROD_YELLOW, LIGHT_GRAY,
        LIGHT_GREEN, LIGHT_PINK, LIGHT_SALMON, LIGHT_SEA_GREEN, LIGHT_SKY_BLUE, LIGHT_SLATE_GRAY,
        LIGHT_STEEL_BLUE, LIGHT_YELLOW, LIME, LIME_GREEN, LINEN, MAGENTA, MAROON,
        MEDIUM_AQUAMARINE, MEDIUM_BLUE, MEDIUM_ORCHID, MEDIUM_PURPLE, MEDIUM_SEA_GREEN,
        MEDIUM_SLATE_BLUE, MEDIUM_SPRING_GREEN, MEDIUM_TURQUOISE, MEDIUM_VIOLET_RED, MIDNIGHT_BLUE,
        MINT_CREAM, MISTY_ROSE, MOCCASIN, NAVAJO_WHITE, NAVY, OLD_LACE, OLIVE, OLIVE_DRAB, ORANGE,
        ORANGE_RED, ORCHID, PALE_GOLDENROD, PALE_GREEN, PALE_TURQUOISE, PALE_VIOLET_RED,
        PAPAYA_WHIP, PEACH_PUFF, PERU, PINK, PLUM, POWDER_BLUE, PURPLE, RED, ROSY_BROWN,
        ROYAL_BLUE, SADDLE_BROWN, SALMON, SANDY_BROWN, SEASHELL, SEA_GREEN, SIENNA, SILVER,
        SKY_BLUE, SLATE_BLUE, SLATE_GRAY, SNOW, SPRING_GREEN, STEEL_BLUE, TAN, TEAL, THISTLE,
        TOMATO, TURQUOISE, VIOLET, WHEAT, WHITE, WHITE_SMOKE, YELLOW, YELLOW_GREEN,
    },
    gamma,
    hsv::{hsv2rgb, Hsv},
    SmartLedsWrite, RGB8,
};
use stm32f4xx_hal::{gpio::NoPin, pac, prelude::*, spi::Spi};
use ws2812_spi as ws2812;

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Constrain clocking registers
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(100.MHz()).freeze();

        let mut delay = cp.SYST.delay(&clocks);

        // GPIOA used for SPI, GPIOC for onboard led
        let gpioa = dp.GPIOA.split();
        let gpioc = dp.GPIOC.split();

        // turn onboard led on
        let mut led = gpioc.pc13.into_push_pull_output();
        led.set_low();

        // Configure pins for SPI
        let sck1 = gpioa.pa5.into_alternate();
        let miso1 = NoPin; // miso not needed
        let mosi1 = gpioa.pa7.into_alternate(); // PA7 is output going to data line of leds

        // SPI1 with 3Mhz
        let spi = Spi::new(
            dp.SPI1,
            (sck1, miso1, mosi1),
            ws2812::MODE,
            3.MHz(),
            &clocks,
        );

        let mut ws = ws2812::Ws2812::new(spi);

        const LED_NUM: usize = 60;
        let mut data = [RGB8::default(); LED_NUM];

        // Breathing LED demo
        // loop {
        //     for j in 60..255 * 2 - 60 {
        //         let bright = if j > 255 { 255 * 2 - j } else { j };
        //         for i in 0..LED_NUM {
        //             data[i] = BLUE;
        //         }
        //         ws.write(gamma(brightness(data.iter().cloned(), bright as u8)))
        //             .unwrap();
        //         delay.delay_ms(10_u8);
        //     }
        // }

        loop {
            // Rainbow LED demo
            for j in 0..256 {
                for i in 0..LED_NUM {
                    // rainbow cycle using HSV, where hue goes through all colors in circle
                    // value sets the brightness
                    let hsv = Hsv {
                        hue: ((i * 3 + j) % 256) as u8,
                        sat: 255,
                        val: 100,
                    };

                    data[i] = hsv2rgb(hsv);
                }
                // before writing, apply gamma correction for nicer rainbow
                ws.write(gamma(data.iter().cloned())).unwrap();
                delay.delay_ms(10u8);
            }

            // All colors demo
            for color in [
                ALICE_BLUE,
                ANTINQUE_WHITE,
                AQUA,
                AQUAMARINE,
                AZURE,
                BEIGE,
                BISQUE,
                BLACK,
                BLANCHED_ALMOND,
                BLUE,
                BLUE_VIOLET,
                BROWN,
                BURLYWOOD,
                CADET_BLUE,
                CHARTREUSE,
                CHOCOLATE,
                CORAL,
                CORNFLOWER_BLUE,
                CORNSILK,
                CRIMSON,
                CYAN,
                DARK_BLUE,
                DARK_CYAN,
                DARK_GOLDENROD,
                DARK_GRAY,
                DARK_GREEN,
                DARK_KHAKI,
                DARK_MAGENTA,
                DARK_OLIVE_GREEN,
                DARK_ORANGE,
                DARK_ORCHID,
                DARK_RED,
                DARK_SALMON,
                DARK_SEA_GREEN,
                DARK_SLATE_BLUE,
                DARK_SLATE_GRAY,
                DARK_TURQUOISE,
                DARK_VIOLET,
                DEEP_PINK,
                DEEP_SKY_BLUE,
                DIM_GRAY,
                DODGER_BLUE,
                FIREBRICK,
                FLORAL_WHITE,
                FOREST_GREEN,
                FUCHSIA,
                GAINSBORO,
                GHOST_WHITE,
                GOLD,
                GOLDENROD,
                GRAY,
                GREEN,
                GREEN_YELLOW,
                HONEYDEW,
                HOT_PINK,
                INDIAN_RED,
                INDIGO,
                IVORY,
                KHAKI,
                LAVENDER,
                LAVENDER_BLUSH,
                LAWN_GREEN,
                LEMON_CHIFFON,
                LIGHT_BLUE,
                LIGHT_CORAL,
                LIGHT_CYAN,
                LIGHT_GOLDENROD_YELLOW,
                LIGHT_GRAY,
                LIGHT_GREEN,
                LIGHT_PINK,
                LIGHT_SALMON,
                LIGHT_SEA_GREEN,
                LIGHT_SKY_BLUE,
                LIGHT_SLATE_GRAY,
                LIGHT_STEEL_BLUE,
                LIGHT_YELLOW,
                LIME,
                LIME_GREEN,
                LINEN,
                MAGENTA,
                MAROON,
                MEDIUM_AQUAMARINE,
                MEDIUM_BLUE,
                MEDIUM_ORCHID,
                MEDIUM_PURPLE,
                MEDIUM_SEA_GREEN,
                MEDIUM_SLATE_BLUE,
                MEDIUM_SPRING_GREEN,
                MEDIUM_TURQUOISE,
                MEDIUM_VIOLET_RED,
                MIDNIGHT_BLUE,
                MINT_CREAM,
                MISTY_ROSE,
                MOCCASIN,
                NAVAJO_WHITE,
                NAVY,
                OLD_LACE,
                OLIVE,
                OLIVE_DRAB,
                ORANGE,
                ORANGE_RED,
                ORCHID,
                PALE_GOLDENROD,
                PALE_GREEN,
                PALE_TURQUOISE,
                PALE_VIOLET_RED,
                PAPAYA_WHIP,
                PEACH_PUFF,
                PERU,
                PINK,
                PLUM,
                POWDER_BLUE,
                PURPLE,
                RED,
                ROSY_BROWN,
                ROYAL_BLUE,
                SADDLE_BROWN,
                SALMON,
                SANDY_BROWN,
                SEASHELL,
                SEA_GREEN,
                SIENNA,
                SILVER,
                SKY_BLUE,
                SLATE_BLUE,
                SLATE_GRAY,
                SNOW,
                SPRING_GREEN,
                STEEL_BLUE,
                TAN,
                TEAL,
                THISTLE,
                TOMATO,
                TURQUOISE,
                VIOLET,
                WHEAT,
                WHITE,
                WHITE_SMOKE,
                YELLOW,
                YELLOW_GREEN,
            ] {
                for i in 0..LED_NUM {
                    data[i] = color;
                }
                ws.write(gamma(brightness(data.iter().cloned(), 100)))
                    .unwrap();
                delay.delay_ms(2000_u16);
            }
        }
    }
    loop {
        continue;
    }
}
