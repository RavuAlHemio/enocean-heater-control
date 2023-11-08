| role               | peripheral     | pin  | mux setting | notes  |
| ------------------ | -------------- | ---- | ----------- | ------ |
| OSC32K IN          | n/a            | PA00 | n/a         |        |
| OSC32K OUT         | n/a            | PA01 | n/a         |        |
| blocked for OSC32K | n/a, GND       | PA02 | n/a         |        |
| blocked for OSC32K | n/a, GND       | PA03 | n/a         |        |
| debug UART µC-Tx   | SERCOM0/PAD[0] | PA04 | D           | TXPO=0 |
| debug UART µC-Rx   | SERCOM0/PAD[1] | PA05 | D           | RXPO=1 |
| OLED ~CS           | PORT           | PA06 | n/a         |        |
| OLED D/~C          | PORT           | PA07 | n/a         |        |
| OLED EN            | PORT           | PA08 | n/a         |        |
| keypad ~CS         | PORT           | PA09 | n/a         |        |
| flash ~CS          | PORT           | PA10 | n/a         |        |
| flash ~write-prot  | PORT           | PA11 | n/a         |        |
| SPI COPI           | SERCOM2/PAD[0] | PA12 | C           | DOPO=0 |
| SPI CIPO           | SERCOM2/PAD[1] | PA13 | C           | DIPO=1 |
| SPI SCK            | SERCOM2/PAD[2] | PA14 | C           | DOPO=0 |
| EnOcean UART µC-Tx | SERCOM1/PAD[0] | PA16 | C           | TXPO=0 |
| EnOcean UART µC-Rx | SERCOM1/PAD[1] | PA17 | C           | RXPO=1 |
| debug SWCLK        | n/a            | PA30 | n/a         |        |
| debug SWDIO        | n/a            | PA31 | n/a         |        |
| blocked for OSC32K | n/a, GND       | PB02 | n/a         |        |
| blocked for OSC32K | n/a, GND       | PB03 | n/a         |        |
