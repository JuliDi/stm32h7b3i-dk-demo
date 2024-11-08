#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32::{exti::ExtiInput, gpio::{AnyPin, Input, Level, Output, Pin, Pull, Speed}};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::task]
async fn blink(pin: AnyPin) {
    let mut led = Output::new(pin, Level::Low, Speed::VeryHigh);

    loop {
        // Timekeeping is globally available, no need to mess with hardware timers.
        led.set_high();
        Timer::after_millis(150).await;
        led.set_low();
        Timer::after_millis(150).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let mut led_3 = Output::new(p.PG11, Level::Low, Speed::VeryHigh);

    // Spawned tasks run in the background, concurrently.
    spawner.spawn(blink(p.PG2.degrade())).unwrap();

    // Using PG2 here after moving it into the task would give a compiler error!
    // You can check by uncommenting this line:
    //let mut led_2 = Output::new(p.PG2, Level::Low, Speed::VeryHigh);

    let button_input = Input::new(p.PC13, Pull::Down);
    let mut button = ExtiInput::new(button_input, p.EXTI13);

    loop {
        // Asynchronously wait for GPIO events, allowing other tasks
        // to run, or the core to sleep.
        button.wait_for_low().await;
        defmt::info!("Button pressed!");
        led_3.set_low();

        button.wait_for_high().await;
        defmt::info!("Button released!");
        led_3.set_high();
    }
}
