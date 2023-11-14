use stm32g0b0::Peripherals;


pub(crate) const SYSTEM_CLOCK_SPEED: u32 = 64_000_000;


fn set_up_clocks(peripherals: &Peripherals) {
    // initial setup:
    // ┌────────┐  ┌────────┐  ┌────────┐
    // │ HSI16  ├──┤ HSISYS ├──┤ SYSCLK │
    // │ 16 MHz │  │ DIV=1  │  │ 16 MHz │
    // └────────┘  └────────┘  └────────┘
    //
    // target setup:
    // ┌────────┐  ┌─────┐ VCO
    // │ HSI16  ├──┤ PLL ├────── 128 MHz
    // │ 16 MHz │  │ N=8 │
    // └────────┘  │ M=1 │ R    ┌────────┐
    //             │ R=2 ├──────┤ SYSCLK │
    //             └─────┘      │ 64 MHz │
    //                          └────────┘

    // note: flash access must be slowed down to 2 wait states for 48MHz < f <= 64MHz
    peripherals.FLASH.acr.modify(|_, w| w
        .latency().two()
    );

    // 1. feed PLL from HSI16
    peripherals.RCC.pllcfgr.modify(|_, w| w
        .pllsrc().hsi16()
        .pllm().div1()
        .plln().mul8()
        .pllpen().clear_bit()
        .pllqen().clear_bit()
        .pllren().set_bit()
        .pllr().div2()
    );

    // 2. enable PLL
    peripherals.RCC.cr.modify(|_, w| w
        .pllon().set_bit()
    );
    while peripherals.RCC.cr.read().pllrdy().bit_is_clear() {
        // wait
    }

    // 3. plug PLL into SYSCLK
    peripherals.RCC.cfgr.modify(|_, w| w
        .sw().pllrclk()
    );
    while !peripherals.RCC.cfgr.read().sws().is_pllrclk() {
        // wait
    }

    // that should be enough
}


pub fn set_up(peripherals: &Peripherals) {
    set_up_clocks(peripherals);
}
