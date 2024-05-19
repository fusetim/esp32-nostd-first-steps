#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use core::fmt::Write;

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::{
    clock::ClockControl,
    embassy,
    gpio::IO,
    peripherals::{Peripherals, self},
    prelude::*, uart::{self, config::AtCmdConfig, TxRxPins, Uart}, Async
};
use esp_backtrace as _;
use esp_println::{println};

const AT_CMD: u8 = '\n' as u8;
const READ_BUF_SIZE: usize = 64;
const BAUDERATE: u32 = 115200;

#[embassy_executor::task]
async fn task() {
    loop {
        println!("Hello from task!");
        Timer::after(Duration::from_millis(1_000)).await;
    }
}

#[embassy_executor::task]
async fn uart_task(mut uart: Uart<'static, peripherals::UART0, Async>) {
    let mut buf = [0u8; READ_BUF_SIZE];
    loop {
        match uart.read_async(&mut buf).await {
            Ok(len) => {
                if len == 0 {
                    continue;
                }
                let s = core::str::from_utf8(&buf).expect("Failed to convert to string").trim();
                println!("Received: {}", s);
                uart.write_fmt(format_args!("echo: {}\n", s)).unwrap();
            },
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
        println!("Received!");
    }
}

#[main]
async fn main(spawner: Spawner) {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut uart0 = Uart::new_async_with_config(
        peripherals.UART0, 
        uart::config::Config::default().baudrate(BAUDERATE),
        Some(TxRxPins::new_tx_rx(io.pins.gpio1, io.pins.gpio3)),
        &clocks
    );
    let at_cfg = AtCmdConfig::new(None, None, None, AT_CMD, None);
    uart0.set_at_cmd(at_cfg);
    uart0
        .set_rx_fifo_full_threshold(READ_BUF_SIZE as u16)
        .unwrap();

    println!("Hello, world!");
    uart0.write_str("Hello, UART0!\r\n").unwrap();

    embassy::init(&clocks, esp_hal::timer::TimerGroup::new_async(peripherals.TIMG0, &clocks));

    spawner.spawn(task()).unwrap();
    spawner.spawn(uart_task(uart0)).unwrap();

    loop {
        println!("Yupi!");
        Timer::after(Duration::from_millis(5_000)).await;
    }
}