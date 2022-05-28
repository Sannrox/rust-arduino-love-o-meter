/*!
 * Show readouts of all ADC channels.
 *
 * This example displays values for all ADC channels over the serial console.  During startup, it
 * also displays the values for Vbandgap, GND, and a readout of the MCU's temperature sensor.  For
 * the meanings of these values, please reference the ATmega328P datasheet.
 *
 * Connections
 * -----------
 *  - `A0` - `A5`: Connect analog voltages as you like to see them read out.
 */
#![no_std]
#![no_main]
extern crate arduino_hal;
use arduino_hal::prelude::*; // trait ResultVoidExt
extern crate panic_halt;
extern crate ufmt;
extern crate ufmt_float;
use arduino_hal::adc;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());


    let led1 = pins.d2.into_output();
    let led2 = pins.d3.into_output();
    let led3 = pins.d4.into_output();
    let a0 = pins.a0.into_analog_input(&mut adc);
    loop {
        let values = a0.analog_read(&mut adc);
        let voltage = values/1024 * 5;
        let tmp = voltage * 100;

        ufmt::uwrite!(&mut serial, "Sensor value: {}", values).void_unwrap();
        ufmt::uwriteln!(&mut serial, "Volts: {}", voltage).void_unwrap();
        ufmt::uwriteln!(&mut serial, "Temp: {}", tmp).void_unwrap();

        arduino_hal::delay_ms(1000);


    }
}

