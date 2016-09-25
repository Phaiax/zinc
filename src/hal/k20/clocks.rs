//! K20 Clock Functionality

use hal::cortex_m4::systick;

/// System (CPU) Clock Frequency (0 = unknown, Hz otherwise)
static mut SystemClock: u32 = 0;
/// Bus (Peripheral) Clock Frequency (0 = unknown, Hz otherwise)
static mut BusClock: u32 = 0;
/// Flash Clock Frequency (0 = unknown, Hz otherwise)
static mut FlashClock: u32 = 0;

/// Get the current SystemClock frequency
#[inline(always)]
pub fn system_clock() -> Option<u32> {
    match unsafe { SystemClock } {
        0 => None,
        frequency => Some(frequency)
    }
}

/// Set the current SystemClock frequency (note, does not affect hardware registers, just the variables tracking effective frequency
#[inline(always)]
pub fn set_system_clock(frequency: u32) {
    unsafe { SystemClock = frequency }
}

/// Get the current BusClock frequency
#[inline(always)]
pub fn bus_clock() -> Option<u32> {
    match unsafe { BusClock } {
        0 => None,
        frequency => Some(frequency)
    }
}

/// Set the current BusClock frequency (note, does not affect hardware registers, just the variables tracking effective frequency
#[inline(always)]
pub fn set_bus_clock(frequency: u32) {
    unsafe { BusClock = frequency }
}

/// Get the current FlashClock frequency
#[inline(always)]
pub fn flash_clock() -> Option<u32> {
    match unsafe { FlashClock } {
        0 => None,
        frequency => Some(frequency)
    }    
}

/// Set the current FlashClock frequency (note, does not affect hardware registers, just the variables tracking effective frequency
#[inline(always)]
pub fn set_flash_clock(frequency: u32) {
    unsafe { FlashClock = frequency }
}

/// Initializes the Systick Interrupt to trigger every 10ms (derrived from SystemClock frequency)
#[inline(always)]
pub fn init_systick(ms: u32) {
    let freq = system_clock().expect("Clock not set");

    // f = Frequency (Hz)
    // ms = Period (ms)
    // f = count / s
    // F * S = count
    // duration(count) = 1/96_000_000
    // counts = period(s)

    // count = 96_000_000 = 1s
    // count =    960_000 = 1s / 100 = 10ms
    // To get MS = 10, count must be 960_000
    // 96_000_000 / 1000 = 96_000
    // 96_000 * 10 = 960_000
    // 960_000 - 1 = 959_999 counter reset value

    systick::setup((freq/1000) * ms - 1);
    systick::enable();
    systick::enable_irq();
}
