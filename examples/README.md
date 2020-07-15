# Examples for WS2818 RGB LED SPI Driver

Make sure to check out [main README.md](../README.md) first.
Here are a few basic examples! Make sure to run them with SPI enabled on 
your device (Raspberry Pi) and the MOSI-pin beeing connected to DIN-port.

**All examples I made are primarily tested on a 8x8 square matrix!**
With a few adjustments you can use them on other chained WS281xx LEDs as well.


## How to run them?
1) Setup Rust + Cargo ;)
2) `cargo run --bin moving-pixel 64` 
   or `cargo run --bin strobo-disco-light 64`
   (there are more examples too in src/bin)
