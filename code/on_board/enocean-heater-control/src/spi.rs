use stm32g0b0::Peripherals;
use stm32g0b0::spi1::cr1::BR_A;


pub trait Spi {
    fn enable_clocks(&self, peripherals: &Peripherals);
    fn set_up_pins(&self, peripherals: &Peripherals);
    fn get_register_block<'a>(&self, peripherals: &'a Peripherals) -> &'a stm32g0b0::spi1::RegisterBlock;

    fn set_up(&self, peripherals: &Peripherals, clock_divider: BR_A) {
        self.enable_clocks(peripherals);
        self.set_up_pins(peripherals);

        let spi = self.get_register_block(peripherals);

        // disable SPI (and I2S, to ensure that no conflict with SPI arises)
        spi.cr1.modify(|_, w| w
            .spe().clear_bit()
        );
        spi.i2scfgr.modify(|_, w| w
            .i2se().clear_bit()
        );

        // set up SPI
        spi.cr1.modify(|_, w| w
            .cpha().clear_bit() // sample at first clock edge (in our case: 0 -> 1)
            .cpol().clear_bit() // clock to 0 when idle
            .mstr().set_bit() // enable master mode
            .lsbfirst().clear_bit() // transmit MSB first
            .ssi().clear_bit() // no internal slave select
            .ssm().set_bit() // software slave management (we select chips manually)
            .rxonly().clear_bit() // send and receive
            .crcen().clear_bit() // no hardware CRC calculation
            .bidimode().clear_bit() // no bidirectional mode; use CIPO and COPI pins
            .br().variant(clock_divider)
        );
        spi.cr2.modify(|_, w| w
            .rxdmaen().clear_bit() // don't use DMA for receive buffer
            .txdmaen().clear_bit() // don't use DMA for transmit buffer
            .nssp().clear_bit() // no ~{CS} pulse
            .frf().clear_bit() // Motorola mode (keep ~{CS} down for whole transfer instead of pulsing before transfer; see ARM manual "ddi0194")
            .errie().clear_bit() // disable interrupt on error
            .rxneie().clear_bit() // disable interrupt on Receive Buffer Not Empty
            .txeie().clear_bit() // disable interrupt on Transmit Buffer Empty
            .ds().bits8() // 8 bits of data per transfer
        );

        // alright, turn on the SPI
        spi.cr1.modify(|_, w| w
            .spe().set_bit()
        );
    }

    /// Sends data from the buffer and writes the responses into the buffer.
    fn exchange_data(&self, peripherals: &Peripherals, bytes: &mut [u8]) {
        let spi = self.get_register_block(peripherals);

        while spi.sr.read().txe().bit_is_clear() {
            // previous byte is still being sent
        }

        for b in bytes {
            // move byte to transmission register
            spi.dr.write(|w| w
                .dr().variant((*b).into())
            );

            // wait for it to be transmitted
            while spi.sr.read().txe().bit_is_clear() {
            }

            // wait for a byte to be received
            while spi.sr.read().rxne().bit_is_clear() {
            }

            // read it
            *b = (spi.dr.read().dr().bits() & 0xFF) as u8;
        }

        // wait for transmission to complete fully
        while spi.sr.read().bsy().bit_is_set() {
        }
    }
}


/// SPI1 with default pins (PB5=COPI, PB4=CIPO, PB3=SCK)
pub struct Spi1;
impl Spi for Spi1 {
    fn enable_clocks(&self, peripherals: &Peripherals) {
        // give clock to GPIOB because we are using its pins
        peripherals.RCC.iopenr.modify(|_, w| w
            .gpioben().set_bit()
        );
        // give clock to SPI1
        peripherals.RCC.apbenr2.modify(|_, w| w
            .spi1en().set_bit()
        );
    }

    fn set_up_pins(&self, peripherals: &Peripherals) {
        // hand over PB3, PB4 and PB5 to SPI1
        peripherals.GPIOB.moder.modify(|_, w| w
            .moder3().alt_func()
            .moder4().alt_func()
            .moder5().alt_func()
        );
        peripherals.GPIOA.afrl.modify(|_, w| w
            .afsel3().alt_func0()
            .afsel4().alt_func0()
            .afsel5().alt_func0()
        );
    }

    fn get_register_block<'a>(&self, peripherals: &'a Peripherals) -> &'a stm32g0b0::spi1::RegisterBlock {
        &peripherals.SPI1
    }
}
