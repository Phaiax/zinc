// Zinc, the bare metal stack for rust.
// Copyright 2016 Daniel Seemer 'phaiax' <phaiax-zinc@invisibletower.de>
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

use super::builder::McuSpecificConfig;
use util::Template;


pub fn create_linker_script(m : &McuSpecificConfig) -> String {
    let mut template = Template::new(LINKERSCRIPTTEMPLATE);

    template.replace("{{ pflashsize }}", &m.get_mcu_parameters().get_program_flash_size().str());
    template.replace("{{ sramsize }}", &m.get_mcu_parameters().get_sram_size().str());
    template.replace("{{ eflashsize }}", &m.get_e_flash_size().str());

    if m.get_memory_config().usbdescriptortable_size.0 > 0 {
        template.replace("/* {{ usbdescriptortable }} */", r#"
            .usbdescriptortable (NOLOAD) : ALIGN(512) {
                *(.usbdescriptortable*)
            } > ram
        "#);
    }

    // TODO implement eeprom stuff
    template.replace("{{ eepromflexram }}", "");

    template.render()
}


const LINKERSCRIPTTEMPLATE: &'static str = r#"

__aeabi_unwind_cpp_pr0 = abort;
__aeabi_unwind_cpp_pr1 = abort;
__aeabi_unwind_cpp_pr2 = abort;

_data_load = LOADADDR(.data);

INCLUDE iomem.ld

ENTRY(main)

MEMORY
{
    VECT (R)      : ORIGIN = 0x00000000, LENGTH = 0x3FC  /* Vector area */
    FIRC (R)      : ORIGIN = 0x000003FC, LENGTH = 4      /* Custom IRC user trim */
    FCFG (R)      : ORIGIN = 0x00000400, LENGTH = 16     /* Flash config */
    FLASH (RX)    : ORIGIN = 0x00000410, LENGTH = {{ pflashsize }} - 0x410 /* Program Flash */
    EFLASH (RX)   : ORIGIN = 0x10000000, LENGTH = {{ eflashsize }} /* FlexNVM Flash */
    {{ eepromflexram }}
    SRAM (WAIL)   : ORIGIN = 0x20000000 - {{ sramsize }} / 2, LENGTH = {{ sramsize }}
}

REGION_ALIAS("vectors", VECT);
REGION_ALIAS("flash_config", FCFG)
REGION_ALIAS("rom", FLASH);
REGION_ALIAS("ram", SRAM);

SECTIONS
{
    .vector ALIGN(4) : /* ISR Table in Flash */
    {
        KEEP(*(.isr_vector))
        KEEP(*(.isr_vector_nvic))
    } > vectors = 0xff

    .flashcfg :
    {
        . =  ALIGN(4);
        KEEP(*(.flash_configuration))
    } > flash_config

    .text : ALIGN(4)
    {
        *(.text*)               /* Code */
        *(.rodata .rodata.*)    /* constants/strings/... */
        *(.glue_7)              /* glue arm to thumb code */
        *(.glue_7t)             /* glue thumb to arm code */
    } > rom = 0xff

    /* Unwind tables */
    .ARM.extab :
    {
        *(.ARM.extab* .gnu.linkonce.armextab.*)
    } > rom = 0xff

    /* Unwind table indices */
    .ARM : {
        __exidx_start = .;
        *(.ARM.exidx*)
        __exidx_end = .;
    } > rom = 0xff

    /* Debug symbol info support */
    .debug_gdb_scripts :
    {
        KEEP(*(.debug_gdb_scripts))
    } > rom = 0xff

    /* ISR Table space in Ram, NOLOAD because it's initialized by code - just make space for it. */
    .vector_ram (NOLOAD) : ALIGN(1024)
    {
        KEEP(*(.ram_isr_vector))
    } > ram

    /* {{ usbdescriptortable }} */

    /* Initialized data sections goes into RAM, startup code
       initializes Ram from Flash (copies _data->_edata to
       _data_load+offset) */
    .data : ALIGN(4)
    {
        _data = .;

        *(SORT_BY_ALIGNMENT(.data*))
        . = ALIGN(4);

        _edata = .;
    } > ram AT>rom = 0xff

    /* BSS sections initialized to 0 by startup code */
    .bss : ALIGN(4)
    {
        _bss = .;

        *(.bss*)
        *(COMMON)
        . = ALIGN(4);

        _ebss = .;

        . += 4;

        __STACK_LIMIT = .;

        . += 4;

        _eglobals = .;
    } > ram

    /DISCARD/ :
    {
        *(.v4_bx)  /* ARMv4 interworking fixup for missing BX */
        *(.vfp11_veneer)  /* VFP11 bugfixes s.a. http://sourceware.org/ml/binutils/2006-12/msg00196.html */
        *(.iplt .igot.plt)  /* STT_GNU_IFUNC symbols */
        *(.rel.*)  /* dynamic relocations */
        /* *(.ARM.exidx*) /* index entries for section unwinding */
        /* *(.ARM.extab*) /* exception unwinding information */
        /* *(.debug_gdb_scripts) */
    }
}

__STACK_BASE  = ORIGIN(ram) + LENGTH(ram);

"#;

