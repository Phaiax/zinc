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
USB Tranceiver functions and utilities.

Sooo... how is all this going to work.

Firstly memory. We need a buffer descriptor table sufficient for 2-16 endpoints allocated with 512byte alignment. The calling code is going to have to do that and pass it in to us, for us to manage.

Then we need some transceiver buffers. We can statically allocate some buffers for known message sizes ourselves (8 byte setup message buffer?), but whenever the calling code wants to receive from USB, they're going to have to give us some memory for us to write into.

 */

use hal::k20::regs::reg;
use hal::isr::isr_k20;

use util::support::nop;

use core::slice;
use core::mem;
use core::ptr;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// A K20 USB-FS Buffer Descriptor
pub struct BufferDescriptor {
    control: usize,
    addr: *const u8
}

impl Default for BufferDescriptor {
    fn default() -> BufferDescriptor {
        BufferDescriptor {
            control: 0,
            addr: ptr::null()
        }
    }
}

#[repr(C)]
/// Buffer Descriptor Ownership (Processor or USB-FS Controller)
pub enum BufferDescriptor_own {
    /// The CPU (and thus rust-code) owns and can modify this descriptor
    Processor = 0,
    /// The USB-FS controller owns this buffer descriptor - do not modify it.
    Controller = 1
}

impl BufferDescriptor {

    /// Create a new buffer descriptor backed by byte buffer (you supply a pointer to the buffer that must remain valid until the USB Controler doesn't need it any more
    pub fn new(buf: *const u8) -> BufferDescriptor {
        let bd = BufferDescriptor {
            control: 0,
            addr: buf
        };
        bd
    }
    
    /// Buffer Byte Count (up to 1024 bytes)
    pub fn byte_count(&self) -> usize {
        // Bits 25..16
        (self.control & 0x03_FF_00_00) >> 16
    }

    /// Buffer Descriptor ownership
    pub fn ownership(&self) -> BufferDescriptor_own {
        // Bit 7
        match self.control & 0x00_00_00_40 {
            0 => BufferDescriptor_own::Processor,
            _ => BufferDescriptor_own::Controller
        }
    }

    /// USB Packet token for this buffer (tok_pid)
    pub fn token(&self) -> u32 {
        // Bits 5..2
        ((self.control & 0b11_1100) >> 2) as u32
    }

    /// If true, the USB-FS keeps ownership of this BDT
    pub fn keep(&self) -> bool {
        // Bit 5
        (self.control & 0b10_0000) != 0
    }

    /// True if the USB-FS shouldn't increment the address it writes to. Usually only used to point the BDT into a fifo.
    pub fn ninc(&self) -> bool {
        // Bit 4
        (self.control & 0b1_0000) != 0
    }

    /// If true, 'Data Tottle Synchronization' is used. No idea - look it up in the manual
    pub fn dts(&self) -> bool {
        // Bit 3
        (self.control & 0b1000) != 0
    }

    /// Trigger a USB Stall if this BDT would be used.
    pub fn bdt_stall(&self) -> bool {
        // Bit 2
        (self.control & 0b100) != 0
    }

    /// Obtain a slice from the address in the BDT plus the number of bytes written into it.
    pub fn buffer(&self) -> &[u8] {
        assert!(!self.addr.is_null());
        unsafe { slice::from_raw_parts(self.addr, self.byte_count()) }
    }
}


/// The K20 USB Endpoint Ping/pong buffer descriptor
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct UsbEndpoint {
    even: BufferDescriptor,
    odd: BufferDescriptor
}

/// A full buffer descriptor entry:
/// rx { even, odd },
/// tx { even, odd}
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct BDTEntry {
    rx: UsbEndpoint,
    tx: UsbEndpoint
}

/// Type for the k20 USB-FS BDT.
pub struct BDT {
    data: *const BDTEntry,
    len: usize
}

impl BDT {
    /// Create a BDT by moving in an array of BDEs
    pub unsafe fn from_raw(memory: &'static mut [u8], endpoints: usize) -> Option<BDT> {
        if memory.as_ptr().is_null() ||
            endpoints < 2 || endpoints > 16 ||
            memory.len() != (endpoints * mem::size_of::<BDTEntry>()) {
            return None;
        }
        
        Some(BDT {
            data: memory.as_ptr() as *const BDTEntry,
            len: endpoints
        })
    }

    /// Return the base address of the Buffer Descriptor Table
    pub fn addr(&self) -> *const BDTEntry {
        self.data
    }

    /// Return the number of Endpoints
    pub fn endpoints(&self) -> usize {
        self.len
    }
}
