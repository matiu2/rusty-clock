#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_time::{Delay, Duration, Ticker};
use uln2003::{Direction, StepperMotor};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Hello World!");
    // For debugging, we'll use every second so we can see the action
    let mut ticker = Ticker::every(Duration::from_secs(60));
    // The pins used to control the stepper motor
    let p = embassy_rp::init(Default::default());

    let mut motor = if cfg!(debug_assertions) {
        let pin_1 = Output::new(p.PIN_16, Level::Low);
        let pin_2 = Output::new(p.PIN_17, Level::Low);
        let pin_3 = Output::new(p.PIN_18, Level::Low);
        let pin_4 = Output::new(p.PIN_19, Level::Low);
        uln2003::ULN2003::new(pin_1, pin_2, pin_3, pin_4, Some(Delay))
    } else {
        let pin_1 = Output::new(p.PIN_2, Level::Low);
        let pin_2 = Output::new(p.PIN_3, Level::Low);
        let pin_3 = Output::new(p.PIN_4, Level::Low);
        let pin_4 = Output::new(p.PIN_5, Level::Low);
        uln2003::ULN2003::new(pin_1, pin_2, pin_3, pin_4, Some(Delay))
    };

    loop {
        every_minute(&mut motor).await;
        ticker.next().await;
    }
}

/// How many steps to drive the motor per minute
const fn steps_per_minute() -> u32 {
    // Arrived at through trial and error
    513
}

/// Moves the minute hand
async fn rotate<T: StepperMotor>(motor: &mut T, steps: u32, direction: Direction) {
    motor.set_direction(direction);
    motor.step_for(steps as i32, 1).unwrap();
}

/// The actual code to run each minute
async fn every_minute<T: StepperMotor>(motor: &mut T) {
    info!("Advancing...");
    rotate(motor, steps_per_minute(), Direction::Normal).await;
    motor.stop().unwrap();
}
