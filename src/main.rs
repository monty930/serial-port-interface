/**
 ******************************************************************************
 * file           : main.rs
 * brief          : Usage: Serial device should be connected to 'PORT_NAME' port
 *                  at 'BAUD_RATE' baud rate.
 *                  Program handles reading from serial port and prints received
 *                  data to standard output. Concurrently, data from standard input
 *                  is read and sent by serial port. The data is sent after pressing
 *                  the 'Return' key. The 'Return' key is not being sent.
 ******************************************************************************
 */

#[allow(unused_imports)]
use debug_print::{debug_eprint, debug_eprintln, debug_print, debug_println};
use serial::run_port_handler;

const PORT_NAME: &str = "/dev/ttyACM0";
const BAUD_RATE: u32 = 115200;

fn main() {
    let exit_code = run_port_handler(PORT_NAME, BAUD_RATE);
    debug_println!("Ending process with exit code: {}.", exit_code);
    std::process::exit(exit_code);
}
