## Pinout
(~ means that the pin can be moved)
### ENC28j60
| |   |     |
|-|---|-----|
|~|PA3|reset|
| |PA4|ncs  |
| |PA5|sck  |
| |PA6|miso |
| |PA7|mosi |
### LED
| |   |     |
|-|---|-----|
|~|PA2|LED  |

## Timer
The WS2812B needs to be able to receive a message between 0.275us and 0.425us, so we'll need to configure an interrupt between 2.3MHz and 3.6 MHz.

The code uses a TIM2 timer at 3 MHz.
