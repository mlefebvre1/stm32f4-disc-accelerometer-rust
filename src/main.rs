#![no_main]
#![no_std]

use crate::board::{
    hal::pac,
    hal::prelude::*,
    led::{LedColor, Leds},
};

use panic_halt as _;
use stm32f407g_disc as board;

use cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;

use accelerometer::Accelerometer;

#[entry]
fn main() -> ! {
    if let (Some(p), Some(c)) = (pac::Peripherals::take(), Peripherals::take()) {
        let gpiod = p.GPIOD.split();
        let gpioa = p.GPIOA.split();
        let gpioe = p.GPIOE.split();

        // Constrain clock registers
        let rcc = p.RCC.constrain();

        // Configure clock to 168 MHz (i.e. the maximum) and freeze it
        let clocks = rcc.cfgr.sysclk(168.mhz()).freeze();

        // Initialize on-board LEDs
        let mut leds = Leds::new(gpiod);

        let mut accelerometer =
            board::accelerometer::Accelerometer::new(gpioa, gpioe, p.SPI1, clocks);

        loop {
            let acceleration = accelerometer.accel_norm().unwrap();
            for led in leds.iter_mut() {
                led.off();
            }
            // Units are in G
            if acceleration.x < -0.5 {
                leds[LedColor::Green].on();
            }

            if acceleration.x > 0.5 {
                leds[LedColor::Red].on();
            }

            if acceleration.y < -0.5 {
                leds[LedColor::Blue].on();
            }
            if acceleration.y > 0.5 {
                leds[LedColor::Orange].on();
            }
        }
    }

    loop {
        continue;
    }
}
