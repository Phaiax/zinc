// Zinc, the bare metal stack for rust.
// Copyright 2014 Ben Gamari <bgamari@gmail.com>
// Based upon work by Ben Harris <mail@bharr.is>
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

//! ISR Data for k20

use core::option::Option::{Some, None};
use core::slice;
use hal::isr::isr_cortex_m4;
use hal::cortex_common::scb;
use hal::cortex_common::nvic;
use hal::isr;

extern {
  fn isr_dma_0();
  fn isr_dma_1();
  fn isr_dma_2();
  fn isr_dma_3();
  fn isr_dma_4();
  fn isr_dma_5();
  fn isr_dma_6();
  fn isr_dma_7();
  fn isr_dma_8();
  fn isr_dma_9();
  fn isr_dma_10();
  fn isr_dma_11();
  fn isr_dma_12();
  fn isr_dma_13();
  fn isr_dma_14();
  fn isr_dma_15();
  fn isr_dma_err();
  fn isr_flash_complete();
  fn isr_flash_collision();
  fn isr_low_volt();
  fn isr_llwu();
  fn isr_wdt();
  fn isr_i2c_0();
  fn isr_i2c_1();
  fn isr_spi_0();
  fn isr_spi_1();
  fn isr_can_0_msg();
  fn isr_can_0_bus();
  fn isr_can_0_err();
  fn isr_can_0_tx();
  fn isr_can_0_rx();
  fn isr_can_0_wake();
  fn isr_i2s_0_tx();
  fn isr_i2s_0_rx();
  fn isr_uart_0_lon();
  fn isr_uart_0_stat();
  fn isr_uart_0_err();
  fn isr_uart_1_stat();
  fn isr_uart_1_err();
  fn isr_uart_2_stat();
  fn isr_uart_2_err();
  fn isr_adc_0();
  fn isr_adc_1();
  fn isr_cmp_0();
  fn isr_cmp_1();
  fn isr_cmp_2();
  fn isr_ftm_0();
  fn isr_ftm_1();
  fn isr_ftm_2();
  fn ist_cmt();
  fn isr_rtc_alarm();
  fn isr_rtc_tick();
  fn isr_pit_0();
  fn isr_pit_1();
  fn isr_pit_2();
  fn isr_pit_3();
  fn isr_pdb();
  fn isr_usb();
  fn isr_usb_dcd();
  fn isr_dac_0();
  fn isr_tsi();
  fn isr_mcg();
  fn isr_lptimer();
  fn isr_port_a();
  fn isr_port_b();
  fn isr_port_c();
  fn isr_port_d();
  fn isr_port_e();
  fn isr_soft();
}

#[link_section=".flash_configuration"]
#[allow(non_upper_case_globals)]
pub static FlashConfigField: [usize; 4] = [
    0xFFFFFFFF,
    0xFFFFFFFF,
    0xFFFFFFFF,
    0xFFFFFFFE,
];

#[allow(non_upper_case_globals)]
const ISR_COUNT: usize = 95;

#[link_section=".isr_vector_nvic"]
#[allow(non_upper_case_globals)]
#[no_mangle]
pub static NVIC_VECTORS: [isr::ISR; ISR_COUNT] = [
  Some(isr_dma_0),
  Some(isr_dma_1),
  Some(isr_dma_2),
  Some(isr_dma_3),
  Some(isr_dma_4),
  Some(isr_dma_5),
  Some(isr_dma_6),
  Some(isr_dma_7),
  Some(isr_dma_8),
  Some(isr_dma_9),
  Some(isr_dma_10),
  Some(isr_dma_11),
  Some(isr_dma_12),
  Some(isr_dma_13),
  Some(isr_dma_14),
  Some(isr_dma_15),
  Some(isr_dma_err),
  None,
  Some(isr_flash_complete),
  Some(isr_flash_collision),
  Some(isr_low_volt),
  Some(isr_llwu),
  Some(isr_wdt),
  None,
  Some(isr_i2c_0),
  Some(isr_i2c_1),
  Some(isr_spi_0),
  Some(isr_spi_1),
  None,
  Some(isr_can_0_msg),
  Some(isr_can_0_bus),
  Some(isr_can_0_err),
  Some(isr_can_0_tx),
  Some(isr_can_0_rx),
  Some(isr_can_0_wake),
  Some(isr_i2s_0_tx),
  Some(isr_i2s_0_rx),
  None,
  None,
  None,
  None,
  None,
  None,
  None,
  Some(isr_uart_0_lon),
  Some(isr_uart_0_stat),
  Some(isr_uart_0_err),
  Some(isr_uart_1_stat),
  Some(isr_uart_1_err),
  Some(isr_uart_2_stat),
  Some(isr_uart_2_err),
  None,
  None,
  None,
  None,
  None,
  None,
  Some(isr_adc_0),
  Some(isr_adc_1),
  Some(isr_cmp_0),
  Some(isr_cmp_1),
  Some(isr_cmp_2),
  Some(isr_ftm_0),
  Some(isr_ftm_1),
  Some(isr_ftm_2),
  Some(ist_cmt),
  Some(isr_rtc_alarm),
  Some(isr_rtc_tick),
  Some(isr_pit_0),
  Some(isr_pit_1),
  Some(isr_pit_2),
  Some(isr_pit_3),
  Some(isr_pdb),
  Some(isr_usb),
  Some(isr_usb_dcd),
  None,
  None,
  None,
  None,
  None,
  None,
  Some(isr_dac_0),
  None,
  Some(isr_tsi),
  Some(isr_mcg),
  Some(isr_lptimer),
  None,
  Some(isr_port_a),
  Some(isr_port_b),
  Some(isr_port_c),
  Some(isr_port_d),
  Some(isr_port_e),
  None,
  None,
  Some(isr_soft),
];

/// The Full set of k20 Interrupt Vectors
#[repr(C)]
#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Copy, Debug)]
pub enum InterruptVector {
    // ARM Common
    InitialStackPointer = 0,
    InitialProgramCounter = 1,
    NonMaskableInterrupt = 2,
    HardFault = 3,
    MemoryManagementFault = 4,
    BusFault = 5,
    UsageFault = 6,
    SupervisorCall = 11,
    DebugMonitor = 12,
    SysPendingReq = 14,
    SysTick = 15,

    DMA0TransferComplete = 16,
    DMA1TransferComplete = 17,
    DMA2TransferComplete = 18,
    DMA3TransferComplete = 19,
    DMA4TransferComplete = 20,
    DMA5TransferComplete = 21,
    DMA6TransferComplete = 22,
    DMA7TransferComplete = 23,
    DMA8TransferComplete = 24,
    DMA9TransferComplete = 25,
    DMA10TransferComplete = 26,
    DMA11TransferComplete = 27,
    DMA12TransferComplete = 28,
    DMA13TransferComplete = 29,
    DMA14TransferComplete = 30,
    DMA15TransferComplete = 31,
    DMAError = 32,

    FlashCommandComplete = 34,
    FlashReadCollision = 35,
    LowVoltageWarnDetect = 36,
    LowLeakageWakeup = 37,
    Watchdog = 38,

    I2C0 = 40,
    I2C1 = 41,
    SPI0 = 42,
    SPI1 = 43,

    Can0OredMessageBuf = 45,
    Can0BusOff = 46,
    Can0Error = 47,
    Can0TxWarning = 48,
    Can0RxWarning = 49,
    Can0Wakup = 50,
    I2S0Tx = 51,
    I2S0Rx = 52,

    Uart0LON = 60,
    Uart0Status = 61,
    Uart0Error = 62,
    Uart1Status = 63,
    Uart1Error = 64,
    Uart2Status = 65,
    Uart2Error = 66,

    ADC0 = 73,
    ADC1 = 74,
    CMP0 = 75,
    CMP1 = 76,
    CMP2 = 77,
    FTM0 = 78,
    FTM1 = 79,
    FTM2 = 80,
    CMT = 81,
    RTCAlarm = 82,
    RTCSeconds = 83,
    Pit0 = 84,
    Pit1 = 85,
    Pit2 = 86,
    Pit3 = 87,
    PDB = 88,
    USBOTG = 89,
    USBDCD = 90,

    DAC0 = 97,

    TSI = 99,
    MCG = 100,
    LowPowerTimer = 101,

    PortAPinDetect = 103,
    PortBPinDetect = 104,
    PortCPinDetect = 105,
    PortDPinDetect = 106,
    PortEPinDetect = 107,

    Software = 110,
}

impl InterruptVector {
    /// Convert an Interrupt Vector to a SCB_VTOR offset
    pub fn offset(&self) -> usize {
        (*self as usize) * 4
    }

    /// True if this interrupt vector is enabled in the NVIC
    pub fn is_enabled(&self) -> bool {
        nvic::is_enabled(self.offset())
    }

    /// Enables this interrupt in the NVIC
    pub fn enable(&self) {
        nvic::enable_irq(self.offset());
    }

    /// Disables this interrupt in the NVIC
    pub fn disable(&self) {
        nvic::disable_irq(self.offset());
    }

    /// True if there this interrupt is flagged as pending 
    pub fn is_pending(&self) -> bool {
        nvic::is_pending(self.offset())
    }
    
    /// Clear the pending flag for this interrupt
    pub fn clear_pending(&self) {
        nvic::clear_pending(self.offset());
    }

    /// True if this interrupt is active
    pub fn is_active(&self) -> bool {
        nvic::is_active(self.offset())
    }

    /// Set the priority of this interrupt
    pub fn set_priority(&self, prio: u8) {
        nvic::set_priority(self.offset(), prio);
    }

    /// Get the priority of this interrupt
    pub fn priority(&self) -> u8 {
        nvic::get_priority(self.offset())
    }

    /// Get the address of the ISR handler
    pub fn handler(&self) -> isr::ISR {
        current_vector_table()[*self as usize]
    }

}

fn current_vector_table() -> isr::VectorTable<'static> {
    unsafe { scb::scb_vtor(TOTAL_ISR_COUNT) }
}

const TOTAL_ISR_COUNT: usize = ISR_COUNT + isr_cortex_m4::ISRCount;

pub type K20VectorTable = [isr::ISR; TOTAL_ISR_COUNT];

#[link_section=".ram_isr_vector"]
#[allow(non_upper_case_globals)]
#[no_mangle]
pub static mut NVIC_VECTORS_RAM: K20VectorTable = [None; TOTAL_ISR_COUNT];

/// Relocate the ISR Table into Ram from flash.
pub fn install_ram_vectors() {
    let flash_addr = &isr_cortex_m4::ISRVectors[0] as *const isr::ISR;

    unsafe {
        let flash_nvic =
            slice::from_raw_parts::<isr::ISR>(flash_addr,
                                              TOTAL_ISR_COUNT);

        let mut ram_nvic: isr::VectorTable = &NVIC_VECTORS_RAM[..];

        ram_nvic.clone_from(&flash_nvic);

        scb::relocate_isrs(ram_nvic);
    }
}
