# WS2818 RGB LED SPI Driver

This crate is a driver for WS2818 RGB LED (chains). They are also known as "NeoPixel" devices. It's a simple, educational
example how to bring your LEDs to life. This [0] is an example device with chained WS2818 RGB LEDs that can be used with this driver.
This driver only works on Linux systems with an SPI device, like Raspberry Pi [1]. This is needed my driver operates at
15.6MHz. This is mandatory because I need to reach specific *timings in nanoseconds* according to the specification [2].

The SPI device in your Raspberry Pi has a reliable clock with high frequencies available. Regular GPIO pins 
**won't work!** Toggling GPIO pins takes 1µs (in my testing) which is *WAY TOO SLOW!* Therefore I use SPI.

Find the `MOSI`-Pin on your device (e.g. Raspberry Pi) and connect it with `DIN`-Port of the LED. That's all what's needed.

Have a look into the examples/code for further explications. :)

##### Links

[0] https://www.az-delivery.de/products/u-64-led-panel?variant=6127700738075
[1] https://www.raspberrypi.org/documentation/hardware/raspberrypi/spi/README.md
[2] https://cdn-shop.adafruit.com/datasheets/WS2812.pdf
