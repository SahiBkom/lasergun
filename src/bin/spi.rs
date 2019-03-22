#![no_main]
#![no_std]

use cortex_m::*;
use cortex_m_rt as rt;
use embedded_hal::*;
use panic_semihosting;
use rt::{entry, exception, ExceptionFrame};
use smart_leds::{Color, SmartLedsWrite};
use stm32f1xx_hal as hal;
use stm32f1xx_hal::prelude::*;
use stm32f1xx_hal::{
    gpio,
    spi::Spi,
    spi::{Mode, Phase, Polarity},
};
// use ws2812_spi::Ws2812;
// use smart_leds_trait::{Color, SmartLedsWrite};
use embedded_hal::spi::FullDuplex;
use nb;
use nb::block;

#[entry]
fn main() -> ! {
    let dp = hal::stm32::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);
    let clocks = rcc
        .cfgr
        .use_hse(8.mhz())
        .sysclk(72.mhz())
        .pclk1(36.mhz())
        .pclk2(36.mhz())
        .hclk(72.mhz())
        .freeze(&mut flash.acr);
    let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

    const MODE: Mode = Mode {
        polarity: Polarity::IdleLow,
        phase: Phase::CaptureOnFirstTransition,
    };

    let sck = gpiob.pb13.into_alternate_push_pull(&mut gpiob.crh);
    let miso = gpiob.pb14;
    let mosi = gpiob.pb15.into_alternate_push_pull(&mut gpiob.crh);
    let mut spi = Spi::spi2(
        dp.SPI2,
        (sck, miso, mosi),
        MODE,
        3_000.khz(),
        clocks,
        &mut rcc.apb1,
    );

    loop {
        for i in 1..10u8 {
            for _ in 0..20 {
                spi.send(0);
                spi.read().ok();
            }
            let g = [
                0b10010010, 0b01001001, 0b00100100, 0b11011011, 0b01101101, 0b10110110, 0b10010010,
                0b01001001, 0b00100100,
            ];
            for t in &g {

                spi.send(*t);
                spi.read().ok();

                // spi.send(*t);
                // spi.read().ok();
            }
            for _ in 0..20 {
                spi.send(0);
                spi.read().ok();
            }
        }
        delay.delay_ms(100 as u16);
    }

    // let mut data: [Color; 3] = [Color::default(); 3];
    // let empty: [Color; 3] = [Color::default(); 3];

    // loop {
    //     data[0] = Color {
    //         r: 0,
    //         g: 0,
    //         b: 0x10,
    //     };
    //     data[1] = Color {
    //         r: 0,
    //         g: 0x10,
    //         b: 0,
    //     };
    //     data[2] = Color {
    //         r: 0x10,
    //         g: 0,
    //         b: 0,
    //     };
    //     ws.write(data.iter().cloned()).expect("write a");
    //     delay.delay_ms(1000 as u16);
    //     ws.write(empty.iter().cloned()).expect("write b");
    //     delay.delay_ms(1000 as u16);
    // }
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

#[exception]
fn DefaultHandler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}

// /// Write a single byte for ws2812 devices
// fn write_byte(spi: FullDuplex<u8>, mut data: u8) {
//     let mut serial_bits: u32 = 0;
//     for _ in 0..3 {
//         let bit = data & 0x80;
//         let pattern = if bit == 0x80 { 0b110 } else { 0b100 };
//         serial_bits = pattern | (serial_bits << 3);
//         data <<= 1;
//     }
//     block!(spi.send((serial_bits >> 1) as u8)).unwrap();
//     // Split this up to have a bit more lenient timing
//     for _ in 3..8 {
//         let bit = data & 0x80;
//         let pattern = if bit == 0x80 { 0b110 } else { 0b100 };
//         serial_bits = pattern | (serial_bits << 3);
//         data <<= 1;
//     }
//     // Some implementations (stm32f0xx-hal) want a matching read
//     // We don't want to block so we just hope it's ok this way
//     spi.read().ok();
//     block!(spi.send((serial_bits >> 8) as u8)).unwrap();
//     spi.read().ok();
//     block!(spi.send(serial_bits as u8)).unwrap();
//     spi.read().ok();
// }

// fn flush(spi: FullDuplex<u8>) {
//     for _ in 0..20 {
//         block!(spi.send(0)).unwarp();
//         spi.read().ok();
//     }
// }

// fn led_write(spi: FullDuplex<u8>, r: u8, g: u8, b: u8) {
//     flush();


//     write_byte(g);
//     write_byte(r);
//     write_byte(b);
//     flush();
// }
