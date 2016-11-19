// Zinc, the bare metal stack for rust.
// Copyright 2016 Geoff Cant 'archaelus' <nem@erlang.geek.nz>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/*!
UART Logging adapter.

This module allows you to setup an initialized UART as a log crate Logger. You must leave the UART in a state where the Write trait on the uart will continue to work.
 */


use log;
use log::{LogRecord, LogLevelFilter, LogMetadata, LogLevel};

use super::uart;
use core::fmt::Write;
use core::fmt::Arguments;
use hal::cortex_m4::systick::get_current;

/// f
pub static mut LOGGING_UART: Option<uart::Uart> = None;

struct UartLogger;

/// Initializes the log->UART system. The given UART must remain in a transmit-capable state after this point.
pub fn init(uart: uart::Uart) {
    unsafe {
        LOGGING_UART = Some(uart);
        log::set_logger_raw(|max_log_level| {
            max_log_level.set(LogLevelFilter::Trace);
            static LOGGER: UartLogger = UartLogger{};
            &LOGGER
        })
    }.expect("Couldn't setup the debug uart logger.");
}

impl log::Log for UartLogger {
    fn enabled(&self, _metadata: &LogMetadata) -> bool {
        true
    }

    fn log(&self, record: &LogRecord) {
        if let &mut Some(ref mut uart) = unsafe { &mut LOGGING_UART } {
            match (record.level(), Some(get_current()) /* super::rtc::time() */ ) {
                (LogLevel::Info, None) => {
                    let _ = write!(uart, "\r\n{} - {}",
                                     record.level(), record.args());
                },
                (level, None) => {
                    let line = record.location().line();
                    let file = record.location().file();
                    let _ = write!(uart, "\r\n{} {}:{} - {}",
                                     level, file, line, record.args());
                },
                (level, Some(time)) => {
                    let _ = write!(uart, "\r\n[{time:>9}] :{line} - {args}",
                                     time = time,
                                     line = record.location().line(),
                                     args = record.args());
/*                    let _ = write!(uart, "\r\n[{time:>9}] {level:5} {module}:{line} - {args}",
                                     time = time, level = level,
                                     module = record.target(),
                                     line = record.location().line(),
                                     args = record.args());
*/                }
            }
        }
    }
}

#[cfg(all(not(test), not(feature = "test")))]
#[lang="panic_fmt"]
extern "C" fn panic_fmt(msg: Arguments, file: &'static str, line: u32) -> ! {
    error!(target: "PANIC! ", "{} ({}:{})", msg, file, line);
    loop {}
}
