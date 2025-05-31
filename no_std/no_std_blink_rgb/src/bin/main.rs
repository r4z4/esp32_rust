#![no_std]
#![no_main]

// You'll need a panic handler e.g. `use esp_backtrace as _;`
use esp_hal::{
    clock::CpuClock,
    delay::Delay,
    gpio::{Io, Level, Output, OutputConfig},
    main,
};

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // Set GPIO0 as an output, and set its state high initially.
    let mut red_led = Output::new(peripherals.GPIO2, Level::Low, OutputConfig::default());
    let mut blue_led = Output::new(peripherals.GPIO5, Level::Low, OutputConfig::default());
    let mut green_led = Output::new(peripherals.GPIO4, Level::Low, OutputConfig::default());
    

    let delay = Delay::new();

    loop {
        red_led.toggle();
        delay.delay_millis(200);
        blue_led.toggle();
        delay.delay_millis(200);
        green_led.toggle();
        delay.delay_millis(200);
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
