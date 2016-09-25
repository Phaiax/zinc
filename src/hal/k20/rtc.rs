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
K20 Real Time Clock
*/

use super::regs::reg::*;

/// Initialize the RTC Oscillator, capacitance loading
#[inline(always)]
pub fn init() {
    if RTC.cr.osce() == Rtc_cr_osce::Disabled { // If RTC OSC is disabled
        RTC.sr
            .set_tce(Rtc_sr_tce::Disabled); // Disable Time Counter
        // Add 20pf load to RTC clock, and enable the RTC oscillator
        RTC.cr.ignoring_state()
            .set_sc8p(true)
            .set_sc4p(true)
            .set_osce(Rtc_cr_osce::Enabled);
    }
}

/// Disable the RTC counter, set prescaler to 0, set the time, return the RTC counter to its original state.
pub fn set(time: u32) {
    let tce = RTC.sr.tce();
    if tce == Rtc_sr_tce::Enabled {
        RTC.sr.set_tce(Rtc_sr_tce::Disabled);
    }

    // Set prescaler to 0
    RTC.tpr.set_tpr(0);
    // Set the clock time
    RTC.tsr.set_tsr(time);

    if tce == Rtc_sr_tce::Enabled {
        RTC.sr.set_tce(Rtc_sr_tce::Enabled);
    }
}

/// Control the RTC Seconds Interrupt (true enabled the interrupt).
pub fn tick_isr(enabled: bool) {
    RTC.ier.set_tsie(match enabled {
        true => Rtc_ier_tsie::Enabled,
        false => Rtc_ier_tsie::Disabled
    });
}

/// Enable the RTC time counter
pub fn enable() {
    RTC.sr.set_tce(Rtc_sr_tce::Enabled);
}

/// Return the value of the K20 RTC's Time Second Register, or None if the time is invalid.
pub fn time() -> Option<u32> {
    match RTC.sr.tif() {
        Rtc_sr_tif::TimeValid => {
            Some(RTC.tsr.tsr())
        },
        Rtc_sr_tif::TimeInvalid => {
            None
        }
    }
}
