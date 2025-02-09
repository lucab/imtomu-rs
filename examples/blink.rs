#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;

use tomu::{prelude::*, Tomu};

#[entry]
fn main() -> ! {
    let mut tomu = Tomu::take().unwrap();

    // constrain CMU and split into device clocks
    // so we can enable gpio with its owned clock
    let clk_mgmt = tomu.CMU.constrain().split();
    let gpio = tomu.GPIO.split(clk_mgmt.gpio).pins();

    // create tomu's led instance from gpio pin
    let leds = led::LEDs::new(gpio.pa0.into(), gpio.pb7.into());

    let mut red = leds.red;
    let mut green = leds.green;

    let mut delay = systick::SystickDelay::new(
        tomu.SYST.constrain(),
        clk_mgmt.hfcoreclk,
    );

    tomu.watchdog.disable();

    loop {
        red.on();
        delay.delay_ms(500u16);
        green.on();
        delay.delay_ms(500u16);
        red.off();
        delay.delay_ms(500u16);
        green.off();
        delay.delay_ms(500u16);
    }
}
