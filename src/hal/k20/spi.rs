// Zinc, the bare metal stack for rust.
// Copyright 2016 Geoff Cant <nem@erlang.geek.nz>
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
SPI configuration.
*/

// use core::intrinsics::abort;

use hal::k20::regs::reg;
use hal::k20::regs::reg::{Spi,
                          SIM,
                          Sim_scgc6_spi0,
                          Sim_scgc6_spi1,
                          Spi_ctar_ctar_br,
                          Spi_ctar_ctar_dbr,
                          Spi_ctar_ctar_pbr,
                          Spi_ctar_ctar_cpol,
                          Spi_ctar_ctar_cpha,
                          Spi_ctar_ctar_lsbfe,
                          Spi_ctar_ctar_Update,
                          Spi_ctar_ctar,
                          Spi_mcr_mstr,
                          Spi_mcr_halt,
                          Spi_mcr_mdis,
                          Spi_pushr_cont,
                          Spi_pushr_ctas,
                          Spi_pushr_eoq,
                          Spi_pushr_ctcnt,
                          Spi_pushr_Update,
                          Spi_sr_eoqf,
                          Spi_sr_tfff
};
use hal::k20::pin::Pin;
use hal::k20::pin::Function::*;
use hal::k20::pin::Port::*;
use hal::k20::clocks;
use self::SPIPeripheral::*;
use self::SPIMode::*;
use self::SPIRole::*;
use self::SPISignificantBit::*;
use self::SPIPinFunction::*;
use self::BaudRate::*;

/// The various SPI Peripherals
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum SPIPeripheral {
    /// First SPI Controller
    SPI0,
    /// Second SPI Controller
    SPI1
}

/// SPI Polarity and Phase (SPI Mode settings)
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum SPIMode {
    /// CPOL = 0, CPHA = 0
    Mode0,
    /// CPOL = 0, CPHA = 1
    Mode1,
    /// CPOL = 1, CPHA = 0
    Mode2,
    /// CPOL = 1, CPHA = 1
    Mode3
}

/// SPI Role (Master or Slave)
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum SPIRole {
    /// SPI Master
    Master,
    /// SPI Slave
    Slave
}

/// SPI Significant Bit
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum SPISignificantBit {
    /// most significant bit first
    MSB,
    /// least significant bit first
    LSB
}

impl SPIPeripheral {
    fn reg(self) -> &'static Spi {
        match self {
            SPIPeripheral::SPI0 => &reg::SPI0,
            SPIPeripheral::SPI1 => &reg::SPI1,
        }
    }
}

/// UART Functions a pin can perform
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum SPIPinFunction {
    /// SPI Clock
    SCK,
    /// SPI Data Out
    SOUT,
    /// SPI Data In
    SIN,
    /// Chip Select 0
    PCS0,
    /// Chip Select 1
    PCS1,
    /// Chip Select 2
    PCS2,
    /// Chip Select 3
    PCS3,
    /// Chip Select 4
    PCS4,
}

/// A generic SPI controller
#[derive(Clone, Copy)]
pub struct SPI {
    reg: &'static Spi,
    peripheral: SPIPeripheral,
    role: SPIRole,
    setup: SPISetup
}

use core::default::Default;
impl Default for SPI {
    fn default() -> SPI {
        SPI {
            reg: &reg::SPI0,
            peripheral: SPI0,
            role: Master,
            setup: Default::default()
        }
    }
}


/// SPI Baud Rate 
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum BaudRate {
    /// Normal Rate
    Normal,
    /// Double Rate
    Doubled
}

/// SPI Transmission/Receive Configuration (baud rate, mode, msb/lsb/...)
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct SPISetup {
    rate: BaudRate,
    mode: SPIMode,
    sb: SPISignificantBit,
}

impl Default for SPISetup {
    fn default() -> SPISetup {
        SPISetup {
            rate: Normal,
            mode: Mode0,
            sb: MSB,
        }
    }
}

impl SPISetup {
    fn update_ctar(&self, mut ctar: Spi_ctar_ctar_Update, frame_size: u32) {
        let dbr = match self.rate {
            Normal => Spi_ctar_ctar_dbr::Normal,
            Doubled => Spi_ctar_ctar_dbr::Doubled
        };
        let cpol = match self.mode {
            Mode0 => Spi_ctar_ctar_cpol::InactiveLow,
            Mode1 => Spi_ctar_ctar_cpol::InactiveLow,
            Mode2 => Spi_ctar_ctar_cpol::InactiveHigh,
            Mode3 => Spi_ctar_ctar_cpol::InactiveHigh,
        };
        let cpha = match self.mode {
            Mode0 => Spi_ctar_ctar_cpha::CaptureLeading,
            Mode1 => Spi_ctar_ctar_cpha::ChangeLeading,
            Mode2 => Spi_ctar_ctar_cpha::CaptureLeading,
            Mode3 => Spi_ctar_ctar_cpha::ChangeLeading,
        };
        let lsbfe = match self.sb {
            MSB => Spi_ctar_ctar_lsbfe::MSBFirst,
            LSB => Spi_ctar_ctar_lsbfe::LSBFirst,
        };

        let fmsz:u32 = frame_size as u32 - 1;
        assert!(3 <= fmsz && fmsz <= 15);

        ctar
            .set_dbr(Spi_ctar_ctar_dbr::Normal)
            .set_cpol(cpol)
            .set_cpha(cpha)
            .set_lsbfe(lsbfe)
            .set_fmsz(fmsz)
            .set_pbr(Spi_ctar_ctar_pbr::Scale5) // Baud Rate scale static hacks for now
            .set_br(Spi_ctar_ctar_br::Scale8);
    }

    /// Returns the baud rate specified by this ctar
    pub fn baud_rate(&self, ctar: Spi_ctar_ctar) -> u32 {
        let fsys = clocks::bus_clock().expect("Bus clock not set.");
        let dbr = ctar.dbr() as u32;
        let pbr = ctar.pbr() as u32;
        let br = ctar.br() as u32;

        (fsys * dbr) / (br * pbr)
    }
}

impl SPI {
    /// Produce a new SPI controller in write only mode (needs only clk and sout pins)
    pub fn write_only(peripheral: SPIPeripheral,
                      clk_pin: Pin, sout_pin: Pin) -> SPI {
        let spi = SPI {
            reg: peripheral.reg(),
            peripheral: peripheral,
            ..Default::default()
        };

        let mstr = match spi.role {
            Slave => unimplemented!(),
            Master => Spi_mcr_mstr::Master,
        };

        spi.init_clock_gate();

        spi.reg.mcr.ignoring_state()
            .set_mstr(mstr)
            .set_mdis(Spi_mcr_mdis::EnableClocks)
            .clear_clr_txf()
            .clear_clr_rxf()
            .set_halt(Spi_mcr_halt::Stop);

        spi.default_ctars();

        spi.init_pin(clk_pin, SCK);
        spi.init_pin(sout_pin, SOUT);

        spi
    }

    fn init_clock_gate(&self) {
        match self.peripheral {
            SPI0 => {SIM.scgc6.set_spi0(Sim_scgc6_spi0::ClockEnabled);},
            SPI1 => {SIM.scgc6.set_spi1(Sim_scgc6_spi1::ClockEnabled);},
        }
    }

    fn default_ctars(&self) {
        // Setup CTAR0 for 8 bit transfers
        self.setup.update_ctar(self.reg.ctar[0].ctar.ignoring_state(), 8);
        // Setup CTAR1 for 16 bit transfers
        self.setup.update_ctar(self.reg.ctar[1].ctar.ignoring_state(), 16);
    }

    /// Return the currently configured Baud-rate in Hz.
    pub fn baud_rate(&self) -> u32 {
        self.setup.baud_rate(self.reg.ctar[1].ctar)
    }
    
    fn init_pin(&self, pin: Pin, function: SPIPinFunction) {
        let altfn = match (pin.port, pin.pin, self.peripheral, function) {
            (PortC, 0, SPI0, PCS4) => AltFunction2,
            (PortC, 1, SPI0, PCS3) => AltFunction2,
            (PortC, 2, SPI0, PCS2) => AltFunction2,
            (PortC, 3, SPI0, PCS1) => AltFunction2,
            (PortC, 4, SPI0, PCS0) => AltFunction2,
            (PortC, 5, SPI0, SCK)  => AltFunction2,
            (PortC, 6, SPI0, SOUT) => AltFunction2,
            (PortC, 7, SPI0, SIN)  => AltFunction2,

            (PortD, 0, SPI0, PCS0) => AltFunction2,
            (PortD, 1, SPI0, SCK)  => AltFunction2,
            (PortD, 2, SPI0, SOUT) => AltFunction2,
            (PortD, 3, SPI0, SIN)  => AltFunction2,
            (PortD, 4, SPI0, PCS1) => AltFunction2,
            (PortD, 5, SPI0, PCS2) => AltFunction2,
            (PortD, 6, SPI0, PCS3) => AltFunction2,

            (PortE, 0, SPI1, PCS1)  => AltFunction2,
            (PortE, 1, SPI1, SOUT)  => AltFunction2,
            (PortE, 1, SPI1, SIN)   => AltFunction7,
            (PortB, 16, SPI1, SOUT) => AltFunction2,
            (PortB, 17, SPI1, SIN)  => AltFunction2,

            _ => panic!("Invalid pin for function")
        };
        
        pin.set_function(altfn);
    }

    fn init_transmission(&self) {
        // Clear TX FIFO and set DSPI to start
        self.reg.mcr
            .clear_clr_txf()
            .set_halt(Spi_mcr_halt::Start);
        // Clear End of Queue Flag
        //self.reg.sr
        //    .clear_eoqf();
    }

    fn wait_for_end_of_queue(&self) {
        wait_for!(self.reg.sr.eoqf() == Spi_sr_eoqf::Set); // Wait for the TX FIFO to reach 0 entries (all commands finished)
    }

    fn wait_for_tx_fifo_space(&self) {
        wait_for!(self.reg.sr.tfff() == Spi_sr_tfff::NotFull);
    }
}

use core::result::Result;

/// Vairous SPI Errors that can occur
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum SPIError {
    /// Shruggy emoticon - no clue, but something did go wrong.
    Unknown
}

/// The result of an SPI Transmission - either a number of values sent, or an error
pub type SPITxResult = Result<u32, SPIError>;

/// A Trait for types that we know how to transmit via SPI. This trait allows us to optimize sending given that the hardware mechanism can only transmit 4-16 bit values.
pub trait SPITransmit {
    /// Transmits a value and returns a result (Number of Frames on success, or an SPIError)
    fn transmit(&self, &SPI) -> SPITxResult;
}

/* The K20DX256 Has a TX FIFO of 4 16bit buffers, so the fastest you can transmit data is 2 32bit frames, 4 16 bit frames, 4 8 bit frames etc
 */
impl<'a> SPITransmit for &'a [u32] {
    /// Transfer a 32 bit Frame in MSB order
    /// Will transmit 32 bit words as individual frames inside one queue
    fn transmit(&self, spi: &SPI) -> SPITxResult {
        assert!(self.len() > 0);
        spi.init_transmission();

        let end = self.len() - 1;
        for i in 0..self.len() {
            // Clear End of Queue Flag
            spi.reg.sr
                .clear_eoqf();
            
            spi.wait_for_tx_fifo_space();
            {
                let mut ms16 = spi.reg.pushr.ignoring_state();

                // If this is the first word to transmit, start a queue
                if i == 0 { ms16.start_queue(); }

                // Most significant 16 bits of 32 bit frame
                ms16.start_frame()
                    .tx16((self[i] & 0xFF_FF_00_00).wrapping_shr(16) as u16);
            }

            spi.wait_for_tx_fifo_space();
            {
                let mut ls16 = spi.reg.pushr.ignoring_state();

                // Least significant 16 bits of 32 bit frame
                ls16.tx16((self[i] & 0x00_00_FF_FF) as u16)
                    .end_frame();
                
                // If this is the last word to transmit, end the queue
                //if i == end { ls16.end_queue(); }
                ls16.end_queue();
            }
            
            spi.wait_for_end_of_queue();
        }

        match spi.reg.tcr.spi_tcnt() {
            frames if frames == self.len() as u32 * 2 => Ok(frames / 2),
            _ => Err(SPIError::Unknown)
        }
    }
}

impl SPITransmit for u32 {
    fn transmit(&self, spi: &SPI) -> SPITxResult {
        spi.init_transmission();
        spi.reg.sr
            .clear_eoqf();

        spi.reg.pushr.ignoring_state()
            .start_queue()
            .start_frame()
            .tx16((self & 0xFF_FF_00_00).wrapping_shr(16) as u16);

        spi.reg.pushr.ignoring_state()
            .tx16((self & 0x00_00_FF_FF) as u16)
            .end_frame()
            .end_queue();

        spi.wait_for_end_of_queue();

        match spi.reg.tcr.spi_tcnt() {
            frames if frames == 2 => Ok(1),
            _ => Err(SPIError::Unknown)
        }
    }
}

impl<'a> Spi_pushr_Update<'a> {
    
    fn start_queue<'b>(&'b mut self) -> &'b mut Spi_pushr_Update<'a> {
        self.set_ctcnt(Spi_pushr_ctcnt::Clear) // First command, so clear count
    }

    fn start_frame<'b>(&'b mut self) -> &'b mut Spi_pushr_Update<'a> {
        self.set_cont(Spi_pushr_cont::KeepPCS) // Keep CS[x] asserted
    }

    fn tx16<'b>(&'b mut self, data: u16) -> &'b mut Spi_pushr_Update<'a> {
        self
            .set_eoq(Spi_pushr_eoq::NotEndOfQueue) // Default not end of queue
            .set_ctas(Spi_pushr_ctas::CTAR1) // use CTAR1 (16bit transfer)
            .set_txdata(data as u32)
    }

    fn tx8<'b>(&'b mut self, data: u8) -> &'b mut Spi_pushr_Update<'a> {
        self
            .set_eoq(Spi_pushr_eoq::NotEndOfQueue) // Default not end of queue
            .set_ctas(Spi_pushr_ctas::CTAR0) // use CTAR0 (8bit transfer)
            .set_txdata(data as u32)
    }

    fn end_frame<'b>(&'b mut self) -> &'b mut Spi_pushr_Update<'a> {
        self.set_cont(Spi_pushr_cont::ResetPCS) // Deassert PCS (end of frame)
    }

    fn end_queue<'b>(&'b mut self) -> &'b mut Spi_pushr_Update<'a> {
        self.set_eoq(Spi_pushr_eoq::EndOfQueue) // this is the last command

    }
}
