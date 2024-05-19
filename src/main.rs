#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::{
    clock::{ClockControl},
    embassy::{self},
    peripherals::{Peripherals},
    prelude::*
};

use esp_backtrace as _;
use esp_println::{println};

#[embassy_executor::task]
async fn task() {
    loop {
        println!("Hello from task!");
        Timer::after(Duration::from_millis(1_000)).await;
    }
}

#[main]
async fn main(spawner: Spawner) {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    println!("Hello, world!");

    embassy::init(&clocks, esp_hal::timer::TimerGroup::new_async(peripherals.TIMG0, &clocks));

    spawner.spawn(task()).unwrap();

    loop {
        println!("Yupu!");
        Timer::after(Duration::from_millis(5_000)).await;
    }
}