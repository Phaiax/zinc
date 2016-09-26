
use hal::k20::regs;

/// Available PIT channels.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq)]
pub enum PITChannel {
    PIT0,
    PIT1,
    PIT2,
    PIT3
}

/// Structure describing a PIT channel.
#[derive(Clone, Copy)]
pub struct Pit {
  #[allow(dead_code)]
  timer: &'static regs::Pit_timer,
  #[allow(dead_code)]
  channel: PITChannel
}

impl PITChannel {
    fn timer(self) -> &'static regs::Pit_timer {
        match self {
            PITChannel::PIT0 => &regs::PIT().timer[0],
            PITChannel::PIT1 => &regs::PIT().timer[1],
            PITChannel::PIT2 => &regs::PIT().timer[2],
            PITChannel::PIT3 => &regs::PIT().timer[3]
        }
    }
}

impl Pit {
    /// Setup a new PIT timer on a given channel.
    pub fn new(channel: PITChannel) -> Pit {
        regs::SIM().scgc6.set_pit(regs::Sim_scgc6_pit::ClockEnabled);
        let pit = Pit {
            timer: channel.timer(),
            channel: channel
        };

        pit
    }

    /// Convert a value in microseconds to a PIT load value. (Calculates the PIT tick duration from fBUS, then estimates a number of ticks equal to the desired duration)
    pub fn us_to_ldval(duration: u32) -> Option<u32> {
        super::clocks::bus_clock()
            .and_then(|fbus| {
                (fbus / 1000000).checked_mul(duration)
            })
    }

    /// Convert a value in milliseconds to a PIT load value. (Calculates the PIT tick duration from fBUS, then estimates a number of ticks equal to the desired duration)
    pub fn ms_to_ldval(duration: u32) -> Option<u32>{
        let fbus = super::clocks::bus_clock().expect("Bus Clock not set.");
        (fbus / 1000).checked_mul(duration)
    }

    /// Convert a value in seconds to a PIT load value. (Calculates the PIT tick duration from fBUS, then estimates a number of ticks equal to the desired duration)
    pub fn s_to_ldval(duration: u32) -> Option<u32> {
        let fbus = super::clocks::bus_clock().expect("Bus Clock not set.");
        fbus.checked_mul(duration)        
    }
}
