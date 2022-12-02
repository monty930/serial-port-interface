/**
 ******************************************************************************
 * file           : lib.rs
 * brief          : Library handling serial port connection and concurrent
 *                  standard input handling.
 ******************************************************************************
 */

#[allow(unused_imports)]
use debug_print::{debug_eprint, debug_eprintln, debug_print, debug_println};
use serialport::SerialPort;
use std::error::Error;
use std::io::{self, Read, Write};
use std::panic::panic_any;
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time;
use std::time::Duration;

#[allow(dead_code)]
const HALF_SEC: Duration = time::Duration::from_millis(500);

#[allow(dead_code)]
const MINI_BREAK: Duration = time::Duration::from_millis(10);

const TIMEOUT: Duration = time::Duration::from_millis(10);

/**
 * Function calls serial port (and standard input) handler.
 * It returns 1 if execution ended with errors or 0 otherwise.
 * See: port_handler function.
 */
pub fn run_port_handler(port_name: &str, baud_rate: u32) -> i32 {
    println!("Running port handler. Press Ctrl+D to end process.");
    println!("Port name: {}, baud rate: {}", port_name, baud_rate);

    match port_handler(port_name, baud_rate) {
        Ok(_) => 0,
        Err(e) => {
            println!("Error: {}", e);
            1
        }
    }
}

/**
 * Function writes given data to the given serial port.
 * The same serial port connection is being returned.
 */
fn port_write(to_send: &String, mut port: Box<dyn SerialPort>) -> Box<dyn SerialPort> {
    debug_println!("Writing: {} to serial port.", to_send);
    match port.write(to_send.as_bytes()) {
        Ok(_) => (),
        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
        Err(e) => panic!("Error while writing data to the port: {}", e),
    };
    debug_println!("sent.");
    port
}

/**
 * Function takes serial port connection and reads data
 * from it. Then redirects data to standard output. The same
 * serial port connection is being returned.
 */
fn port_read(mut port: Box<dyn SerialPort>) -> Box<dyn SerialPort> {
    let mut serial_buf: Vec<u8> = vec![0; 1000];
    match port.read(serial_buf.as_mut_slice()) {
        Ok(t) => io::stdout().write_all(&serial_buf[..t]).unwrap(),
        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
        Err(e) => eprintln!("{:?}", e),
    }
    port
}

/**
 * Checks if data from standard input is being transmited
 * and handles its receipt. The data is pushed to to_send
 * string which is returned. idx value is set to data size.
 * ready is set to true if 'Return' value (end of line)
 * has been encountered, false otherwise. connection_broke
 * is set to true if 'EOF' has been encountered, false
 * otherwise.
 */
fn input_receive(
    mut to_send: String,
    input_channel: Receiver<u8>,
    mut idx: i32,
    mut ready: bool,
) -> (String, Receiver<u8>, i32, bool, bool) {
    let mut connection_broke: bool = false;

    match input_channel.try_recv() {
        Ok(x) => {
            debug_println!("Received {} from thread.", x);

            if idx == 0 {
                to_send = String::from("");
            }
            if (x as char) != '\n' {
                to_send.push(x as char);
            } else {
                ready = true;
            }

            idx += 1;
        }
        Err(mpsc::TryRecvError::Empty) => (),
        Err(mpsc::TryRecvError::Disconnected) => {
            debug_println!("Input service broke.");
            connection_broke = true;
        }
    }

    (to_send, input_channel, idx, ready, connection_broke)
}

/**
 * Function opens serial port connection with the given
 * port name and bound rate. It continuously checks
 * if there is data to read on serial port or standard
 * input, along with writing to the serial port data
 * received on standard input.
 */
fn port_handler(port_name: &str, baud_rate: u32) -> Result<(), Box<dyn Error>> {
    let mut port = serialport::new(port_name, baud_rate)
        .timeout(TIMEOUT)
        .open()
        .map_err(|ref e| format!("Error. Failed to connect to serial port: {}.", e))?;

    debug_println!("Connected.");

    let mut input_channel = input_handler();
    let mut to_send: String = String::from("");
    let mut idx = 0;
    let mut ready = false;

    loop {
        port = port_read(port);

        if ready {
            port = port_write(&to_send, port);
            idx = 0;
            ready = false;
        }

        let connection_broke;
        (to_send, input_channel, idx, ready, connection_broke) =
            input_receive(to_send, input_channel, idx, ready);
        if connection_broke {
            break;
        }
    }

    Ok(())
}

/**
 * Concurrently checks if there is data on standard input and transmits
 * it to the main thread.
 */
fn input_handler() -> mpsc::Receiver<u8> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        debug_println!("Thread created.");
        let mut buffer = [0; 32];
        loop {
            match io::stdin().read(&mut buffer) {
                Ok(0) => {
                    debug_println!("EOF. Stopping standard input connection.");
                    drop(tx);
                    break;
                }
                Ok(s) => {
                    debug_println!("Received {} chars on std input.", s);
                    for i in 0..s {
                        tx.send(buffer[i]).unwrap()
                    }
                }
                Err(e) => panic_any(e),
            }
        }
    });

    rx
}
