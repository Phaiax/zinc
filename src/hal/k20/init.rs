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
K20 Initialization utility functions
*/

use hal::mem_init;
use hal::k20::regs::reg::*;
use hal::isr::isr_k20;

use hal::k20::clocks;

/// K20 initialization function for 16MHz crystal and 96MHz system clock
#[inline(always)]
pub fn startup(f_cpu: u32) {
    assert!(f_cpu == 96_000_000);
    mem_init::init_stack();
    mem_init::init_data();
    isr_k20::install_ram_vectors();
    
    // SIM_SCGC3 = SIM_SCGC3_ADC1 | SIM_SCGC3_FTM2;
    // Turn on clock for FlexTimer2 and ADC1
    //r SIM.scgc3
    //r     .ignoring_state()
    //r     .set_ftm2(Sim_scgc3_ftm2::ClockEnabled)
    //r     .set_adc1(Sim_scgc3_adc1::ClockEnabled);

    // SIM_SCGC5 = 0x00043F82;		// clocks active to all GPIO
    // Turn on clock for all 5 ports (all GPIO)
    //r SIM.scgc5
    //r     .ignoring_state()
    //r     .set_porta(Sim_scgc5_porta::ClockEnabled)
    //r     .set_portb(Sim_scgc5_portb::ClockEnabled)
    //r     .set_portc(Sim_scgc5_portc::ClockEnabled)
    //r     .set_portd(Sim_scgc5_portd::ClockEnabled)
    //r     .set_porte(Sim_scgc5_porte::ClockEnabled);
    
    // SIM_SCGC6 = SIM_SCGC6_RTC | SIM_SCGC6_FTM0 | SIM_SCGC6_FTM1 | SIM_SCGC6_ADC0 | SIM_SCGC6_FTFL;
    SIM.scgc6.ignoring_state()
        .set_rtc(Sim_scgc6_rtc::Enabled)
    //r     .set_ftm0(Sim_scgc6_ftm0::ClockEnabled)
    //r     .set_ftm1(Sim_scgc6_ftm1::ClockEnabled)        
    //r     .set_adc0(Sim_scgc6_adc0::ClockEnabled)
        .set_ftfl(Sim_scgc6_ftfl::ClockEnabled);

    // if the RTC oscillator isn't enabled, get it started early
    super::rtc::init();
    
    // release I/O pins hold, if we woke up from VLLS mode
    // if (PMC_REGSC & PMC_REGSC_ACKISO) PMC_REGSC |= PMC_REGSC_ACKISO;
    if PMC.regsc.ackiso() == Pmc_regsc_ackiso::Latched {
        PMC.regsc.set_ackiso(Pmc_regsc_ackiso::Latched);
    }

    // since this is a write once register, make it visible to all F_CPU's
    // so we can into other sleep modes in the future at any speed
    // SMC_PMPROT = SMC_PMPROT_AVLP | SMC_PMPROT_ALLS | SMC_PMPROT_AVLLS;
    SMC.pmprot.ignoring_state()
        .set_avlp(Smc_pmprot_avlp::Allowed)
        .set_alls(Smc_pmprot_alls::Allowed)
        .set_avlls(Smc_pmprot_avlls::Allowed);
    
    /*
	// default all interrupts to medium priority level
	for (i=0; i < NVIC_NUM_INTERRUPTS + 16; i++) _VectorsRam[i] = _VectorsFlash[i];
	for (i=0; i < NVIC_NUM_INTERRUPTS; i++) NVIC_SET_PRIORITY(i, 128);
	SCB_VTOR = (uint32_t)_VectorsRam;	// use vector table in RAM
     */

    // hardware always starts in FEI mode
    //  C1[CLKS] bits are written to 00
    //  C1[IREFS] bit is written to 1
    //  C6[PLLS] bit is written to 0
    // MCG_SC[FCDIV] defaults to divide by two for internal ref clock
    // I tried changing MSG_SC to divide by 1, it didn't work for me

    // enable capacitors for crystal
    // OSC0_CR = OSC_SC8P | OSC_SC2P;
    OSC.cr
        .ignoring_state()
        .set_sc8p(true)
        .set_sc2p(true);

    // enable osc, 8-32 MHz range, low power mode
    // MCG_C2 = MCG_C2_RANGE0(2) | MCG_C2_EREFS;
    MCG.c2
        .ignoring_state()
        .set_range0(Mcg_c2_range0::VeryHigh)
        .set_erefs0(Mcg_c2_erefs0::Oscillator);
    // switch to crystal as clock source, FLL input = 16 MHz / 512
    // MCG_C1 =  MCG_C1_CLKS(2) | MCG_C1_FRDIV(4);
    MCG.c1
        .ignoring_state()
        .set_clks(Mcg_c1_clks::External)
        .set_frdiv(4);
    // wait for crystal oscillator to begin
    // while ((MCG_S & MCG_S_OSCINIT0) == 0) ;
    wait_for!(MCG.status.oscinit0() == Mcg_status_oscinit0::Initialized);

    // while ((MCG_S & MCG_S_IREFST) != 0) ;
    wait_for!(MCG.status.irefst() == Mcg_status_irefst::External);
    // wait for MCGOUT to use oscillator
    // while ((MCG_S & MCG_S_CLKST_MASK) != MCG_S_CLKST(2)) ;
    wait_for!(MCG.status.clkst() == Mcg_status_clkst::External);

    // now in FBE mode
    //  C1[CLKS] bits are written to 10
    //  C1[IREFS] bit is written to 0
    //  C1[FRDIV] must be written to divide xtal to 31.25-39 kHz
    //  C6[PLLS] bit is written to 0
    //  C2[LP] is written to 0
    /*
    #if F_CPU == 72000000
	MCG_C5 = MCG_C5_PRDIV0(5);		 // config PLL input for 16 MHz Crystal / 6 = 2.667 Hz
    #else
	MCG_C5 = MCG_C5_PRDIV0(3);		 // config PLL input for 16 MHz Crystal / 4 = 4 MHz
    #endif
     */
    // config PLL input for 16 MHz Crystal / 4 = 4 MHz
    MCG.c5.ignoring_state()
        .set_prdiv0(3);
    /*
    #if F_CPU == 168000000
	MCG_C6 = MCG_C6_PLLS | MCG_C6_VDIV0(18); // config PLL for 168 MHz output
    #elif F_CPU == 144000000
	MCG_C6 = MCG_C6_PLLS | MCG_C6_VDIV0(12); // config PLL for 144 MHz output
    #elif F_CPU == 120000000
	MCG_C6 = MCG_C6_PLLS | MCG_C6_VDIV0(6); // config PLL for 120 MHz output
    #elif F_CPU == 72000000
	MCG_C6 = MCG_C6_PLLS | MCG_C6_VDIV0(3); // config PLL for 72 MHz output
    #elif F_CPU == 96000000 || F_CPU == 48000000 || F_CPU == 24000000
	MCG_C6 = MCG_C6_PLLS | MCG_C6_VDIV0(0); // config PLL for 96 MHz output
    #elif F_CPU > 16000000
    #error "This clock speed isn't supported..."
    #endif
     */
    MCG.c6.ignoring_state()
        .set_plls(Mcg_c6_plls::PLL)
        .set_vdiv0(0);
    
    // wait for PLL to start using xtal as its input
    // while (!(MCG_S & MCG_S_PLLST)) ;
    wait_for!(MCG.status.pllst() == Mcg_status_pllst::PLL);
    // wait for PLL to lock
    // while (!(MCG_S & MCG_S_LOCK0)) ;
    wait_for!(MCG.status.lock0() == Mcg_status_lock0::Locked);
    // now we're in PBE mode

    // config divisors: 96 MHz core, 48 MHz bus, 24 MHz flash, USB = 96 / 2
    // SIM_CLKDIV1 = SIM_CLKDIV1_OUTDIV1(0) | SIM_CLKDIV1_OUTDIV2(1) | SIM_CLKDIV1_OUTDIV4(3);
    SIM.clkdiv1.ignoring_state()
        .set_outdiv1(0)
        .set_outdiv2(1)
        .set_outdiv4(3);
    // SIM_CLKDIV2 = SIM_CLKDIV2_USBDIV(1);
    SIM.clkdiv2.ignoring_state()
        .set_usbdiv(1);
    
    // switch to PLL as clock source, FLL input = 16 MHz / 512
    // MCG_C1 = MCG_C1_CLKS(0) | MCG_C1_FRDIV(4);
    MCG.c1.ignoring_state()
        .set_clks(Mcg_c1_clks::PLLS)
        .set_frdiv(4);
    // wait for PLL clock to be used
    //while ((MCG_S & MCG_S_CLKST_MASK) != MCG_S_CLKST(3)) ;
    wait_for!(MCG.status.clkst() == Mcg_status_clkst::PLL);
    // now we're in PEE mode
    // USB uses PLL clock, trace is CPU clock, CLKOUT=OSCERCLK0
    // SIM_SOPT2 = SIM_SOPT2_USBSRC | SIM_SOPT2_PLLFLLSEL | SIM_SOPT2_TRACECLKSEL
    //		| SIM_SOPT2_CLKOUTSEL(6);
    SIM.sopt2.ignoring_state()
        .set_usbsrc(Sim_sopt2_usbsrc::PllFll)
        .set_pllfllsel(Sim_sopt2_pllfllsel::Pll)
        .set_traceclksel(Sim_sopt2_traceclksel::SystemClock)
        .set_clkoutsel(Sim_sopt2_clkoutsel::OscERClk0);

    // Record the clock frequencies we've just set up.
    clocks::set_system_clock(f_cpu);
    clocks::set_bus_clock(f_cpu / 2);
    clocks::set_flash_clock(f_cpu / 4);
    
    //init_pins();
    //__enable_irq();
    
    //_init_Teensyduino_internal_();
    /*
    #if defined(KINETISK)
	// RTC initialization
	if (RTC_SR & RTC_SR_TIF) {
		// this code will normally run on a power-up reset
		// when VBAT has detected a power-up.  Normally our
		// compiled-in time will be stale.  Write a special
		// flag into the VBAT register file indicating the
		// RTC is set with known-stale time and should be
		// updated when fresh time is known.
		#if ARDUINO >= 10600
		rtc_set((uint32_t)&__rtc_localtime);
		#else
		rtc_set(TIME_T);
		#endif
		*(uint32_t *)0x4003E01C = 0x5A94C3A5;
	}
	if ((RCM_SRS0 & RCM_SRS0_PIN) && (*(uint32_t *)0x4003E01C == 0x5A94C3A5)) {
		// this code should run immediately after an upload
		// where the Teensy Loader causes the Mini54 to reset.
		// Our compiled-in time will be very fresh, so set
		// the RTC with this, and clear the VBAT resister file
		// data so we don't mess with the time after it's been
		// set well.
		#if ARDUINO >= 10600
		rtc_set((uint32_t)&__rtc_localtime);
		#else
		rtc_set(TIME_T);
		#endif
		*(uint32_t *)0x4003E01C = 0;
	}
#endif


	__libc_init_array();

	startup_late_hook();
	main();
*/

}
