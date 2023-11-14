# STM32G0B0KE

| role               | peripheral | pin  | mux setting | notes  |
| ------------------ | ---------- | ---- | ----------- | ------ |
| debug UART µC-Tx   | USART1_TX  | PA09 | AF1         |        |
| debug UART µC-Rx   | USART1_RX  | PA10 | AF1         |        |
| OLED ~CS           | GPIO       | PA04 | n/a         |        |
| OLED D/~C          | GPIO       | PB02 | n/a         |        |
| OLED EN            | GPIO       | PA08 | n/a         |        |
| keypad ~CS         | GPIO       | PA11 | n/a         |        |
| flash ~CS          | GPIO       | PA12 | n/a         |        |
| flash ~write-prot  | GPIO       | PA15 | n/a         |        |
| SPI COPI           | SPI1_MOSI  | PB05 | AF0         |        |
| SPI CIPO           | SPI1_MISO  | PB04 | AF0         |        |
| SPI SCK            | SPI1_SCK   | PB03 | AF0         |        |
| EnOcean UART µC-Tx | USART2_TX  | PA02 | AF1         |        |
| EnOcean UART µC-Rx | USART2_RX  | PA03 | AF1         |        |
| debug SWCLK        | debug      | PA14 | AF0         |        |
| debug SWDIO        | debug      | PA13 | AF0         |        |
