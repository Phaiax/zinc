#[path="../../util/ioreg.rs"]
#[macro_use] mod ioreg;

/// Registers
#[allow(dead_code)]
pub mod reg {
    use volatile_cell::VolatileCell;
    use core::ops::Drop;
    use core::convert::From;

    ioregs!(Osc = { //! System Oscillator
        0x0 => reg8 cr { //! System Oscillator Control Register
            7 => er { //! External Reference
                0x0 => Disabled,
                0x1 => Enabled
            },
            5 => er_stop { //! External Reference enabled in STOP mode
                0x0 => StopDisabled,
                0x1 => StopEnabled
            },
            0 => sc16p, //= Add 16pF to external xtal load
            1 => sc8p,  //= Add  8pF to external xtal load
            2 => sc4p,  //= Add  4pF to external xtal load
            3 => sc2p   //= Add  2pF to external xtal load
        }
    });

    ioregs!(Mcg = {
        0x0 => reg8 c1 { //! MGC_C1
            0 => irefsten { //! Internal Reference enabled in STOP mode?
                0x0 => StopDisabled,
                0x1 => StopEnabled //= enabled if IRCLKEN is set or if MCG is in FEI, FBI, or BLPI modes before entering Stop mode.
            },
            1 => irclken { //! Internal Reference Clock Enable
                0x0 => Inactive,
                0x1 => Active
            },
            2 => irefs { //! Internal Reference Select
                0x0 => External,
                0x1 => SlowInternal
            },
            3..5 => frdiv, //= FLL External Reference Divider. Interpretation depends on RANGE and OSCSEL
            6..7 => clks { //! Clock Source Select for MCGOUTCLK
                0x0 => PLLS,
                0x1 => Internal,
                0x2 => External
            }
        },
        0x1 => reg8 c2 { //! MCG_C2
            0 => ircs { //! Internal Reference Clock Select
                0x0 => SlowInternal,
                0x1 => FastInternal
            },
            1 => lp { //! Low Power Select
                0x0 => NotDisabled,
                0x1 => Disabled //= FLL or PLL disabled in bypass modes
            },
            2 => erefs0 { //! External Reference Select
                0x0 => External,
                0x1 => Oscillator
            },
            3 => hgo0{ //! High Gain Oscillator Select
                0x0 => LowPower,
                0x1 => HighGain
            },
            4..5 => range0 { //! Frequency Range Select
                0x0 => Low,
                0x1 => High,
                0x2 => VeryHigh
            },
            7 => locre0 { //! Loss of Clock Reset Enable
                0x0 => Interrupt,
                0x1 => Reset
            }
        },
        0x2 => reg8 c3 {
            0..7 => sctrim //= Slow Internal Reference Clock Trim Setting
        },
        0x3 => reg8 c4 {
            0 => scftrim, //= Bit nine of reg mgc_c3
            1..4 => fctrim, //= Fast Internal Reference Clock Trim Setting
            5..6 => drst_drs, //= DCO Range Select
            7 => dmx32 //= DCO Maximum Frequency
        },
        0x4 => reg8 c5 {
            0..4 => prdiv0, //= PLL External Reference Divider (1-25, 0b00000 = 1)
            5 => pllsten0 { //! PLL Stop Enable
                0x0 => Disabled,
                0x1 => Enabled
            },
            6 => pllclken0 { //! PLL Clock Enable
                0x0 => Inactive,
                0x1 => Active
            }
        },
        0x5 => reg8 c6 {
            0..4 => vdiv0, //= VCO 0 Divider 24-55 (0b00000 = 24)
            5 => cme0 { //! Clock Monitor Enable
                0x0 => Disabled,
                0x1 => Enabled
            },
            6 => plls { //! PLL Select
                0x0 => FLL,
                0x1 => PLL
            },
            7 => lolie0 { //! Loss of Lock Interrupt Enable
                0x0 => NoInterrupt,
                0x1 => Interrupt
            }
        },
        0x6 => reg8 status { //! Status Register
            0 => ircst: ro { //! Internal Reference Clock Status
                0 => Slow, //= 32kHz
                1 => Fast //= 2 MHz
            },
            1 => oscinit0: ro { //! OSC Initialization
                0 => Initializing,
                1 => Initialized
            },
            2..3 => clkst: ro { //! Clock Mode Status
                0 => FLL,
                1 => Internal,
                2 => External,
                3 => PLL
            },
            4 => irefst: ro { //! Internal Reference Status (FLL ref source)
                0 => External,
                1 => Internal
            },
            5 => pllst: ro { //! PLL Select Status
                0 => FLL,
                1 => PLL
            },
            6 => lock0: ro { //! Lock Status
                0 => Unlocked,
                1 => Locked
            },
            7 => lols0: ro { //! Loss of Lock Status
                0 => NotLost,
                1 => Lost
            }
        },
        0x8 => reg8 sc { //! Status and Control Register
            0 => locs0: ro { //! OSC0 Loss of Clock Status
                0 => NotLost,
                1 => Lost
            },
            1..3 => fcrdiv, //= Fast Clock Internal Reference Divider (1-128, vals 2^fcrdiv, 0b000 = /1)
            4 => fltprsrv { //! FLL Filter Preserve (on clock setup change)
                0 => Reset,
                1 => Preserve
            },
            5 => atmf: ro { //!  Automatic Trim Machine Fail
                0 => Normal,
                1 => Fail
            },
            6 => atms { //! Automatic Trim Machine Select
                0 => Int32k,
                1 => Int4M
            },
            7 => atme { //! Automatic Trim Machine Enable
                0 => TrimDisabled,
                1 => TrimEnabled
            }
        },
        0xA => reg8 atcvh { //! Auto Trim Compare Value High
            0..7 => val
        },
        0xB => reg8 atcvl { //! Auto Trim Compare Value Low
            0..7 => val
        },
        0xC => reg8 c7 {
            0 => oscsel { //! Oscillator Select
                0 => SystemOscillator,
                1 => RTCOscillator
            },
            1..7 => nil: ro
        },
        0xD => reg8 c8 {
            0 => locs1: ro { //! RTC Loss of Clock Status
                0 => NotLost,
                1 => Lost
            },
            1..4 => nil: ro,
            5 => cme1 { //! Clock Monitor Enable 1 (RTC External Ref)
                0 => Disabled,
                1 => Enabled
            },
            6 => lolre { //! Loss of Clock Reset Enable (PLL)
                0 => Interrupt, //= c6.lolie1 must also be Interrupt
                1 => Reset
            },
            7 => locre1 { //! Loss of Clock Reset Enable (RTC External Ref Clock)
                0 => Interrupt,
                1 => Reset
            },
        }
    });

    ioregs!(Sim = { //! System Integration Module
        0x0000 => reg32 sopt1 { //! System Options Register 1
            31 => usbregen { //! USB Voltage Regulator Enable
                0x0 => Disabled,
                0x1 => Enabled
            },
            30 => usbsstby { //! USB Voltage Regulator Enabled in STOP/VLPS/LLS/VLLS
                0x0 => Disabled,
                0x1 => Enabled
            },
            29 => usbvstby { //! USB Voltage Regulator Enabled in VLPR/VLPW
                0x0 => Disabled,
                0x1 => Enabled
            },
            18..19 => osc32ksel { //! 32kHz Oscillator clock select
                0b00 => SystemOscillator,
                0b10 => Rtc32k,
                0b11 => Lpo1k
            },
            12..15 => ramsize: ro { //! Ram Size
                0b0001 => Ram8k,
                0b0011 => Ram16k,
                0b0101 => Ram32k,
                0b0111 => Ram64k
            }
        },
        0x0004 => reg32 sopt1cfg { //! SOPT1 Confuration Register
            26 => usswe { //! USBSSTBY Write Enable
                0x0 => WriteProtected,
                0x1 => Writable
            },
            25 => uvswe { //! USBVSTBY Write Enable
                0x0 => WriteProtected,
                0x1 => Writable
            },
            24 => urwe { //! USBREGEN Write Enable
                0x0 => WriteProtected,
                0x1 => Writable
            },
        },
        0x1004 => reg32 sopt2 { //! System Options Register 2
            18 => usbsrc { //! USB Clock Source Select
                0 => ExternalBypassClock, //= USBCLKIN
                1 => PllFll //= MCGPLLCLK or MCGFLLCLK / USB Fractional Divider
            },
            16 => pllfllsel { //! PLL/FLL Clock Select
                0 => Fll,
                1 => Pll
            },
            12 => traceclksel { //! Debug Trace Clock Select
                0 => McgOutClk,
                1 => SystemClock
            },
            11 => ptd7pad { //! PTD7 Pad Drive Strength
                0 => SinglePad,
                1 => DoublePad
            },
            5..7 => clkoutsel { //! CLKOUT Select
                0b010 => FlashClock,
                0b011 => LpoClock,
                0b100 => McgIRClk,
                0b101 => Rtc32kClk,
                0b110 => OscERClk0
            },
            4 => rtcclkoutsel { //! RTC Clock Out Select
                0 => Rtc1Hz,
                1 => Rtc32kHz
            }
        },
        0x100C => reg32 sopt4 { //! System Options Register 4
            29 => ftm0trg1src { //! FlexTimer 0 Hardware Trigger 1 Source Select
                0 => Pdb,
                1 => Ftm32
            },
            28 => ftm0trg0src { //! FlexTimer 0 Hardware Trigger 0 Source Select
                0 => Hscmp0,
                1 => Ftm1
            },
            26 => ftm2clksel { //! FlexTimer 2 External Clock Pin Select
                0 => FtmClk0,
                1 => FtmClk1
            },
            25 => ftm1clksel { //! FlexTimer 1 External Clock Pin Select
                0 => FtmClk0,
                1 => FtmClk1
            },
            24 => ftm0clksel { //! FlexTimer 0 External Clock Pin Select
                0 => FtmClk0,
                1 => FtmClk1
            },
            20..21 => ftm2ch0src { //! FTM2 channel 0 input capture source select
                0b00 => Ftm2Ch0,
                0b01 => Cmp0,
                0b10 => Cmp1
            },
            18..19 => ftm1ch0src { //! FTM1 channel 0 input capture source select
                0b00 => Ftm1Ch0,
                0b01 => Cmp0,
                0b10 => Cmp1,
                0b11 => UsbStartOfFramePulse
            },
            8 => ftm2flt0 { //! FTM2 Fault 0 Select
                0 => Ftm2Flt0,
                1 => Cmp0
            },
            4 => ftm1flt0 { //! FTM1 Fault 0 Select
                0 => Ftm1Flt0,
                1 => Cmp0
            },
            2 => ftm0flt2 { //! FTM0 Fault 2 Select
                0 => Ftm0Flt2,
                1 => Cmp2
            },
            1 => ftm0flt1 { //! FTM0 Fault 1 Select
                0 => Ftm0Flt1,
                1 => Cmp1
            },
            0 => ftm0flt0 { //! FTM0 Fault 0 Select
                0 => Ftm0Flt0,
                1 => Cmp0
            },
        },
        0x1010 => reg32 sopt5 { //! System Options Register 5
            6..7 => uart1rxsrc { //! UART 1 receive data source select
                0b00 => Uart1Rx,
                0b01 => Cmp0,
                0b10 => Cmp1
            },
            4..5 => uart1txsrc { //! UART 1 transmit data source select
                0b00 => Uart1Tx,
                0b01 => Uart1TxModFtm1Ch0,
                0b10 => Uart1TxModFtm2Ch0,
            },
            2..3 => uart0rxsrc { //! UART 0 receive data source select
                0b00 => Uart0Rx,
                0b01 => Cmp0,
                0b10 => Cmp1
            },
            0..1 => uart0txsrc { //! UART 0 transmit data source select
                0b00 => Uart1Tx,
                0b01 => Uart1TxModFtm1Ch0,
                0b10 => Uart1TxModFtm2Ch0,
            }
        },
        0x1018 => reg32 sopt7 { //! System Options Register 7
            15 => adc1alttrgen { //! ADC1 alternate trigger enable
                0 => Pdb,
                1 => Alt
            },
            12 => adc1pretrgsel { //! ADC1 pre-trigger select
                0 => A,
                1 => B
            },
            8..11 => adc1trgsel { //! ADC1 Trigger Select
                0b0000 => PdbExtTriggerPin,
                0b0001 => HighSpdComparator0,
                0b0010 => HighSpdComparator1,
                0b0011 => HighSpdComparator2,
                0b0100 => PitTrigger0,
                0b0101 => PitTrigger1,
                0b0110 => PitTrigger2,
                0b0111 => PitTrigger3,
                0b1000 => Ftm0Trigger,
                0b1001 => Ftm1Trigger,
                0b1010 => Ftm2Trigger,
                0b1100 => RtcAlarm,
                0b1101 => RtcSeconds,
                0b1110 => LowPowerTimerTrigger
            },
            7 => adc0alttrgen { //! ADC0 alternate trigger enable
                0 => Pdb,
                1 => Alt
            },
            4  => adc0pretrgsel { //! ADC0 pre-trigger select
                0 => A,
                1 => B
            },
            0..3 => adc0trgsel { //! ADC0 Trigger Select
                0b0000 => PdbExtTriggerPin,
                0b0001 => HighSpdComparator0,
                0b0010 => HighSpdComparator1,
                0b0011 => HighSpdComparator2,
                0b0100 => PitTrigger0,
                0b0101 => PitTrigger1,
                0b0110 => PitTrigger2,
                0b0111 => PitTrigger3,
                0b1000 => Ftm0Trigger,
                0b1001 => Ftm1Trigger,
                0b1010 => Ftm2Trigger,
                0b1100 => RtcAlarm,
                0b1101 => RtcSeconds,
                0b1110 => LowPowerTimerTrigger
            }
        },
        0x1024 => reg32 sdid {
            12..15 => revid: ro, //= Device Revision Number
            4..6 => famid: ro { //! Kinetis Family ID
                0b000 => K10,
                0b001 => K20,
                0b010 => K30,
                0b011 => K40,
                0b110 => K50,
                0b111 => K51
            },
            0..3 => pinid: ro { //! Pincount ID
                0b0101 => Pin64,
                0b0110 => Pin80,
                0b0111 => Pin81,
                0b1000 => Pin100
            }
        },
        0x1028 => reg32 scgc1 { //! System Clock Gating Control Register 1
            0..31 => nothing: ro
        },
        0x102C => reg32 scgc2 { //! System Clock Gating Control Register 2
            12 => dac0 { //! Dac0 Clock Gate Control
                0 => ClockDisabled,
                1 => ClockEnabled
            }
        },
        0x1030 => reg32 scgc3 { //! System Clock Gating Control Register 3
            27 => adc1 { //! ADC1 Clock Gate Control
                0 => ClockDisabled,
                1 => ClockEnabled
            },
            24 => ftm2 { //! FlexTimer 2 Clock Gate Control
                0 => ClockDisabled,
                1 => ClockEnabled
            }
        },
        0x1034 => reg32 scgc4 { //! System Clock Gating Control Register 4
            20 => vref { //! VREG Clock Gate Control
                0 => ClockDisabled,
                1 => ClockEnabled
            },
            19 => cmp { //! Comparator Clock Gate Control
                0 => ClockDisabled,
                1 => ClockEnabled
            },
            18 => usbotg { //! USB Clock Gate Control
                0 => ClockDisabled,
                1 => ClockEnabled
            },
            12 => uart2 { //! UART2 Clock Gate Control
                0 => ClockDisabled,
                1 => ClockEnabled
            },
            11 => uart1 { //! UART1 Clock Gate Control
                0 => ClockDisabled,
                1 => ClockEnabled
            },
            10 => uart0 { //! UART0 Clock Gate Control
                0 => ClockDisabled,
                1 => ClockEnabled
            },
            7 => i2c1 { //! I^2C1 Clock Gate Control
                0 => ClockDisabled,
                1 => ClockEnabled
            },
            6 => i2c0 { //! I^2C0 Clock Gate Control
                0 => ClockDisabled,
                1 => ClockEnabled
            },
            2 => cmt { //! CMT Clock Gate Control
                0 => ClockDisabled,
                1 => ClockEnabled
            },
            1 => ewm { //! EWM Clock Gate Control
                0 => ClockDisabled,
                1 => ClockEnabled
            }
        },
        0x1038 => reg32 scgc5 { //! System Clock Gating Control Register 5
            13 => porte { //! Port E Clock Gate Control
                0 => ClockDisabled,
                1 => ClockEnabled
            },
            12 => portd { //! Port D Clock Gate Control
                0 => ClockDisabled,
                1 => ClockEnabled
            },
            11 => portc { //! Port C Clock Gate Control
                0 => ClockDisabled,
                1 => ClockEnabled
            },
            10 => portb { //! Port B Clock Gate Control
                0 => ClockDisabled,
                1 => ClockEnabled
            },
            9 => porta { //! Port A Clock Gate Control
                0 => ClockDisabled,
                1 => ClockEnabled
            },
            5 => tsi { //! TSI Clock Gate Control
                0 => ClockDisabled,
                1 => ClockEnabled
            },
            0 => lptimer { //! Low Power Timer Access Control
                0 => AccessDisabled,
                1 => AccessEnabled
            },
        },
        0x103C => reg32 scgc6 { //! System Clock Gating Control Register 6
            29 => rtc { //! RTC Access Control
                0 => Disabled, //= Access and interrupts disabled
                1 => Enabled //= Access and interrupts enabled
            },
            27 => adc0 { //! ADC0 Clock Gate Control
                0 => ClockDisabled,
                1 => ClockEnabled
            },
            25 => ftm1 { //! FlexTimer 1 Clock Gate Control
                0 => ClockDisabled,
                1 => ClockEnabled
            },
            24 => ftm0 { //! FlexTimer 0 Clock Gate Control
                0 => ClockDisabled,
                1 => ClockEnabled
            },
            23 => pit { //! PIT Clock Gate Control
                0 => ClockDisabled,
                1 => ClockEnabled
            },
            22 => pdb { //! PDB Clock Gate Control
                0 => ClockDisabled,
                1 => ClockEnabled
            },
            21 => usbdcd { //! USB DCD Clock Gate Control
                0 => ClockDisabled,
                1 => ClockEnabled
            },
            18 => crc { //! CRC Clock Gate Control
                0 => ClockDisabled,
                1 => ClockEnabled
            },
            15 => i2s { //! I^2S Clock Gate Control
                0 => ClockDisabled,
                1 => ClockEnabled
            },
            13 => spi1 { //! SPI1 Clock Gate Control
                0 => ClockDisabled,
                1 => ClockEnabled
            },
            12 => spi0 { //! SPI0 Clock Gate Control
                0 => ClockDisabled,
                1 => ClockEnabled
            },
            4 => flexcan0 { //! FlexCAN0 Clock Gate Control
                0 => ClockDisabled,
                1 => ClockEnabled
            },
            1 => dmamux { //! DMA Mux Clock Gate Control
                0 => ClockDisabled,
                1 => ClockEnabled
            },
            0 => ftfl { //! Flash Memory Clock Gate Control
                0 => ClockDisabled,
                1 => ClockEnabled
            }
        },
        0x1040 => reg32 scgc7 { //! System Clock Gating Control Register 6
            1 => dma { //! DMA Clock Gate Control
                0 => ClockDisabled,
                1 => ClockEnabled
            }
        },
        0x1044 => reg32 clkdiv1 { //! System Clock Divider Register 1
            28..31 => outdiv1, //= Clock 1 (Core/System) divider value (div by 1-16, 0b0000 = div-by-1)
            24..27 => outdiv2, //= Clock 2 (Peripheral) divider value (div by 1-16, 0b0000 = div-by-1)
            16..19 => outdiv4  //= Clock 1 (Flash) divider value (div by 1-16, 0b0000 = div-by-1)
        },
        0x1048 => reg32 clkdiv2 { //! System Clock Divider Register 1
            1..3 => usbdiv, //= USB Clock Divisor (Fractional clock divisor when SOPT2[USBSRC] is 1)
            0 => usbfrac //= USB Clock Divider Fraction (Divider output clock = Divider input clock × [ (USBFRAC+1) / (USBDIV+1) ])
        },
        0x104C => reg32 fcfg1 { //! Flash Configuration Register 1
            28..31 => nvmsize: ro { //! FlexNVM Size
                0b0000 => NVM0kb,
                0b0011 => NVM32kb //= 32KB of FlexNVM, 4KB Protection Region
            },
            24..27 => pfsize: ro { //! Program Flash Size
                0b0101 => Prog64Kb,
                0b0111 => Prog128Kb,
                0b1001 => Prog256kb
            },
            16..19 => eesize: ro { //! EEPROM Size
                0b0011 => EE2048b,
                0b0100 => EE1024b,
                0b0101 => EE512b,
                0b0110 => EE256b,
                0b0111 => EE128b,
                0b1000 => EE64b,
                0b1001 => EE32b,
                0b1111 => EE0b
            },
            8..11 => depart: ro, //= FlexNVM Partition
            1 => flashdoze { //! FlashDoze
                0 => Enabled, //= Flash Enabled in WAIT mode
                1 => Disabled //= Flash Disabled in WAIT mode
            },
            0 => flashdis { //! Flash Disable
                0 => FlashEnabled,
                1 => FlashDisabled
            }
        },
        0x1050 => reg32 fcfg2 { //! Flash Configuration Register 2
            24..30 => maxaddr0: ro, //= Max Address Block 0
            23 => pflsh: ro { //! Program Flash
                0 => Block1FlexNVM,
                1 => Block1ProgFlsh
            },
            16..22 => maxaddr1: ro //= Max Address Block 1
        },
        0x1054 => reg32 uidh { //! Unique ID Register High
            0..31 => uid: ro //= Device Unique ID
        },
        0x1058 => reg32 uidmh { //! Unique ID Register Mid-High
            0..31 => uid: ro //= Device Unique ID
        },
        0x105C => reg32 uidml { //! Unique ID Register Mid-Low
            0..31 => uid: ro //= Device Unique ID
        },
        0x1060 => reg32 uidl { //! Unique ID Register Low
            0..31 => uid: ro //= Device Unique ID
        }
    });

    ioregs!(Rtc = { //! Real Time Clock Module
        0x0000 => reg32 tsr { //! RTC Time Seconds Register
            0..31 => tsr
        },
        0x0004 => reg32 tpr { //! RTC Time Prescaler Register
            0..15 => tpr
        },
        0x0008 => reg32 tar { //! RTC Time Alarm Register
            0..31 => tar
        },
        0x000C => reg32 tcr { //! RTC Time Compensation Register
            31..24 => cic:ro, //= Compensation Interval Counter
            23..16 => tcv:ro, //= Time Compensation Value
            8..15 => cir, //= Compensation Interval Register
            0..7 => tcr //= Time Compensation Register
        },
        0x0010 => reg32 cr { //! RTC Control Register
            13 => sc2p, //= Add 2pf to RTC load
            12 => sc4p, //= Add 4pf to RTC load
            11 => sc8p, //= Add 8pf to RTC load
            10 => sc16p, //= Add 16pf to RTC load
            9 => clko { //! RTC Clock Ouput to other peripherals
                0 => Output,
                1 => NoOutput
            },
            8 => osce { //! Oscillator Enable
                0 => Disabled,
                1 => Enabled
            },
            3 => um { //! Update Mode
                0 => NoWritesWhileLocked,
                1 => WritesWhileLocked
            },
            2 => sup { //! Supervisor Access
                0 => SupervisorOnly,
                1 => NonSupervisors
            },
            1 => wpe { //! Wakeup Pin Enabled
                0 => Disabled,
                1 => Enabled
            },
            0 => swr { //! Software Reset
                0 => NoEffect,
                1 => ResetSWR
            }
        },
        0x0014 => reg32 sr { //! RTC Status Register
            4 => tce { //! Time Counter Enable
                0 => Disabled,
                1 => Enabled
            },
            2 => taf:ro { //! Time Alarm Flag
                0 => TimeAlarmNotOccurred,
                1 => TimeAlarmOccurred
            },
            1 => tof:ro { //! Time Overflow Flag
                0 => TimeOverflowNotOccurred,
                1 => TimeOverflowOccurred
            },
            0 => tif:ro { //! Time Invalid Flag
                0 => TimeValid,
                1 => TimeInvalid
            }
        },
        0x0018 => reg32 lr { //! RTC Lock Register
            6 => lrl { //! Lock Register Lock
                0 => Locked, //= Lock Register writes ignored
                1 => Unlocked
            },
            5 => srl { //! Status Register Lock
                0 => Locked, //= Status Register writes ignored
                1 => Unlocked
            },
            4 => crl { //! Control Register Lock
                0 => Locked, //= Control Register writes ignored
                1 => Unlocked
            },
            3 => tcl { //! Time Compensation Lock
                0 => Locked, //= Time Compensation Register writes ignored
                1 => Unlocked
            }
        },
        0x001C => reg32 ier { //! RTC Interrupt Enable Register
            4 => tsie { //! Time Seconds Interrupt Enable
                0 => Disabled,
                1 => Enabled
            },
            2 => taie { //! Time Alarm Interrupt Enable
                0 => Disabled,
                1 => Enabled
            },
            1 => toie { //! Time Overflow Interrupt Enable
                0 => Disabled,
                1 => Enabled
            },
            0 => tiie { //! Time Invalid Interrupt Enable
                0 => Disabled,
                1 => Enabled
            }
        },
        0x0800 => reg32 war { //! RTC Write Access Register
            7 => ierw { //! Interrupe Enable Register Write
                0 => WritesIgnored,
                1 => Normal
            },
            6 => lrw { //! Lock Register Write
                0 => WritesIgnored,
                1 => Normal
            },
            5 => srw { //! Status Register Write
                0 => WritesIgnored,
                1 => Normal
            },
            4 => crw { //! Control Register Write
                0 => WritesIgnored,
                1 => Normal
            },
            3 => tcrw { //! Time Compensation Register Write
                0 => WritesIgnored,
                1 => Normal
            },
            2 => tarw { //! Time Alarm Register Write
                0 => WritesIgnored,
                1 => Normal
            },
            1 => tprw { //! Time Prescaler Register Write
                0 => WritesIgnored,
                1 => Normal
            },
            0 => tsrw { //! Time Seconds Register Write
                0 => WritesIgnored,
                1 => Normal
            }
        },
        0x0804 => reg32 rar { //! RTC Read Access Register
            7 => ierr { //! Interrupe Enable Register Read
                0 => ReadsIgnored,
                1 => Normal
            },
            6 => lrr { //! Lock Register Read
                0 => ReadsIgnored,
                1 => Normal
            },
            5 => srr { //! Status Register Read
                0 => ReadsIgnored,
                1 => Normal
            },
            4 => crr { //! Control Register Read
                0 => ReadsIgnored,
                1 => Normal
            },
            3 => tcrr { //! Time Compensation Register Read
                0 => ReadsIgnored,
                1 => Normal
            },
            2 => tarr { //! Time Alarm Register Read
                0 => ReadsIgnored,
                1 => Normal
            },
            1 => tprr { //! Time Prescaler Register Read
                0 => ReadsIgnored,
                1 => Normal
            },
            0 => tsrr { //! Time Seconds Register Read
                0 => ReadsIgnored,
                1 => Normal
            }
        }
    });

    ioregs!(Pmc = { //! Power Management Controller
        0 => reg8 lvdsc1 { //! Low Voltage Detect Status And Control 1 register
            7 => lvdf:ro { //! Low Voltage Detect Flag
                0 => NotDetected,
                1 => Detected
            },
            6 => lvdack:wo, //= Low-Voltage Detect Acknowledge
            5 => lvdie { //! Low-Voltage Detect Interrupt Enable
                0 => InterruptDisabled,
                1 => InterruptEnabled
            },
            4 => lvdre { //! Low-Voltage Detect Reset Enable
                0 => NoHardwareReset,
                1 => HardwareReset
            },
            0..1 => lvdv { //! Low-Voltage Detect Voltage Select
                0b00 => LowTripPoint, //= V(LVD) == V(LVDL)
                0b01 => HighTripPoint //= V(LVD) == V(LVDH)
            }
        },
        1 => reg8 lvdsc2 { //! Low Voltage Detect Status And Control 2 register
            7 => lvwf:ro{ //! Low Voltage Warning Flag
                0 => NoWarning,
                1 => Warning
            },
            6 => lvwack:wo, //= Low-Voltage Warning Acknowledge
            5 => lvwie { //! Low-Voltage Warning Interrupt Enable
                0 => InterruptDisabled,
                1 => InterruptEnabled
            },
            0..1 => lvwv { //! Low-Voltage Warning Voltage Select
                0b00 => LowTripPoint, //= V(LVW) == V(LVW1)
                0b01 => Mid1TripPoint, //=  V(LVW) == V(LVW2)
                0b10 => Mid2TripPoint, //= V(LVW) == V(LVW3)
                0b11 => HighTripPoint //= V(LVD) == V(LVW4)
            }
        },
        2 => reg8 regsc { //! Regulator Status And Control register
            4 => bgen { //! Bandgap Enable In VLPx Operation
                0 => Disabled,
                1 => Enabled
            },
            3 => ackiso { //! Acknowledge Isolation
                0 => Normal,
                1 => Latched //= Writing Latched when this flag == Latched releases peripherals to their normal run mode state
            },
            2 => regons:ro { //! Regulator In Run Regulation Status
                0 => StopRegulation,
                1 => RunRegulation
            },
            0 => bgbe { //! Bandgap Buffer Enable
                0 => Diabled,
                1 => Enabled
            }
        }
    });

    ioregs!(Smc = { //! System Mode Controller
        0x0000 => reg8 pmprot { //! Power Mode Protection Register
            5 => avlp { //! Allow Very-Low-Power Modes
                0 => NotAllowed, //= VLPR, VLPW and VLPS are not allowed
                1 => Allowed
            },
            3 => alls { //! Allow Low-Leakage Stop Mode
                0 => NotAllowed,
                1 => Allowed
            },
            1 => avlls { //! Allow Very-Low-Leakage Stop Mode
                0 => NotAllowed,
                1 => Allowed
            }
        },
        0x0001 => reg8 pmctrl { //! Power Mode Control Register
            6..5 => runm {
                0b00 => Normal,
                0b10 => VeryLowPower,
            },
            3 => stopa: ro { //! Stop Aborted
                0 => Success,
                1 => Aborted
            },
            2..0 => stopm { //! Stop Mode Control
                0b000 => NormalStop,
                0b010 => VeryLowPowerStop,
                0b011 => LowLeakageStop,
                0b100 => VeryLowLeakageStop
            }
        },
        0x0002 => reg8 vllsctrl { //! VLLS Control Register
            2..0 => vllsm { //! VLLS Mode Control
                0b001 => VLLS1,
                0b010 => VLLS2,
                0b011 => VLLS3,
            }
        },
        0x003 => reg8 pmstat { //! Power Mode Status Register
            6..0 => pmstat: ro { //! Current Power Mode
                0b000_0001 => Run,
                0b000_0010 => Stop,
                0b000_0100 => VLPR,
                0b000_1000 => VLPW,
                0b001_0000 => VLPS,
                0b010_0000 => LLS,
                0b100_0000 => VLLS
            }
        }
    });

    ioregs!(Pit = { //! Periodic Interrupt Timer
        0x0000 => reg32 mcr { //! Module Control Register
            1 => mdis { //! Module Disable
                0 => PITClockEnabled,
                1 => PITClockDisabled
            },
            0 => frz { //! Freeze
                0 => RunInDebugMode,
                1 => StopInDebugMode
            }
        },
        0x0100 => group timer[4] { //! PIT Channel
            0x00 => reg32 ldval { //! Timer Load Value
                31..0 => tsv //= Timer Start Value
            },
            0x04 => reg32 cval { //! Current Timer Value
                31..0 => tvl //= Timer current value
            },
            0x08 => reg32 tctrl { //! Timer Control
                2 => chn { //! Chain Mode Timer[N-1] must expire to tick this timer
                    0 => NotChained,
                    1 => Chained
                },
                1 => tie { //! Timer Interrupt Enabled
                    0 => InterruptsDisabled,
                    1 => InterruptsEnabled
                },
                0 => ten { //! Timer Enabled (counting down)
                    0 => TimerDisabled,
                    1 => TimerEnabled
                }
            },
            0x0C => reg32 tflg { //! Timer Flag Register
                0 => tif: set_to_clear {
                    0 => NoTimeoutYet,
                    1 => Timeout
                }
            }
        }
    });

    ioregs!(Spi = { //! SPI Controller
        0x000 => reg32 mcr { //! Module Configuration Register
            31 => mstr { //! Master/Slave Mode Select
                0 => Slave,
                1 => Master
            },
            30 => cont_scke { //! Continuous SCK Enable
                0 => Disabled,
                1 => Enabled
            },
            29..28 => dconf { //! Configuration
                0b00 => SPI
            },
            27 => frz { //! Freeze
                0 => DoNotHalt, //= Don't halt serial transfers in debug mode
                1 => Halt
            },
            26 => mtfe { //! Modifed Timing Enabled
                0 => NormalTiming,
                1 => ModifiedTiming
            },
            24 => rooe { //! Receive FIFO Overflow Overwrite Enable
                0 => Ignore,
                1 => Overwrite
            },
            20..16 => pcsis, //= Peripheral Chip Select [4..0] 0 = Inactive Low
            15 => doze { //! Doze Enable
                0 => NoEffect,
                1 => DisableSPI
            },
            14 => mdis { //! Module Disable
                0 => EnableClocks,
                1 => CanDisable
            },
            13 => dis_txf { //! Disable Transmit FIFO
                0 => FIFO,
                1 => DoubleBuffered
            },
            12 => dis_rxf { //! Disable Receive FIFO
                0 => FIFO,
                1 => DoubleBuffered
            },
            11 => clr_txf: set_to_clear, //= Flush the TX FIFO
            10 => clr_rxf: set_to_clear, //= Flush the RX FIFO
            9..8 => smpl_pt { //! Sample Point in modified transfer format
                0b00 => NoClock,
                0b01 => OneClock,
                0b10 => TwoClocks
            },
            0 => halt { //! Starts and stops module transfers
                0 => Start,
                1 => Stop
            }
        },
        0x008 => reg32 tcr { //! DSPI Transfer Count Register
            31..16 => spi_tcnt //= SPI Transfer Counter
        },
        0x00C => group ctar[2] { //! DSPI Clock and Transfer Attributes Register (Master Mode)
            0 => reg32 ctar {
                31 => dbr { //! Double Baud Rate
                    0 => Normal,
                    1 => Doubled
                },
                30..27 => fmsz, //= Frame Size in bits (3=4..15=16) actual frame size is this value + 1 bit.
                26 => cpol { //! Clock Polarity
                    0 => InactiveLow,
                    1 => InactiveHigh
                },
                25 => cpha { //! Clock Phase
                    0 => CaptureLeading, //= Data is captured on the leading edge of SCK and changed on the following edge.
                    1 => ChangeLeading //= Data is changed on the leading edge of SCK and captured on the following edge.
                },
                24 => lsbfe { //! LSB First
                    0 => MSBFirst,
                    1 => LSBFirst
                },
                23..22 => pcssck { //! PCS to SCK Delay Prescaler
                    0b00 => Scale1,
                    0b01 => Scale3,
                    0b10 => Scale5,
                    0b11 => Scale7
                },
                21..20 => pasc { //! After SCK Delay Prescaler
                    0b00 => Scale1,
                    0b01 => Scale3,
                    0b10 => Scale5,
                    0b11 => Scale7
                },
                19..18 => pdt { //! Delay after Transfer prescaler
                    0b00 => Scale1,
                    0b01 => Scale3,
                    0b10 => Scale5,
                    0b11 => Scale7
                },
                17..16 => pbr { //! Baud Rate Prescaler
                    0b00 => Scale2,
                    0b01 => Scale3,
                    0b10 => Scale5,
                    0b11 => Scale7
                },
                15..12 => cssck { //! PCS to SCK Delay Scaler
                    0b0000 => Scale2,
                    0b0001 => Scale4,
                    0b0010 => Scale8,
                    0b0011 => Scale16,
                    0b0100 => Scale32,
                    0b0101 => Scale64,
                    0b0110 => Scale128,
                    0b0111 => Scale256,
                    0b1000 => Scale512,
                    0b1001 => Scale1024,
                    0b1010 => Scale2048,
                    0b1011 => Scale4096,
                    0b1100 => Scale8192,
                    0b1101 => Scale16384,
                    0b1110 => Scale32768,
                    0b1111 => Scale65536
                },
                11..8 => asc, //= After SCK Delay Scaler
                7..4 => dt, //= Delay After Transfer Scaler
                3..0 => br { //! Baud Rate Scaler
                    0b0000 => Scale2,
                    0b0001 => Scale4,
                    0b0010 => Scale6,
                    0b0011 => Scale8,
                    0b0100 => Scale16,
                    0b0101 => Scale32,
                    0b0110 => Scale64,
                    0b0111 => Scale128,
                    0b1000 => Scale256,
                    0b1001 => Scale512,
                    0b1010 => Scale1024,
                    0b1011 => Scale2048,
                    0b1100 => Scale4096,
                    0b1101 => Scale8192,
                    0b1110 => Scale16384,
                    0b1111 => Scale32768
                }
            }
        },
        0x00C => reg32 ctar0_slave { //! DSPI Clock and Transfer Attributes Register (Slave Mode)
            31..27 => fmsz, //= Frame Size (actual frame size is this +1, min is 3)
            26 => cpol { //! Clock Polarity
                0 => InactiveLow,
                1 => InactiveHigh
            },
            25 => cpha { //! Clock Phase
                0 => CaptureLeading, //= Data is captured on the leading edge of SCK and changed on the following edge.
                1 => ChangeLeading //= Data is changed on the leading edge of SCK and captured on the following edge.
            }
        },
        0x02C => reg32 sr { //! DSPI Status Register
            31 => tcf: set_to_clear { //! Transfer Complete Flag
                0 => TransferNotComplete,
                1 => TransferComplete
            },
            30 => txrxs: set_to_clear { //! TX and RX Status
                0 => Stopped,
                1 => Running
            },
            28 => eoqf: set_to_clear { //! End of Queue Flag
                0 => NotSet,
                1 => Set
            },
            27 => tfuf: set_to_clear { //! Transmit FIFO Underflow Flag
                0 => NoUnderflow,
                1 => Underflow
            },
            25 => tfff: set_to_clear { //! Transmit FIFO Fill Flag
                0 => Full, //= TX FIFO is full
                1 => NotFull //= TX FIFO isn't full - you can push data
            },
            19 => rfof: set_to_clear { //! Receive FIFO Overflow Flag
                0 => NoOverflow,
                1 => Overflow
            },
            17 => rfdf: set_to_clear { //! Receive FIFO Drain Flag
                0 => Empty, //= RX FIFO Empty
                1 => NotEmpty //= RX FIFO has data
            },
            15..12 => txctr, //= TX FIFO Counter
            11..8 => txnxtptr, //= Transmit Next Pointer
            7..4 => rxctr, //= RX FIFO Counter
            3..0 => popnxtptr //= Pop Next Pointer
        },
        0x030 => reg32 rser { //! DSPI DMA/Interrupt Request Select and Enable Register
            31 => tcf_re { //! Transmission Complete Request Enable
                0 => InterruptDisabled,
                1 => InterruptEnabled
            },
            28 => eoqf_re { //! DSPI Finished Request Enable
                0 => InterruptDisabled,
                1 => InterruptEnabled
            },
            27 => tfuf_re { //! Transmit FIFO Underflow Request Enable
                0 => InterruptDisabled,
                1 => InterruptEnabled
            },
            25 => tfff_re { //! Transmit FIFO Fill Request Enable
                0 => RequestDisabled,
                1 => RequestEnabled
            },
            24 => tfff_dirs { //! Transmit FIFO Fill DMA or Interrupt Request
                0 => Interrupt,
                1 => DMA
            },
            19 => rfof_re { //! Receive FIFO Overflow Request Enable
                0 => InterruptDisabled,
                1 => InterruptEnabled
            },
            17 => rfdf_re { //! Receive FIFO Drain Request Enable
                0 => RequestDisabled,
                1 => RequestEnabled
            },
            16 => rfdf_dirs { //! Receive FIFO Drain DMA or Interrupt Request
                0 => Interrupt,
                1 => DMA
            },
        },
        0x034 => reg32 pushr { //! DSPI Push TX FIFO Register in Master Mode
            31 => cont { //! Continuous Peripheral Chip Select Enable
                0 => ResetPCS,
                1 => KeepPCS
            },
            30..28 => ctas { //! Clock and Transfer Attributes Select
                0b000 => CTAR0,
                0b001 => CTAR1
            },
            27 => eoq { //! End Of Queue
                0 => NotEndOfQueue, //= non-last command in FIFO queue
                1 => EndOfQueue //= Last command in FIFO queue
            },
            26 => ctcnt { //! Clear Transfer Counter
                0 => Keep, //= Keep TCR[CTNT] before transmitting frame
                1 => Clear //= Clear TCR[CTNT] before transmitting frame
            },
            21..16 => pcs, //= PCS[5..0] Clock Select lines to assert
            15..0 => txdata //= Transmit Data
        },
        //0x034 => reg32 pushr_slave { //! DSPI Push TX FIFO Register in SLave Mode
        //    15..0 => rxdata //= Receive Data
        //},
        0x038 => reg32 popr { //! DSPI Pop RX FIFO Register
            31..0 => rxdata
        },
        0x03C => group txfr[4] { //! DSPI Transmit FIFO Registers for debugging
            0x00 => reg32 txfr {
                31..16 => txcmd, //= Transmit Command (Master mode)
                15..00 => txdata //= Transmit Data
            }
        },
        0x07C => group rxfr[4] { //! DSPI Receive FIFO Registers for debugging
            0x00 => reg32 rxfr {
                31..0 => rxdata: ro //= Receive Data
            }
        }
    });

    ioregs!(Usb = { //! USB OTG Controller
        0x0000 => reg8 perid { //! Peripheral ID Register
            5..0 => id: ro //= Peripheral ID (always reads as 0x4)
        },
        0x0004 => reg8 idcomp { //! Peripheral ID Completent Register
            5..0 => nid: ro //= Negated Peripheral ID (~0x4)
        },
        0x0008 => reg8 rev { //! Peripheral Revision
            7..0 => rev: ro //= Revision
        },
        0x000C => reg8 addinfo { //! Peripheral Additional Info register
            7..3 => irqnum: ro, //= Assigned Interrupt Request Number
            0 => iehost: ro { //! Host/Device mode indicator
                0 => Device,
                1 => Host
            }
        },
        0x0010 => reg8 otgistat { //! OTG Interrupt Status register
            7 => idchg { //! This bit is set when a change in the ID Signal from the USB connector is sensed.
                0 => NoChange,
                1 => Change
            },
            6 => onemsec { //! 1msec timer expired, must be cleared by software
                0 => NotExpired,
                1 => Expired
            },
            5 => line_state_change { //! USB Line state change detection
                0 => NoStateChange,
                1 => StateChanged
            },
            3 => sessvldchg { //! This bit is set when a change in VBUS is detected indicating a session valid or a session no longer valid.
                0 => NoChange,
                1 => Changed
            },
            2 => b_sess_chg { //! This bit is set when a change in VBUS is detected on a B device.
                0 => NoChange,
                1 => Changed
            },
            0 => avbuschg { //! This bit is set when a change in VBUS is detected on an A device.
                0 => NoChange,
                1 => Changed
            }
        },
        0x0014 => reg8 otgicr { //! OTG Interrupt Control Register
            7 => iden { //! ID Interrupt Enable
                0 => InterruptDisabled,
                1 => InterruptEnabled
            },
            6 => onemsecen { //! One Millisecond Interrupt Enable
                0 => InterruptDisabled,
                1 => InterruptEnabled
            },
            5 => linestateen { //! Line State Change Interrupt Enable
                0 => InterruptDisabled,
                1 => InterruptEnabled
            },
            3 => sessvlden { //! Session Valid Interrupt Enable
                0 => InterruptDisabled,
                1 => InterruptEnabled
            },
            2 => bsessen { //! B Session END Interrupt Enable
                0 => InterruptDisabled,
                1 => InterruptEnabled
            },
            0 => avbusen { //! A VBUS Valid Interrupt Enable
                0 => InterruptDisabled,
                1 => InterruptEnabled
            }
        },
        0x0018 => reg8 otgstat { //! OTG Status Register
            7 => id { //! Indicates the current state of the ID pin on the USB connector
                0 => TypeACable,
                1 => NCorBCable
            },
            5 => linestatestable { //! Indicates that the internal signals that control the LINE_STATE_CHG field of OTGISTAT are stable for at least 1 millisecond.
                0 => Unstable,
                1 => Stable
            },
            3 => sess_vld { //! Session Valid
                0 => BelowBValidThreshold,
                1 => AboveBValidThreshold
            },
            2 => bsessend { //! B Session End
                0 => BelowBEndThreshold,
                1 => AboveBEndThreshold
            },
            0 => avbusvld { //! A VBUS Valid
                0 => BelowAValidThreshold,
                1 => AboveAValidThreshold
            }
        },
        0x001C => reg8 otgctl { //! OTG Control Register
            7 => dphigh { //! D+ Data Line pullup resistor enable
                0 => NoPullup,
                1 => Pullup
            },
            5 => dplow { //! D+ Data Line pull-down resistor enable, This bit should always be enabled together with bit 4 (DMLOW)

                0 => NoPulldown,
                1 => Pulldown
            },
            4 => dmlow { //! D– Data Line pull-down resistor enable
                0 => NoPulldown,
                1 => Pulldown
            },
            2 => otgen { //! On-The-Go pullup/pulldown resistor enable
                0 => SetFromCTL,
                1 => SetFromOTGCTL
            }
        },
        0x0080 => reg8 istat { //! Interrupt Status register
            //! Contains fields for each of the interrupt sources within the USB Module. Each of these fields are qualified with their respective interrupt enable bits. All fields of this register are logically OR'd together along with the OTG Interrupt Status Register (OTGSTAT) to form a single interrupt source for the processor's interrupt controller. After an interrupt bit has been set it may only be cleared by writing a one to the respective interrupt bit. This register contains the value of 0x00 after a reset.
            7 => stall:set_to_clear, //= Stall Interrupt
            6 => attach:set_to_clear, //= Attach Interrupt
            5 => resume:set_to_clear, //= This bit is set depending upon the DP/DM signals, and can be used to signal remote wake-up signaling on the USB bus. When not in suspend mode this interrupt must be disabled.
            4 => sleep:set_to_clear, //= This bit is set when the USB Module detects a constant idle on the USB bus for 3 ms. The sleep timer is reset by activity on the USB bus.
            3 => tokdne:set_to_clear, //= This bit is set when the current token being processed has completed.
            2 => softok:set_to_clear, //= This bit is set when the USB Module receives a Start Of Frame (SOF) token.
            1 => error:set_to_clear, //= This bit is set when any of the error conditions within Error Interrupt Status (ERRSTAT) register occur. The processor must then read the ERRSTAT register to determine the source of the error.
            0 => usbrst:set_to_clear, //= This bit is set when the USB Module has decoded a valid USB reset.
        },
        0x0084 => reg8 inten { //! Interrupt Enable Register
            7 => stallen { //! STALL Interrupt Enable
                0 => InterruptDisabled,
                1 => InterruptEnabled
            },
            6 => attachen { //! ATTACH Interrupt Enable
                0 => InterruptDisabled,
                1 => InterruptEnabled
            },
            5 => resumeen { //! RESUME Interrupt Enable
                0 => InterruptDisabled,
                1 => InterruptEnabled
            },
            4 => sleepen { //! SLEEP Interrupt Enable
                0 => InterruptDisabled,
                1 => InterruptEnabled
            },
            3 => tokdneen { //! TOKDNE Interrupt Enable
                0 => InterruptDisabled,
                1 => InterruptEnabled
            },
            2 => softoken { //! SOFTOK Interrupt Enable
                0 => InterruptDisabled,
                1 => InterruptEnabled
            },
            1 => erroren { //! ERROR Interrupt Enable
                0 => InterruptDisabled,
                1 => InterruptEnabled
            },
            0 => usbrsten { //! USBRST Interrupt Enable
                0 => InterruptDisabled,
                1 => InterruptEnabled
            }
        },
        0x0088 => reg8 errstat { //! Error Interrupt Status register
            7 => btserr:set_to_clear, //= This bit is set when a bit stuff error is detected
            5 => dmaerr:set_to_clear, //= This bit is set if the USB Module has requested a DMA access to read a new BDT but has not been given the bus before it needs to receive or transmit data.
            4 => btoerr:set_to_clear, //= This bit is set when a bus turnaround timeout error occurs.
            3 => dfn8:set_to_clear, //= This bit is set if the data field received was not 8 bits in length.
            2 => crc16:set_to_clear, //= This bit is set when a data packet is rejected due to a CRC16 error.
            1 => crc5eof:set_to_clear, //= This error interrupt has two functions. When the USB Module is operating in peripheral mode (HOSTMODEEN=0), this interrupt detects CRC5 errors in the token packets generated by the host. If set the token packet was rejected due to a CRC5 error. Otherwise check K20 Sub-Family Reference Manual, Rev. 1.1, Dec 2012, page 986
            0 => piderr:set_to_clear //= This bit is set when the PID check field fails.
        },
        0x008C => reg8 erren { //! Error Interrupt Enable register
            7 => btserren { //! BTSERR Interrupt Enable
                0 => InterruptDisabled,
                1 => InterruptEnabled
            },
            5 => dmaerren { //! DMAERR Interrupt Enable
                0 => InterruptDisabled,
                1 => InterruptEnabled
            },
            4 => btoerren { //! BTOERR Interrupt Enable
                0 => InterruptDisabled,
                1 => InterruptEnabled
            },
            3 => dfn8en { //! DFN8 Interrupt Enable
                0 => InterruptDisabled,
                1 => InterruptEnabled
            },
            2 => crc16en { //! CRC16 Interrupt Enable
                0 => InterruptDisabled,
                1 => InterruptEnabled
            },
            1 => crc5eofen { //! CRC5/EOF Interrupt Enable
                0 => InterruptDisabled,
                1 => InterruptEnabled
            },
            0 => piderren { //! PIDERR Interrupt Enable
                0 => InterruptDisabled,
                1 => InterruptEnabled
            }
        },
        0x0090 => reg8 stat { //! USB Status Register
            7..4 => endp: ro, //= This four-bit field encodes the endpoint address that received or transmitted the previous token. This allows the processor core to determine the BDT entry that was updated by the last USB transaction.
            3 => tx: ro { //! Transmit Indicator (last transaction was:...)
                0 => Rx,
                1 => Tx
            },
            2 => odd:ro //= This bit is set if the last buffer descriptor updated was in the odd bank of the BDT.
        },
        0x0094 => reg8 ctl { //! USB Control register
            7 => jstate, //= Live USB differential receiver JSTATE signal
            6 => se0, //= Live USB Single Ended Zero signal
            5 => txsuspendtokenbusy, //= In Host mode, TOKEN_BUSY is set when the USB module is busy executing a USB token. Software must not write more token commands to the Token Register when TOKEN_BUSY is set.. Software should check this field before writing any tokens to the Token Register to ensure that token commands are not lost.
            //=In Target mode, TXD_SUSPEND is set when the SIE has disabled packet transmission and reception. Clearing this bit allows the SIE to continue token processing. This bit is set by the SIE when a SETUP Token is received allowing software to dequeue any pending packet transactions in the BDT before resuming token processing.
            4 => reset, //= Setting this bit enables the USB Module to generate USB reset signaling. This allows the USB Module to reset USB peripherals. This control signal is only valid in Host mode (HOSTMODEEN=1). Software must set RESET to 1 for the required amount of time and then clear it to 0 to end reset signaling. For more information on reset signaling see Section 7.1.4.3 of the USB specification version 1.0.
            3 => hostmodeen, //= When set to 1, this bit enables the USB Module to operate in Host mode. In host mode, the USB module performs USB transactions under the programmed control of the host processor.
            2 => resume, //= When set to 1 this bit enables the USB Module to execute resume signaling. This allows the USB Module to perform remote wake-up. Software must set RESUME to 1 for the required amount of time and then clear it to 0. If the HOSTMODEEN bit is set, the USB module appends a Low Speed End of Packet to the Resume signaling when the RESUME bit is cleared. For more information on RESUME signaling see Section 7.1.4.5 of the USB specification version 1.0.
            1 => oddrst, //= Setting this bit to 1 resets all the BDT ODD ping/pong fields to 0, which then specifies the EVEN BDT bank.
            0 => usbensofen { //! USB Enable
                //! Setting this bit causes the SIE to reset all of its ODD bits to the BDTs. Therefore, setting this bit resets much of the logic in the SIE. When host mode is enabled, clearing this bit causes the SIE to stop sending SOF tokens.
                0 => DisableUSBModule,
                1 => EnableUSBModule
            }
        },
        0x0098 => reg8 addr { //! USB Address Register
            7 => lsen { //! Low Speed Enable
                0 => RegularSpeed,
                1 => LowSpeed
            },
            6..0 => addr //= USB Address
        },
        0x009C => reg8 bdtpage1 { //! BDT Page Register 1
            7..1 => bdtba //= Bits 15..9 of the BDT base address (bits 8..0 are always 0 due to 512 byte alignment)
        },
        0x00A0 => reg8 frmnuml { //! Frame Number Register Low
            7..0 => frm //= This 8-bit field and the 3-bit field in the Frame Number Register High are used to compute the address where the current Buffer Descriptor Table (BDT) resides in system memory.
        },
        0x00A4 => reg8 frmnumh { //! Frame Number Register High
            2..0 => frm //= This 3-bit field and the 8-bit field in the Frame Number Register Low are used to compute the address where the current Buffer Descriptor Table (BDT) resides in system memory.
        },
        0x00A8 => reg8 token { //! Token register
            7..4 => tokenpid { //! Contains the token type executed by the USB module.
                0b0001 => OutToken,
                0b1001 => InToken,
                0b1101 => SetupToken
            },
            3..0 => tokenendpt //= Holds the Endpoint address for the token command. The four bit value written must be a valid endpoint.
        },
        0x00AC => reg8 softhld { //! SOF Threshold Register
            7..0 => cnt //= Represents the SOF count threshold in byte times.
        },
        0x00B0 => reg8 bdtpage2 { //! BDT Page Register 2
            7..1 => bdtba //= Bits 23..16 of the BDT base address (bits 8..0 are always 0 due to 512 byte alignment)
        },
        0x00B4 => reg8 bdtpage3 { //! BDT Page Register 3
            7..1 => bdtba //= Bits 31..24 of the BDT base address (bits 8..0 are always 0 due to 512 byte alignment)
        },
        0x00C0 => group endpt[16] {
            0x00 => reg8 endpt {
                7 => hostwohub, //= This is a Host mode only field and is present in the control register for endpoint 0 (ENDPT0) only. When set this bit allows the host to communicate to a directly connected low speed device. When cleared, the host produces the PRE_PID. It then switches to low-speed signaling when sends a token to a low speed device as required to communicate with a low speed device through a hub.
                6 => retrydis, //= This is a Host mode only bit and is present in the control register for endpoint 0 (ENDPT0) only. When set this bit causes the host to not retry NAK'ed (Negative Acknowledgement) transactions. When a transaction is NAKed, the BDT PID field is updated with the NAK PID, and the TOKEN_DNE interrupt is set. When this bit is cleared NAKed transactions is retried in hardware. This bit must be set when the host is attempting to poll an interrupt endpoint.
                4 => epctldis { //! This bit, when set, disables control (SETUP) transfers. When cleared, control transfers are enabled. This applies if and only if the EPRXEN and EPTXEN bits are also set.
                    0 => SetupEnabled,
                    1 => SetupDisabled
                },
                3 => eprxen { //! Endpoint RX Enable
                    0 => RxDisabled,
                    1 => RxEnabled
                },
                2 => eptxen { //! Endpoint TX Enable
                    0 => TxDisabled,
                    1 => TxEnabled
                },
                1 => epstall { //! When set this bit indicates that the endpoint is called. This bit has priority over all other control bits in the EndPoint Enable Register, but it is only valid if EPTXEN=1 or EPRXEN=1. Any access to this endpoint causes the USB Module to return a STALL handshake. After an endpoint is stalled it requires intervention from the Host Controller.
                    0 => NotStalled,
                    1 => Stalled
                },
                0 => ephshk { //! When set this bet enables an endpoint to perform handshaking during a transaction to this endpoint. This bit is generally 1 unless the endpoint is Isochronous.
                    0 => NoHandshaking, //= Isochronous transfer only?
                    1 => Handshake
                }
            },
            0x01 => reg8 pad1 {},
            0x02 => reg8 pad2 {},
            0x03 => reg8 pad3 {}
        },
        0x0100 => reg8 usbctrl { //! USB Control Register
            7 => susp { //! Places the USB tranceiver in suspend state
                0 => NotSuspended,
                1 => Suspended
            },
            6 => pde { //! Enable the weak pulldowns on D+/D-
                0 => PulldownsDisabled,
                1 => PulldownsEnabled
            }
        },
        0x0104 => reg8 observe { //! USB OTG Observe regiser
            7 => dppu:ro { //! D+ Pullup
                0 => PullupDisabled,
                1 => PullupEnabled
            },
            6 => dppd:ro { //! D+ Pulldown
                0 => PulldownDisabled,
                1 => PulldownEnabled
            },
            4 => dmpd:ro { //! D- Pulldown
                0 => PulldownDisabled,
                1 => PulldownEnabled
            }
        },
        0x0108 => reg8 control { //! USB OTG Control register
            4 => dppullupnonotg { //! Provides control of the DP Pullup in the USB OTG module, if USB is configured in non-OTG device mode.
                0 => PullupNotEnabled,
                1 => PullupEnabled
            }
        },
        0x010C => reg8 usbtrc0 { //! USB Transceiver Control Register 0
            7 => usbreset: set_to_clear, //= USB Reset
            6 => undocumented, //= http://kevincuzner.com/2014/12/12/teensy-3-1-bare-metal-writing-a-usb-driver/ suggests this is an undocumented interrupt bit.
            5 => usbresmen { //! Asynchronous Resume Interrupt Enable
                0 => WakeupDisabled, //= USB asynchronous wakeup from suspend mode disabled.
                1 => WakeupEnabled
            },
            1 => sync_det:ro { //! Synchronous USB Interrupt Detect
                0 => SyncNotDetected,
                1 => SyncDetected
            },
            0 => usb_resume_int: ro { //! USB Asynchronous Interrupt
                0 => NoInterrupt,
                1 => InterruptGenerated
            }
        },
        0x0114 => reg8 usbfrmadjust { //! Frame Adjust Register
            7..0 => adj //= Frame Adjustment. In Host mode, the frame adjustment is a twos complement number that adjusts the period of each USB frame in 12-MHz clock periods. A SOF is normally generated every 12,000 12-MHz clock cycles. The Frame Adjust Register can adjust this by -128 to +127 to compensate for inaccuracies in the USB 48-MHz clock. Changes to the ADJ bit take effect at the next start of the next frame.
        }
    });

//     ioregs!(BufferDescriptor = { //! An individual K20 Buffer Descriptor
//         0 => reg32 control { //! Control attributes
//             25..16 => bc, //= Byte Count
//             7 => own { //! Determines whether the processor or the USB-FS currently owns the buffer.
//                 0 => Processor, //= this buffer descriptor can be modified by code/the CPU
//                 1 => Controller //= this buffer descriptor can only be modifed by the USB controller
//             },
//             6 => data01 { //! Defines whether a DATA0 field (DATA0/1=0) or a DATA1 (DATA0/1=1) field was transmitted or received. It is unchanged by the USB-FS.
//                 0 => Data0,
//                 1 => Data1
//             },
//             5 => keep, //= Tok[3] _or_ 'Keep'. Typically, this bit is 1 with ISO endpoints feeding a FIFO. The microprocessor is not informed that a token has been processed, the data is simply transferred to or from the FIFO. When KEEP is set, normally the NINC bit is also set to prevent address increment.
//             4 => ninc, //= Tok[2] _or_ 'No Increment'. Disables the DMA engine address increment. This forces the DMA engine to read or write from the same address. This is useful for endpoints when data needs to be read from or written to a single location such as a FIFO. Typically this bit is set with the KEEP bit for ISO endpoints that are interfacing to a FIFO.
//             3 => dts, //= Tok[1] _or_ 'Data Toggle Synchronization'. Setting this bit enables the USB-FS to perform Data Toggle Synchronization.
//             2 => bdt_stall, //= Tok[0] _or_ trigger STALL handshake if this BDT is used. Setting this bit causes the USB-FS to issue a STALL handshake if a token is received by the SIE that would use the BDT in this location.
//         },
//         4 => reg32 addr { //! Buffer Address
//             31..0 => addr //= The 32bit address of the buffer in memory.
//         }
//     });
//
//     // Add an override field for pid_tok over 5..2 of a BufferDescriptor. (Hand implement what ioregs would)
//     impl BufferDescriptor_control {
//         /// Return the token for this buffer descriptor (overloaded with keep/ninc/dts/bdt_stall
//         pub fn pid_tok(&self) -> u32 {
//             BufferDescriptor_control_Get::new(self)
//                 .pid_tok()
//         }
//
//         /// Set the value of the pid_tok field
//         pub fn set_pid_tok<'a>(&'a self, new_value: u32) -> BufferDescriptor_control_Update<'a> {
//             let mut setter: BufferDescriptor_control_Update = BufferDescriptor_control_Update::new(self);
//             setter.set_pid_tok(new_value);
//             setter
//         }
//     }
//
//     impl BufferDescriptor_control_Get {
//         /// Return the token for this buffer descriptor (overloaded with keep/ninc/dts/bdt_stall
//         pub fn pid_tok(&self) -> u32 {
//             ((self.value >> 2) & 0b1111)
//         }
//     }
//
//     impl<'a> BufferDescriptor_control_Update<'a> {
//         /// Set the value of the pid_tok field
//         #[inline(always)]
//         pub fn set_pid_tok<'b>(&'b mut self, new_value: u32) -> &'b mut BufferDescriptor_control_Update<'a> {
//
//               self.value = (self.value & !(0b1111 << 2)) | ((new_value as u32) & 0b1111) << 2;
//               self.mask |= 0b1111 << 2;
//               self
//         }
//
//     }

    impl<'a> Usb_istat_Update<'a> {
        /// Clear all ISR flags
        #[inline(always)]
        pub fn clear_all<'b>(&'b mut self) -> &'b mut Usb_istat_Update<'a> {
            self.value = 0xFF;
            self.mask = 0xFF;
            self
        }
    }

    impl<'a> Usb_errstat_Update<'a> {
        /// Clear all error flags
        #[inline(always)]
        pub fn clear_all<'b>(&'b mut self) -> &'b mut Usb_errstat_Update<'a> {
            self.value = 0xFF;
            self.mask = 0xFF;
            self
        }
    }

    impl<'a> Usb_otgistat_Update<'a> {
        /// Clear all OTG Interrupt status flags
        #[inline(always)]
        pub fn clear_all<'b>(&'b mut self) -> &'b mut Usb_otgistat_Update<'a> {
            self.value = 0xFF;
            self.mask = 0xFF;
            self
        }
    }

    impl<'a> Usb_erren_Update<'a> {
        /// Enables all the USB Error Interrupts. (0xBF as bit[6] is reserved)
        #[inline(always)]
        pub fn enable_all<'b>(&'b mut self) -> &'b mut Usb_erren_Update<'a> {
            self.value = 0xBF;
            self.mask = 0xBF;
            self
        }
    }

    impl From<Spi_ctar_ctar_dbr> for u32 {
        fn from(dbr: Spi_ctar_ctar_dbr) -> u32 {
            match dbr {
                Spi_ctar_ctar_dbr::Normal => 1,
                Spi_ctar_ctar_dbr::Doubled => 2,
            }
        }
    }

    impl From<Spi_ctar_ctar_pbr> for u32 {
        fn from(pbr: Spi_ctar_ctar_pbr) -> u32 {
            match pbr {
                Spi_ctar_ctar_pbr::Scale2 => 2,
                Spi_ctar_ctar_pbr::Scale3 => 3,
                Spi_ctar_ctar_pbr::Scale5 => 5,
                Spi_ctar_ctar_pbr::Scale7 => 7,
            }
        }
    }

    impl From<Spi_ctar_ctar_br> for u32 {
        fn from(br: Spi_ctar_ctar_br) -> u32 {
            match br {
                Spi_ctar_ctar_br::Scale2 => 2,
                Spi_ctar_ctar_br::Scale4 => 4,
                Spi_ctar_ctar_br::Scale6 => 6,
                Spi_ctar_ctar_br::Scale8 => 8,
                Spi_ctar_ctar_br::Scale16 => 16,
                Spi_ctar_ctar_br::Scale32 => 32,
                Spi_ctar_ctar_br::Scale64 => 64,
                Spi_ctar_ctar_br::Scale128 => 128,
                Spi_ctar_ctar_br::Scale256 => 256,
                Spi_ctar_ctar_br::Scale512 => 512,
                Spi_ctar_ctar_br::Scale1024 => 1024,
                Spi_ctar_ctar_br::Scale2048 => 2048,
                Spi_ctar_ctar_br::Scale4096 => 4096,
                Spi_ctar_ctar_br::Scale8192 => 8192,
                Spi_ctar_ctar_br::Scale16384 => 16384,
                Spi_ctar_ctar_br::Scale32768 => 32768
            }
        }
    }

    extern {
        #[link_name="k20_iomem_OSC"] pub static OSC: Osc;
        #[link_name="k20_iomem_MCG"] pub static MCG: Mcg;
        #[link_name="k20_iomem_SIM"] pub static SIM: Sim;
        #[link_name="k20_iomem_RTC"] pub static RTC: Rtc;
        #[link_name="k20_iomem_PMC"] pub static PMC: Pmc;
        #[link_name="k20_iomem_SMC"] pub static SMC: Smc;
        #[link_name="k20_iomem_PIT"] pub static PIT: Pit;
        #[link_name="k20_iomem_SPI0"] pub static SPI0: Spi;
        #[link_name="k20_iomem_SPI1"] pub static SPI1: Spi;
        #[link_name="k20_iomem_USB"] pub static USB: Usb;
    }
}
