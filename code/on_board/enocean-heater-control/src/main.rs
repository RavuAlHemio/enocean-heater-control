#![no_main]
#![no_std]


mod setup;
mod spi;
mod uart;


use core::panic::PanicInfo;

use cortex_m_rt::entry;
use stm32g0b0::Peripherals;
use stm32g0b0::spi1::cr1::BR_A;

use crate::setup::set_up;
use crate::spi::{Spi, Spi1};
use crate::uart::{Uart, Uart1, Uart2};


#[panic_handler]
fn handle_panic(_info: &PanicInfo) -> ! {
    loop {
    }
}


#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    set_up(&peripherals);

    // we will be using GPIO A and B; pass the clock to them
    peripherals.RCC.iopenr.modify(|_, w| w
        .gpioaen().set_bit()
        .gpioben().set_bit()
    );

    // configure GPIO pins that are not used for µC peripherals
    peripherals.GPIOA.moder.modify(|_, w| w
        .moder4().output() // OLED ~{CS}
        .moder8().output() // OLED EN
        .moder11().output() // keypad ~{CS}
        .moder12().output() // flash ~{CS}
        .moder15().output() // flash ~{write-prot}
    );
    peripherals.GPIOB.moder.modify(|_, w| w
        .moder2().output() // OLED D/~C
    );

    // pull all CS pins up (inactive)
    peripherals.GPIOA.bsrr.write(|w| w
        .bs4().set_bit() // up: OLED ~{CS}
        .bs11().set_bit() // up: keypad ~{CS}
        .bs12().set_bit() // up: flash ~{CS}
    );

    // turn on UART1 for debugging
    Uart1.set_up(&peripherals, 115_200);
    Uart1.send(&peripherals, b"OIDA\r\n");

    // turn on UART2 for EnOcean
    Uart2.set_up(&peripherals, 57_600);

    // turn on SPI1 for... well, everything else
    Spi1.set_up(&peripherals, BR_A::Div2);

    // blinky blink
    // set PA5 to GPIO output
    peripherals.GPIOA.moder.modify(|_, w| w
        .moder5().output()
    );

    loop {
        // turn on PA5
        peripherals.GPIOA.bsrr.write(|w| w
            .bs5().set_bit()
        );

        // wait
        for _ in 0..1*1024*1024 {
            cortex_m::asm::nop();
        }

        // turn off PA5
        peripherals.GPIOA.bsrr.write(|w| w
            .br5().set_bit()
        );

        // wait
        for _ in 0..1*1024*1024 {
            cortex_m::asm::nop();
        }
    }
}
