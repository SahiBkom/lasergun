#![no_main]
#![no_std]

use crate::hal::{
    delay,
    prelude::*,
    spi::Spi,
    spi::{Mode, Phase, Polarity},
    stm32,
};
use cortex_m;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
#[allow(unused)]
use panic_halt;
use stm32f1xx_hal as hal;

#[entry]
fn main() -> ! {
    const MODE: Mode = Mode {
        polarity: Polarity::IdleHigh,
        phase: Phase::CaptureOnSecondTransition,
    };

    if let Some(device) = stm32::Peripherals::take() {
        cortex_m::interrupt::free(move |cs| {
            let mut flash = device.FLASH.constrain();
            let mut rcc = device.RCC.constrain();
            hprintln!("set clocks").unwrap();
            let clocks = rcc
                .cfgr
                .use_hse(8.mhz())
                .hclk(72.mhz())
                .sysclk(72.mhz())
                .pclk1(36.mhz())
                .pclk1(72.mhz())
                .freeze(&mut flash.acr);

            hprintln!("set io").unwrap();
            let mut gpioa = device.GPIOA.split(&mut rcc.apb2);
            let mut gpiob = device.GPIOB.split(&mut rcc.apb2);
            let mut gpioc = device.GPIOC.split(&mut rcc.apb2);

            let mut delay = delay::Delay::new(cortex_m::Peripherals::take().unwrap().SYST, clocks);

            // Led
            let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

            // Configure pins for SPI
            let sck = gpiob.pb13.into_alternate_push_pull(&mut gpiob.crh);
            let miso = gpiob.pb14;
            let mosi = gpiob.pb15.into_alternate_push_pull(&mut gpiob.crh);
            hprintln!("set spi").unwrap();
            let mut spi = Spi::spi2(
                device.SPI2,
                (sck, miso, mosi),
                MODE,
                2_400_000.hz(),
                clocks,
                &mut rcc.apb1,
            );

            hprintln!("run").unwrap();
            loop {
                led.set_high();
                hprintln!(".").unwrap();
                for r in 0..6 {
                    let _ = spi.write(&convert(0, 0xff, 0x80));
                }
                delay.delay_ms(100 as u16);
                led.set_low();
                delay.delay_ms(100 as u16);
            }
        });
    }

    loop {
        continue;
    }
}

fn convert(r: u8, g: u8, b: u8) -> [u8; 9] {
    let mut out_r = 0u32;
    let mut out_g = 0u32;
    let mut out_b = 0u32;
    let mut mask = 0x80u8;
    for i in 0..8 {
        if (mask & r) == mask {
            out_r |= 0b110
        } else {
            out_r |= 0b100
        }
        if (mask & g) == mask {
            out_g |= 0b110
        } else {
            out_g |= 0b100
        }
        if (mask & b) == mask {
            out_b |= 0b110
        } else {
            out_b |= 0b100
        }
        mask = mask >> 1;
        out_r = out_r >> 3;
        out_g = out_g >> 3;
        out_b = out_b >> 3;
    }
    let r0: u8 = ((out_r >> 24) & 0xff) as u8;
    let r1: u8 = ((out_r >> 16) & 0xff) as u8;
    let r2: u8 = ((out_r >> 8) & 0xff) as u8;
    let g0: u8 = ((out_g >> 24) & 0xff) as u8;
    let g1: u8 = ((out_g >> 16) & 0xff) as u8;
    let g2: u8 = ((out_g >> 8) & 0xff) as u8;
    let b0: u8 = ((out_b >> 24) & 0xff) as u8;
    let b1: u8 = ((out_b >> 16) & 0xff) as u8;
    let b2: u8 = ((out_b >> 8) & 0xff) as u8;
    return [r0, r1, r2, g0, g1, g2, b0, b1, b2];
}
