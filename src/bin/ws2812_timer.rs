#![no_main]
#![no_std]

use cortex_m::*;
use cortex_m_rt as rt;
use embedded_hal::spi::FullDuplex;
use embedded_hal::*;
use nb;
use nb::block;
use panic_semihosting;
use rt::{entry, exception, ExceptionFrame};
use smart_leds::{Color, SmartLedsWrite};
use stm32f1xx_hal as hal;
use stm32f1xx_hal::prelude::*;
use stm32f1xx_hal::{
    delay::Delay,
    gpio,
    spi::Spi,
    spi::{Mode, Phase, Polarity},
    timer::Timer,
};
use ws2812_timer_delay::Ws2812;

#[entry]
fn main() -> ! {
    let dp = hal::stm32::Peripherals::take().expect("dp");
    let cp = cortex_m::Peripherals::take().expect("cp");
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

    // Get delay provider
    let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

    /* (Re-)configure PA7 as output */
    let mut ws_data_pin = cortex_m::interrupt::free(move |cs| {
        gpioa
            .pa7
            .into_push_pull_output_with_state(&mut gpioa.crl, stm32f1xx_hal::gpio::State::High)
    });

    let timer = Timer::tim3(dp.TIM3, 3.mhz(), clocks, &mut rcc.apb1);

    let mut ws = Ws2812::new(timer, &mut ws_data_pin);
    let mut data: [Color; 3] = [Color::default(); 3];
    let empty: [Color; 3] = [Color::default(); 3];

    data[0] = Color {
        r: 0,
        g: 0,
        b: 0x10,
    };
    data[1] = Color {
        r: 0,
        g: 0x10,
        b: 0,
    };
    data[2] = Color {
        r: 0x10,
        g: 0,
        b: 0,
    };

    loop {
        ws.write(data.iter().cloned()).expect("write1");
        delay.delay_ms(10 as u16);
        ws.write(empty.iter().cloned()).expect("write2");
        delay.delay_ms(10 as u16);
    }
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

#[exception]
fn DefaultHandler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
