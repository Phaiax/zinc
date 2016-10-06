
use super::super::builder::McuSpecificConfig;
use builder::InitPart;

pub fn teensy_configurator(s : &mut McuSpecificConfig) {


    s.get_base_config_mut().add_to_init(InitPart::UseStatement, "\nuse zinc::hal::k20::watchdog;".into());
    s.get_base_config_mut().add_to_init(InitPart::UseStatement, "\nuse zinc::hal::k20::regs::*;".into());
    s.get_base_config_mut().add_to_init(InitPart::UseStatement, "\nuse zinc::hal::k20::clocks;".into());
    s.get_base_config_mut().add_to_init(InitPart::UseStatement, "\nuse zinc::hal::k20::rtc;".into());
    s.get_base_config_mut().add_to_init(InitPart::Pre, r#"
        watchdog::init(watchdog::State::Disabled);

        SIM().scgc6.ignoring_state()
            .set_rtc(Sim_scgc6_rtc::Enabled)           // Allow access to RTC module
            .set_ftfl(Sim_scgc6_ftfl::ClockEnabled);   // Enable clock for flash memory

        // if the RTC oscillator isn't enabled, get it started early
        rtc::init();

        // release I/O pins hold, if we woke up from VLLS mode
        // if (PMC_REGSC & PMC_REGSC_ACKISO) PMC_REGSC |= PMC_REGSC_ACKISO;
        if PMC().regsc.ackiso() == Pmc_regsc_ackiso::Latched {
            PMC().regsc.set_ackiso(Pmc_regsc_ackiso::Latched);
        }

        // since this is a write once register, make it visible to all F_CPU's
        // so we can into other sleep modes in the future at any speed
        // SMC_PMPROT = SMC_PMPROT_AVLP | SMC_PMPROT_ALLS | SMC_PMPROT_AVLLS;
        SMC().pmprot.ignoring_state()
            .set_avlp(Smc_pmprot_avlp::Allowed)
            .set_alls(Smc_pmprot_alls::Allowed)
            .set_avlls(Smc_pmprot_avlls::Allowed);

        // enable osc, 8-32 MHz range, low power mode
        // MCG_C2 = MCG_C2_RANGE0(2) | MCG_C2_EREFS;
        OSC().cr
            .ignoring_state()
            .set_sc8p(true)
            .set_sc2p(true);

        // enable osc, 8-32 MHz range, low power mode
        // MCG_C2 = MCG_C2_RANGE0(2) | MCG_C2_EREFS;
        MCG().c2
            .ignoring_state()
            .set_range0(Mcg_c2_range0::VeryHigh)
            .set_erefs0(Mcg_c2_erefs0::Oscillator);
        // switch to crystal as clock source, FLL input = 16 MHz / 512
        // MCG_C1 =  MCG_C1_CLKS(2) | MCG_C1_FRDIV(4);
        MCG().c1
            .ignoring_state()
            .set_clks(Mcg_c1_clks::External)
            .set_frdiv(4);

        // wait for crystal oscillator to begin
        wait_for!(MCG().status.oscinit0() == Mcg_status_oscinit0::Initialized);
        wait_for!(MCG().status.irefst() == Mcg_status_irefst::External);
        // wait for MCGOUT to use oscillator
        wait_for!(MCG().status.clkst() == Mcg_status_clkst::External);


        // config PLL input for 16 MHz Crystal / 6 = 2.667 Hz
        MCG().c5.ignoring_state().set_prdiv0(5);

        // config PLL for 72 MHz output
        MCG().c6.ignoring_state()
            .set_plls(Mcg_c6_plls::PLL)
            .set_vdiv0(3);

        // wait for PLL to start using xtal as its input
        wait_for!(MCG().status.pllst() == Mcg_status_pllst::PLL);
        // wait for PLL to lock
        wait_for!(MCG().status.lock0() == Mcg_status_lock0::Locked);
        // now we're in PBE mode

        // config divisors: 72 MHz core, 36 MHz bus, 24 MHz flash, USB = 72 * 2 / 3
        // SIM_CLKDIV1 = SIM_CLKDIV1_OUTDIV1(0) | SIM_CLKDIV1_OUTDIV2(1) | SIM_CLKDIV1_OUTDIV4(2);
        SIM().clkdiv1.ignoring_state()
            .set_outdiv1(0)
            .set_outdiv2(1)
            .set_outdiv4(2);
        // SIM_CLKDIV2 = SIM_CLKDIV2_USBDIV(2) | SIM_CLKDIV2_USBFRAC;
        SIM().clkdiv2.ignoring_state()
            .set_usbdiv(1)
            .set_usbfrac(true);

        // switch to PLL as clock source, FLL input = 16 MHz / 512
        // MCG_C1 = MCG_C1_CLKS(0) | MCG_C1_FRDIV(4);
        MCG().c1.ignoring_state()
            .set_clks(Mcg_c1_clks::PLLS)
            .set_frdiv(4);
        // wait for PLL clock to be used
        wait_for!(MCG().status.clkst() == Mcg_status_clkst::PLL);
        // now we're in PEE mode
        
        // USB uses PLL clock, trace is CPU clock, CLKOUT=OSCERCLK0
        // SIM_SOPT2 = SIM_SOPT2_USBSRC | SIM_SOPT2_PLLFLLSEL | SIM_SOPT2_TRACECLKSEL
        //      | SIM_SOPT2_CLKOUTSEL(6);
        SIM().sopt2.ignoring_state()
            .set_usbsrc(Sim_sopt2_usbsrc::PllFll)
            .set_pllfllsel(Sim_sopt2_pllfllsel::Pll)
            .set_traceclksel(Sim_sopt2_traceclksel::SystemClock)
            .set_clkoutsel(Sim_sopt2_clkoutsel::OscERClk0);

        // Record the clock frequencies we've just set up.
        clocks::set_system_clock(72_000_000);
        clocks::set_bus_clock(36_000_000);
        clocks::set_flash_clock(24_000_000);
        // USB is at 48_000_000


"#.into());

    s.get_base_config_mut().add_to_init(InitPart::UseStatement, "\nuse zinc::hal::cortex_m4::systick;".into());

    s.get_base_config_mut().add_to_init(InitPart::Main, r#"
        // The CLKSOURCE bit in SysTick Control and Status register is 
        // always set to select the core clock.
        // Because the timing reference (FCLK) is a variable frequency, the TENMS bit in the
        // SysTick Calibration Value Register is always zero.
        // Set tick freq to 1 ms
        systick::setup(72_000_000/1000); // 
        systick::enable();
"#.into());
}