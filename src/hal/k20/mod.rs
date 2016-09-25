// Zinc, the bare metal stack for rust.
// Copyright 2014 Ben Gamari <bgamari@gmail.com>
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

//! HAL for Freescale Kinetis K20.

/// Initialization functions for bringing up a k20 chip into operation
pub mod init;

/// Pin function and control settings for k20 pins
pub mod pin;
/// K20 UART/Serial interface
pub mod uart;

#[cfg(feature = "loglib")]
/// UART Debug Logging module
pub mod uart_logger;

/// K20 Software watchdog timer interface
pub mod watchdog;
/// K20 SPI interface
pub mod spi;
/// K20 register definitions
pub mod regs;

/// K20 Real Time Clock interface
pub mod rtc;

/// K20 Oscillator and Clocks interface
pub mod clocks;

/// K20 USB Tranceiver
pub mod usb;

/// K20 Peripheral Interrupt Timer
pub mod pit;
