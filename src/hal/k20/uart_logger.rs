/*!
UART Logging adapter.

This module allows you to setup an initialized UART as a log crate Logger. You must leave the UART in a state where the Write trait on the uart will continue to work.
 */


use log;
use log::{LogRecord, LogLevelFilter, LogMetadata, LogLevel};

use super::uart;
use core::fmt::Write;

static mut LOGGING_UART: Option<uart::UART> = None;

struct UartLogger;

/// Initializes the log->UART system. The given UART must remain in a transmit-capable state after this point.
pub fn init(uart: uart::UART) {
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
            match (record.level(), super::rtc::time()) {
                (LogLevel::Info, None) => {
                    let _ = writeln!(uart, "\r\n{} - {}",
                                     record.level(), record.args());
                },
                (level, None) => {
                    let line = record.location().line();
                    let file = record.location().file();
                    let _ = writeln!(uart, "\r\n{} {}:{} - {}",
                                     level, file, line, record.args());
                },
                (level, Some(time)) => {
                    let _ = writeln!(uart, "\r\n[{time:>9}] {level} {module}:{line} - {args}",
                                     time = time, level = level,
                                     module = record.location().module_path(),
                                     line = record.location().line(),
                                     args = record.args());
                }
            }
        }
    }
}
