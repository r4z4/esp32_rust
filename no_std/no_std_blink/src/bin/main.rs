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
    let mut led = Output::new(peripherals.GPIO2, Level::High, OutputConfig::default());
    

    let delay = Delay::new();

    loop {
        led.toggle();
        delay.delay_millis(1000);
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
