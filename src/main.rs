#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use embedded_hal::serial::Read;


#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    avr_device::interrupt::disable();

    // Get the peripherals so we can access serial and the LED.
    //
    // SAFETY: Because main() already has references to the peripherals this is an unsafe
    // operation - but because no other code can run after the panic handler was called,
    // we know it is okay.
    let peripherals = unsafe { arduino_hal::Peripherals::steal() };
    let pins = arduino_hal::pins!(peripherals);
    let mut serial = arduino_hal::default_serial!(peripherals, pins, 57600);

    // Print out panic location
    ufmt::uwriteln!(&mut serial, "Firmware panic!\r").void_unwrap();
    if let Some(loc) = info.location() {
        ufmt::uwriteln!(
            &mut serial,
            "  At {}:{}:{}\r",
            loc.file(),
            loc.line(),
            loc.column(),
        )
        .void_unwrap();
    }

    // Blink LED rapidly
    let mut led = pins.d13.into_output();
    loop {
        led.toggle();
        arduino_hal::delay_ms(100);
    }
}



#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    ufmt::uwriteln!(&mut serial, "Hello from Arduino!\r").void_unwrap();

    loop {
        let b = nb::block!(serial.read()).void_unwrap();
        ufmt::uwriteln!(&mut serial, "Got {}!\r", b).void_unwrap();

        // panic!("Testing to panic.");
    }
}
