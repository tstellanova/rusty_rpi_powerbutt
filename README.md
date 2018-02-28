# rusty_rpi_powerbutt
Power button for Raspberry Pi 3 using Rust

Use a hardware power button attached to two GPIO pins to:
- Indicate that the RPi has power
- Allow the user to gracefully shutdown the RPi

Also, detect low power condition (where the USB supply voltage drops below +5V)
and shut down the RPi3 gracefully.

### Dependencies
- Currently we utilize `pigpiod` to allow any process to access the GPIO pins, and to eg drive the power indication light with PWM 
- TODO currently we require a special fork of the `rustgpio` crate that supports PWM in order to drive the LED fade up/down.   
- Using the (rp3_firmware_access)[https://github.com/tstellanova/rpi3_firmware_access] crate to allow us to detect the low power condition.
 
### Hardware required
- RPi3 or similar (untested on other hardware) 
- We use a combo LED + SPST switch similar to the [E-Switch B200202](http://spec_sheets.e-switch.com/specs/B200202.pdf)
- The switch is wired as follows:
  - LED is wired to +Vcc on one side and GPIO output pin on the other. Setting the GPIO pin low causes the LED to light up.
  - One side of the SPST switch is attached to a GPIO input pin. The other is also attached to V+ 
- See `main.rs` for the GPIO pins used.
- When the button is pressed, the GPIO input pin is driven high.

 
