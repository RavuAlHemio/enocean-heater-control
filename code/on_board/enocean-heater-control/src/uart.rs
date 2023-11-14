use stm32g0b0::Peripherals;

use crate::setup::SYSTEM_CLOCK_SPEED;


pub trait Uart {
    fn enable_clocks(&self, peripherals: &Peripherals);
    fn set_up_pins(&self, peripherals: &Peripherals);
    fn get_register_block<'a>(&self, peripherals: &'a Peripherals) -> &'a stm32g0b0::usart1::RegisterBlock;

    fn set_up(&self, peripherals: &Peripherals, baud_rate: u32) {
        self.enable_clocks(peripherals);
        self.set_up_pins(peripherals);

        let usart = self.get_register_block(peripherals);

        // set up USART as UART (without FIFO mode)
        usart.cr1_fifo_disabled().modify(|_, w| w
            .ue().clear_bit() // disable USART1
            .uesm().clear_bit() // disable USART1 in low-power mode
            .re().clear_bit() // disable receiver
            .te().set_bit() // enable transmitter
            .idleie().clear_bit() // disable interrupt on IDLE
            .rxneie().clear_bit() // disable interrupt on RX [register] Not Empty
            .tcie().clear_bit() // disable interrupt on Transmission Complete
            .txeie().clear_bit() // disable interrupt on TX [register] Empty
            .peie().clear_bit() // disable interrupt on Parity Error
            .ps().clear_bit() // Even Parity (doesn't matter, we are disabling it anyway)
            .pce().clear_bit() // disable parity
            .wake().clear_bit() // wakeup from Mute mode on idle line (unused anyway)
            .m0().clear_bit() // 1 start bit, 8 data bits (part 1)
            .m1().clear_bit() // 1 start bit, 8 data bits (part 2)
            .mme().clear_bit() // disable Mute mode
            .cmie().clear_bit() // disable interrupt on character match
            .over8().clear_bit() // oversample by 16
            // keep DEDT and DEAT unchanged
            .rtoie().clear_bit() // disable interrupt on Receiver Timeout
            .eobie().clear_bit() // disable interrupt on End Of Block
            .fifoen().clear_bit() // disable FIFO mode
        );
        usart.cr2.modify(|_, w| w
            .slven().clear_bit() // disable synchronous slave mode
            .lbdie().clear_bit() // disable interrupt on LIN Break Detection
            .clken().clear_bit() // disable SCLK pin
            .stop().one() // one stop bit
            .linen().clear_bit() // disable LIN mode
            .swap().clear_bit() // don't swap Rx and Tx
            .rxinv().clear_bit() // don't assume Rx is inverted
            .txinv().clear_bit() // don't invert Tx
            .datainv().clear_bit() // don't invert data
            .msbfirst().clear_bit() // send LSB first (as expected by RS232)
            .abren().clear_bit() // don't do automatic baud rate detection
            .rtoen().clear_bit() // don't do receiver timeout
        );
        usart.cr3.modify(|_, w| w
            .eie().clear_bit() // disable error interrupt
            .iren().clear_bit() // disable IrDA mode
            .hdsel().clear_bit() // full duplex (if you actually do half-duplex, we cannot be friends)
            .nack().clear_bit() // no smartcard NACK
            .scen().clear_bit() // disable smartcard mode
            .dmar().clear_bit() // disable receiver DMA
            .dmat().clear_bit() // disable transmitter DMA
            .rtse().clear_bit() // disable RTS (ready-to-receive) flow control
            .ctse().clear_bit() // disable CTS (clear-to-send) flow control
            .ctsie().clear_bit() // disable interrupt on clear-to-send
            .onebit().clear_bit() // use 3-bit sample mode
            .ovrdis().set_bit() // ignore receive buffer overruns
            .dem().clear_bit() // disable external transceiver control
            .wufie().clear_bit() // disable interrupt on wake-up from low power state
            .txftie().clear_bit() // disable interrupt on TX FIFO threshold reached
            .tcbgtie().clear_bit() // disable interrupt on transmission complete before guard time reached
            .rxftie().clear_bit() // disable interrupt on RX FIFO threshold reached
        );

        // baud rate calculation
        // USARTDIV = usart_ker_ckpres / baud
        // where usart_ker_ckpres = usart_ker_ck prescaled by value in PRESC
        // assuming usart_ker_ck = usart_pclk = sysclk = 64 MHz
        // and desired baud rate = 115200 bps (bps = Hz)
        // => USARTDIV = 555
        let baud_divider = SYSTEM_CLOCK_SPEED / baud_rate;
        debug_assert!(baud_divider <= 0xFFFF);
        usart.brr.modify(|_, w| w
            .brr().variant((baud_divider & 0xFFFF) as u16)
        );

        // alright, turn on the UART
        usart.cr1_fifo_disabled().modify(|_, w| w
            .ue().set_bit() // USART enable
        );
    }

    fn send(&self, peripherals: &Peripherals, bytes: &[u8]) {
        let usart = self.get_register_block(peripherals);

        while usart.isr_fifo_disabled().read().txe().bit_is_clear() {
            // previous byte is still being sent
        }

        for &b in bytes {
            // move byte to transmission register
            usart.tdr.write(|w| w
                .tdr().variant(b.into())
            );

            // wait for it to be transmitted
            while usart.isr_fifo_disabled().read().txe().bit_is_clear() {
            }
        }

        // wait for transmission to complete fully
        while usart.isr_fifo_disabled().read().tc().bit_is_clear() {
        }
    }
}


/// UART on USART1 with default pins (PA9=Tx, PA10=Rx)
pub struct Uart1;
impl Uart for Uart1 {
    fn enable_clocks(&self, peripherals: &Peripherals) {
        // give clock to GPIOA because we are using its pins
        peripherals.RCC.iopenr.modify(|_, w| w
            .gpioaen().set_bit()
        );
        // give clock to USART1
        peripherals.RCC.apbenr2.modify(|_, w| w
            .usart1en().set_bit()
        );
    }

    fn set_up_pins(&self, peripherals: &Peripherals) {
        // hand over PA9 and PA10 to USART1
        peripherals.GPIOA.moder.modify(|_, w| w
            .moder9().alt_func()
            .moder10().alt_func()
        );
        peripherals.GPIOA.afrh.modify(|_, w| w
            .afsel9().alt_func1()
            .afsel10().alt_func1()
        );
    }

    fn get_register_block<'a>(&self, peripherals: &'a Peripherals) -> &'a stm32g0b0::usart1::RegisterBlock {
        &peripherals.USART1
    }
}


/// UART on USART2 with default pins (PA2=Tx, PA3=Rx)
pub struct Uart2;
impl Uart for Uart2 {
    fn enable_clocks(&self, peripherals: &Peripherals) {
        // give clock to GPIOA because we are using its pins
        peripherals.RCC.iopenr.modify(|_, w| w
            .gpioaen().set_bit()
        );
        // give clock to USART2
        peripherals.RCC.apbenr1.modify(|_, w| w
            .usart2en().set_bit()
        );
    }

    fn set_up_pins(&self, peripherals: &Peripherals) {
        // hand over PA2 and PA3 to USART2
        peripherals.GPIOA.moder.modify(|_, w| w
            .moder2().alt_func()
            .moder3().alt_func()
        );
        peripherals.GPIOA.afrl.modify(|_, w| w
            .afsel2().alt_func1()
            .afsel3().alt_func1()
        );
    }

    fn get_register_block<'a>(&self, peripherals: &'a Peripherals) -> &'a stm32g0b0::usart1::RegisterBlock {
        &peripherals.USART2
    }
}
