#![no_std]
#![no_main]

use core::convert::Infallible;

use embedded_hal::{blocking::serial::Write, serial::Read};

use teensy4_bsp as bsp;
use teensy4_panic as _;

use bsp::{
    board,
    hal::{lpuart::ReadFlags, timer::Blocking},
};

use kondo_ics::commnd_generator::{CommandGenerator, Position};

#[bsp::rt::entry]
fn main() -> ! {
    let board::Resources {
        pit,
        pins,
        mut gpio4,
        lpuart6,
        ..
    } = board::t41(board::instances());

    let mut delay = Blocking::<_, { board::PERCLK_FREQUENCY }>::from_pit(pit.0);

    let mut enable_pin = gpio4.output(pins.p2);

    let mut lpuart6: board::Lpuart6 = board::lpuart(lpuart6, pins.p1, pins.p0, 115200);

    lpuart6.disable(|lpuart| {
        lpuart.set_parity(Some(bsp::hal::lpuart::Parity::Even));
    });

    enable_pin.clear();
    delay.block_ms(1000);

    loop {
        // let cmd = CommandGenerator::unsafe_set_position(0, 7500);
        let cmd = CommandGenerator::set_position(0, Position::new_num(7500).unwrap()).unwrap();
        send_servo(&mut delay, &mut lpuart6, &mut enable_pin, &cmd, 3);
        delay.block_ms(3000);
        // let cmd = CommandGenerator::unsafe_set_position(0, 5500);
        let cmd = CommandGenerator::set_position(0, Position::new_num(5500).unwrap()).unwrap();
        send_servo(&mut delay, &mut lpuart6, &mut enable_pin, &cmd, 3);
        delay.block_ms(3000);
        // let cmd = CommandGenerator::unsafe_set_position(0, 9500);
        let cmd = CommandGenerator::set_position(0, Position::new_num(9500).unwrap()).unwrap();
        send_servo(&mut delay, &mut lpuart6, &mut enable_pin, &cmd, 3);
        delay.block_ms(3000);
    }
}

fn send_servo<T, D, P, E>(
    delay: &mut D,
    uart: &mut T,
    enable_pin: &mut P,
    cmd: &[u8],
    read_size: u8,
) where
    T: Write<u8, Error = Infallible> + Read<u8, Error = ReadFlags>,
    D: embedded_hal::blocking::delay::DelayUs<u32>,
    P: embedded_hal::digital::v2::OutputPin<Error = E>,
    E: core::fmt::Debug,
{
    enable_pin.set_high().unwrap();

    delay.delay_us(1000);

    uart.bwrite_all(&cmd).unwrap();

    delay.delay_us(1000);

    enable_pin.set_low().unwrap();

    for _ in 0..read_size {
        match uart.read() {
            Ok(byte) => {
                log::info!("{byte}");
            }
            Err(_) => {}
        };
    }
}
