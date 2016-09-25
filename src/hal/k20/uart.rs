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

/*!
UART configuration.
*/

use core::intrinsics::abort;

use drivers::chario::CharIO;
use hal::uart;
use hal::k20::regs::reg::*;
use hal::k20::pin::Pin;
use hal::k20::pin::Port::*;
use hal::k20::pin::Function::*;

use self::UARTPeripheral::*;
use self::UARTPinFunction::*;

/// Available UART peripherals.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq)]
pub enum UARTPeripheral {
  UART0,
  UART1,
  UART2,
}

/// UART Functions a pin can perform
#[derive(PartialEq, Clone, Copy)]
pub enum UARTPinFunction {
  /// Serial Receive
  Rx,
  /// Serial Transmit
  Tx,
  /// Request to Send
  RTS,
  /// Clear to Send
  CTS
}


/// Structure describing a UART instance.
#[derive(Clone, Copy)]
pub struct UART {
  reg: &'static reg::UART,
  peripheral: UARTPeripheral
}

/// Stop bits configuration.
/// K20 UART only supports one stop bit.
#[derive(Clone, Copy)]
pub enum StopBit {
  /// Single stop bit.
  StopBit1bit  = 0,
}

impl StopBit {
  /// Convert from number to StopBit.
  pub fn from_u8(val: u8) -> StopBit {
    match val {
      1 => StopBit::StopBit1bit,
      _ => unsafe { abort() },
    }
  }
}

impl UARTPeripheral {
  fn reg(self) -> &'static reg::UART {
    match self {
      UART0 => &reg::UART0,
      UART1 => &reg::UART1,
      UART2 => &reg::UART2,
    }
  }
}

impl UART {
  /// Returns platform-specific UART object that implements CharIO trait.
  pub fn new(peripheral: UARTPeripheral, baudrate:  u32, word_len: u8,
             parity: uart::Parity, stop_bits: u8,
             rx_pin: Pin, tx_pin: Pin) -> UART {
    let uart = UART {
      reg: peripheral.reg(),
      peripheral: peripheral,
    };

    // Enable Clock Gate for the UART
    match peripheral {
      UART0 => {SIM.scgc4.set_uart0(Sim_scgc4_uart0::ClockEnabled);},
      UART1 => {SIM.scgc4.set_uart1(Sim_scgc4_uart1::ClockEnabled);},
      UART2 => {SIM.scgc4.set_uart2(Sim_scgc4_uart2::ClockEnabled);},
    }
    
    uart.set_baud_rate(baudrate);
    uart.set_mode(reg::UART_c1_m::from_u8(word_len), parity, StopBit::from_u8(stop_bits));
    uart.set_fifo_enabled(true);

    uart.init_pin(rx_pin, Rx);
    uart.init_pin(tx_pin, Tx);
    
    uart
  }

  fn uart_clock(&self) -> u32 {
    match self.peripheral {
      UART0|UART1 => super::clocks::system_clock().expect("Clock not initialized"),
      UART2 => super::clocks::bus_clock().expect("Clock not initialized"),
    }
  }

  fn set_baud_rate(&self, baud_rate: u32) {
    let sbr: u32 = self.uart_clock() / 16 / baud_rate;
    let brfa: u32 = (2 * self.uart_clock() / baud_rate) % 32;
    (*self.reg).bdh.set_sbr((sbr >> 8) as u8);
    (*self.reg).bdl.set_sbr((sbr & 0xff) as u8);
    (*self.reg).c4.set_brfa(brfa as u8);
  }

  fn set_mode(&self, word_len: reg::UART_c1_m, parity: uart::Parity,
              _stop_bits: StopBit) {
    use hal::uart::Parity::*;
    let mut c1 = (*self.reg).c1.set_m(word_len);
    match parity {
      Disabled => {c1.set_pe(false);}
      Odd      => {c1.set_pe(true).set_pt(reg::UART_c1_pt::Odd);}
      Even     => {c1.set_pe(true).set_pt(reg::UART_c1_pt::Even);}
      Forced1  => unsafe { abort() },
      Forced0  => unsafe { abort() },
    };
    (*self.reg).c2.set_te(true).set_re(true);
  }

  fn set_fifo_enabled(&self, enabled: bool) {
    (*(self.reg)).pfifo.set_rxfe(enabled).set_txfe(enabled);
  }

  fn init_pin(&self, pin: Pin, function: UARTPinFunction) {
    let altfn = match (pin.port, pin.pin, self.peripheral, function) {
      (PortE,  0, UART1, Tx)  => AltFunction3,
      (PortE,  1, UART1, Rx)  => AltFunction3,
      (PortA,  0, UART0, CTS) => AltFunction2,
      (PortA,  1, UART0, Rx)  => AltFunction2,
      (PortA,  2, UART0, Tx)  => AltFunction2,
      (PortA,  3, UART0, RTS) => AltFunction2,
      (PortB,  2, UART0, RTS) => AltFunction3,
      (PortB,  3, UART0, CTS) => AltFunction3,
      (PortB, 16, UART0, Rx)  => AltFunction3,
      (PortB, 17, UART0, Tx)  => AltFunction3,
      (PortC,  1, UART1, RTS) => AltFunction3,
      (PortC,  2, UART1, CTS) => AltFunction3,
      (PortC,  3, UART1, Rx)  => AltFunction3,
      (PortC,  4, UART1, Tx)  => AltFunction3,
      (PortD,  0, UART2, RTS) => AltFunction3,
      (PortD,  1, UART2, CTS) => AltFunction3,
      (PortD,  2, UART2, Rx)  => AltFunction3,
      (PortD,  3, UART2, Tx)  => AltFunction3,
      (PortD,  4, UART0, RTS) => AltFunction3,
      (PortD,  5, UART0, CTS) => AltFunction3,
      (PortD,  6, UART0, Rx)  => AltFunction3,
      (PortD,  7, UART0, Tx)  => AltFunction3,
      _ => panic!("Invalid pin for function")
    };

    pin.set_function(altfn);
  }
}

impl CharIO for UART {
  fn putc(&self, value: char) {
    wait_for!(self.reg.s1.tdre());
    self.reg.d.set_re(value as u8);
  }
}

use core::fmt::{Write, Result};

impl Write for UART {
    fn write_str(&mut self, s: &str) -> Result {
        for byte in s.bytes() {
            wait_for!(self.reg.s1.tdre());
            self.reg.d.set_re(byte as u8);
        }
        Ok(())
    }
}


/// Register definitions
pub mod reg {
  use volatile_cell::VolatileCell;
  use core::ops::Drop;
  use core::intrinsics::abort;

  ioregs!(UART = {
    0x0    => reg8 bdh {  //! baud rate high
      0..4  => sbr,       //= baud rate (high 5 bits)
      6     => rxedgie,   //= RxD input active edge interrupt enable
      7     => lbkdie,    //= LIN break detect interrupt enable
    },

    0x1    => reg8 bdl {  //! baud rate low
      0..7  => sbr,       //= baud rate (low 8 bits)
    }

    0x2    => reg8 c1 {   //! Control register 1
      0     => pt {       //! parity type
        0x0 => Even,    //=   even parity
        0x1 => Odd,     //=   odd parity
      }
      1     => pe,        //= parity enable
      2     => ilt {      //! idle line type select
        0x0 => AfterStart, //= idle character bit count starts after start bit
        0x1 => AfterStop,  //= idle character bit count starts after stop bit
      }
      3     => wake {     //! receiver wakeup method select
        0x0 => IdleLineWakeup,
        0x1 => AddressMarkWakeup,
      }
      4     => m {        //! bit width mode select
        0x0 => DataBits8, //= start + 8 data bits + stop
        0x1 => DataBits9, //= start + 9 data bits + stop
      }
      5     => rsrc {     //! receiver source select
        0x0 => RxLoopback, //= select internal loop-back mode
        0x1 => SingleWire, //= single-wire UART mode
      }
      6     => uartswai, //= should UART stop in Wait mode
      7     => loops,    //= loop mode enable
    },

    0x3    => reg8 c2 {  //! Control register 2
      0     => sbk,      //= send break
      1     => rwu,      //= receiver wakeup control
      2     => re,       //= receiver enable
      3     => te,       //= transmitter enable
      4     => ilie,     //= idle line interrupt enable
      5     => rie,      //= receiver full interrupt enable
      6     => tcie,     //= transmission complete interrupt enable
      7     => tie,      //= transmitter interrupt enable
    },

    0x4    => reg8 s1 {  //! Status register 1
      0     => pf,       //= parity error flag
      1     => fe,       //= framing error flag
      2     => nf,       //= noise flag
      3     => or,       //= receiver overrun  flag
      4     => idle,     //= idle line flag
      5     => rdrf,     //= receive data register full flag
      6     => tc,       //= transmit complete flag
      7     => tdre,     //= transmit data register empty flag
    },

    0x5    => reg8 s2 {  //! Status register 2
      0     => raf: ro,  //= reciever active flag
      1     => lbkde,    //= LIN break detection enable
      2     => brk13,    //= break transmit character length
      3     => rwuid,    //= receive wakeup idle detect
      4     => rxinv,    //= receive data inversion
      5     => msbf,     //= most significant bit first
      6     => rxedgif,  //= RxD pin active edge interrupt flag
      7     => lbkdif,   //= LIN break detect interrupt flag
    },

    0x6    => reg8 c3 {  //! Control register 3
      0     => peie,     //= parity error interrupt enable
      1     => feie,     //= framing error interrupt enable
      2     => neie,     //= noise error interrupt enable
      3     => orie,     //= overrun error interrupt enable
      4     => txinv,    //= transmit data inversion
      5     => txdir,    //= transmitter pin data direction in single-wire mode
      6     => t8,       //= transmit bit 8
      7     => r8: ro,   //= receieved bit 8
    },

    0x7    => reg8 d {   //! Data register
      0..7  => re,       //= reads return the contents of the receive data register,
                         //= writes go to the transmit data register.
    },

    0x8    => reg8 ma1 { //! Match address register 1
      0..7  => ma,       //= match address
    },

    0x9    => reg8 ma2 { //! Match address register 2
      0..7  => ma,       //= match address
    },

    0xa    => reg8 c4 {  //! Control register 4
      0..4  => brfa,     //= baud rate fine adjust
      5     => m10,      //= 10-bit mode select
      6     => maen2,    //= match address 2 enable
      7     => maen1,    //= match address 1 enable
    },

    0xb    => reg8 c5 {  //! Control register 5
      5     => rdmas,    //= receiver full DMA select
      7     => tdmas,    //= transmitter DMA select
    },

    0xc    => reg8 ed {  //! Extended data register
      6     => paritye: ro, //= The current received data word has a parity error
      7     => noisy: ro,   //= The current received data word was received with noise
    },

    0xd    => reg8 modem { //! Modem register
      0     => txctse,   //= transmitter clear-to-send enable
      1     => txrese,   //= transmitter request-to-send enable
      2     => txrtspol, //= transmitter request-to-send polarity
      3     => rxrtse,   //= receiver request-to-send polarity
    },

    0xe    => reg8 ir {  //! Infrared register
      0..1  => tnp {     //! transmitter narrow pulse
        0  => PULSE_3_16,
        1  => PULSE_1_16,
        2  => PULSE_1_32,
        3  => PULSE_1_4,
      },
      2     => iren,     //= infrared enable
    },

    0x10   =>  reg8 pfifo { //! FIFO parameters
      0..2  => rxfifosize: ro, //= receive FIFO buffer depth
      3     => rxfe,           //= receive FIFO enable
      4..6  => txfifosize: ro, //= transmit FIFO buffer depth
      7     => txfe,           //= transmit FIFO enable
    },

    0x11   => reg8 cfifo { //! FIFO control
      0     => rxufe,        //= receive FIFO underflow interrupt enable
      1     => txofe,        //= transmit FIFO overflow interrupt enable
      2     => rxofe,        //= receive FIFO overflow interrupt enable
      6     => rxflush: wo,  //= flush receive FIFO
      7     => txflush: wo,  //= flush transmit FIFO
    },

    0x12   => reg8 sfifo { //! FIFO status
      0     => rxuf,         //= recieve FIFO underflow flag
      1     => txof,         //= transmit FIFO overflow flag
      2     => rxof,         //= recieve FIFO overflow flag
      6     => rxempt: ro,   //= recieve FIFO empty flag
      7     => txempt: ro,   //= transmit FIFO empty flag
    },

    0x13   => reg8 twfifo { //! Transmit FIFO watermark
      0..7  => txwater,      //= number of data words to transmit before generating
                             //= an interrupt or DMA request
    },

    0x14   => reg8 tcfifo { //! Transmit FIFO count
      0..7  => txcount,      //= number of data words in the transmit FIFO buffer.
    }

    0x15   => reg8 rwfifo { //! Receive FIFO watermark
      0..7  => rxwater,      //= number of data words to receive before generating
                             //= an interrupt or DMA request
    },

    0x16   => reg8 rcfifo { //! Receive FIFO count
      0..7  => rxcount,      //= number of data words in the receive FIFO buffer.
    },

    // FIXME(bgamari): Specialized registers omitted
  });

  impl UART_c1_m {
    /// UART data word length flag value from bit count
    pub fn from_u8(val: u8) -> UART_c1_m {
      use self::UART_c1_m::*;
      match val {
        8 => DataBits8,
        9 => DataBits9,
        _ => unsafe { abort() },
      }
    }
  }

  extern {
    #[link_name="k20_iomem_UART0"] pub static UART0: UART;
    #[link_name="k20_iomem_UART1"] pub static UART1: UART;
    #[link_name="k20_iomem_UART2"] pub static UART2: UART;
  }
}
