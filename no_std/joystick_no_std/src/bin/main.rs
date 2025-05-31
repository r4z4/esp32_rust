#![no_std]
#![no_main]

// You'll need a panic handler e.g. `use esp_backtrace as _;`

use esp_hal::{
    main, peripherals, timer::{timg::TimerGroup, OneShotTimer}, Blocking
};

// use atomic_enum::atomic_enum;
use esp_println::println;

#[main]
fn main() -> ! {


    let peripherals = esp_hal::init(esp_hal::Config::default());
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let mut one_shot = OneShotTimer::new(timg0.timer0);

    let urx = peripherals.GPIO25;
    let ury = peripherals.GPIO26;
    let adc_peripheral = peripherals.ADC2;
    let mut adc2_config = esp_hal::analog::adc::AdcConfig::new();
    let mut urx_pin =
        adc2_config.enable_pin(urx, esp_hal::analog::adc::Attenuation::_0dB);
    let mut ury_pin =
        adc2_config.enable_pin(ury, esp_hal::analog::adc::Attenuation::_0dB);
    let mut adc2 = esp_hal::analog::adc::Adc::new(adc_peripheral, adc2_config);

    loop {
        let urx_pin_value: u16 =
            nb::block!(adc2.read_oneshot(&mut urx_pin)).unwrap();
        let ury_pin_value: u16 =
            nb::block!(adc2.read_oneshot(&mut ury_pin)).unwrap();

        println!("X: {} || Y: {}", urx_pin_value, ury_pin_value);


        one_shot.delay_millis(500);
    }
}


#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
