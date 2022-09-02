#![no_main]
#![no_std]

use black_pill as _; // global logger + panicking-behavior + memory layout
use defmt::Format;
use hal::blocking::delay::DelayMs;
use stm32f4xx_hal::gpio::gpiob::{PB6, PB7};
use stm32f4xx_hal::gpio::{Alternate, OpenDrain};
use stm32f4xx_hal::hal;
use stm32f4xx_hal::i2c::{I2c, Mode};
use stm32f4xx_hal::pac::I2C1;
use stm32f4xx_hal::pac::{CorePeripherals, Peripherals};
use stm32f4xx_hal::prelude::*;
use stm32f4xx_hal::timer::SysDelay;

const ADDR: u8 = 0x38;
const CMD_INIT: u8 = 0xE1;
const INIT_ARG1: u8 = 0x08;
const INIT_ARG2: u8 = 0x00;
const CMD_MEASURE: u8 = 0xAC;
const MEASURE_ARG1: u8 = 0x33;
const MEASURE_ARG2: u8 = 0x00;
const CMD_RESET: u8 = 0xBA;

pub struct Aht100 {
    i2c: I2c<I2C1, (PB6<Alternate<4, OpenDrain>>, PB7<Alternate<4, OpenDrain>>)>,
}

#[derive(Format)]
pub struct AhtData {
    pub temp: f32,
    pub hum: f32,
}

pub struct AhtStatus {
    busy: bool,
    _mode: AhtMode,
    cal: bool,
}

pub enum AhtMode {
    Nor,
    Cyc,
    Cmd,
}

impl Aht100 {
    pub fn new(
        i2c: I2c<I2C1, (PB6<Alternate<4, OpenDrain>>, PB7<Alternate<4, OpenDrain>>)>,
    ) -> Self {
        Self { i2c }
    }

    pub fn reset(&mut self) -> Result<(), ()> {
        match self.i2c.write(ADDR, &[CMD_RESET]) {
            Ok(_) => Ok(()),
            Err(_e) => {
                // println!("Write reset command failed. {}", e);
                Err(())
            }
        }
    }

    pub fn init(&mut self, delay: &mut SysDelay) -> Result<AhtStatus, ()> {
        delay.delay_ms(40_u16);
        match self.i2c.write(ADDR, &[CMD_INIT, INIT_ARG1, INIT_ARG2]) {
            Ok(_) => {
                delay.delay_ms(75_u16);
                let mut buffer = [0; 6];
                match self.i2c.read(ADDR, &mut buffer) {
                    Ok(_) => Ok(self.decode_status(buffer[0])),
                    Err(_e) => {
                        // println!("Read device status failed. {}", e);
                        Err(())
                    }
                }
            }
            Err(_e) => {
                // println!("Write init command failed. {}", e);
                Err(())
            }
        }
    }

    pub fn measure(&mut self, delay: &mut SysDelay) -> Result<[u8; 5], ()> {
        match self
            .i2c
            .write(ADDR, &[CMD_MEASURE, MEASURE_ARG1, MEASURE_ARG2])
        {
            Ok(_) => {
                delay.delay_ms(75_u16);
                let mut buffer = [0; 6];
                match self.i2c.read(ADDR, &mut buffer) {
                    Ok(_) => {
                        let status = self.decode_status(buffer[0]);
                        if status.busy {
                            // println!("Device is busy");
                            return Err(());
                        }
                        if !status.cal {
                            // println!("Device is not calibration");
                            return Err(());
                        }
                        Ok([buffer[1], buffer[2], buffer[3], buffer[4], buffer[5]])
                    }
                    Err(_e) => {
                        // println!("Read device status failed. {}", e);
                        Err(())
                    }
                }
            }
            Err(_e) => {
                // println!("Write measure command failed. {}", e);
                Err(())
            }
        }
    }
    // pub fn free(self) {
    //     self.i2c.free();
    // }

    fn decode_status(&self, byte: u8) -> AhtStatus {
        AhtStatus {
            busy: byte > 0x7F,
            _mode: match byte & 0b01100000 {
                0x00 => AhtMode::Nor,
                0x20 => AhtMode::Cyc,
                _ => AhtMode::Cmd,
            },
            cal: (byte & 0x08) == 0x08,
        }
    }

    fn decode_data(&self, bytes: [u8; 5]) -> AhtData {
        let hum = ((bytes[0] as u32) << 12) | ((bytes[1] as u32) << 4) | ((bytes[2] >> 4) as u32);
        let temp =
            (((bytes[2] << 4 >> 4) as u32) << 16) | ((bytes[3] as u32) << 8) | (bytes[4] as u32);
        let d = 1_u32 << 20;
        let hum: f32 = hum as f32 / d as f32 * 100.0;
        let temp: f32 = temp as f32 / d as f32 * 200.0 - 50.0;
        AhtData { hum, temp }
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Read AHT100 ...");

    let p = Peripherals::take().unwrap();
    let cp = CorePeripherals::take().unwrap();
    let rcc = p.RCC.constrain();

    let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();
    let mut delay = cp.SYST.delay(&clocks);

    let gpiob = p.GPIOB.split();

    let scl = gpiob
        .pb6
        .into_alternate_open_drain()
        .internal_pull_up(false);

    let sda = gpiob
        .pb7
        .into_alternate_open_drain()
        .internal_pull_up(false);

    let i2c = I2c::new(p.I2C1, (scl, sda), Mode::standard(100.kHz()), &clocks);

    let mut aht100 = Aht100::new(i2c);

    loop {
        let _ = aht100.init(&mut delay);
        if let Ok(result) = aht100.measure(&mut delay) {
            defmt::info!("{:?}", aht100.decode_data(result));
        }

        delay.delay_ms(1000_u16);
    }
}
