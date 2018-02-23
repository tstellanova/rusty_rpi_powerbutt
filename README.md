# rusty_rpi_powerbutt
Power button for Raspberry Pi 3 using Rust

Use a hardware power button attached to two GPIO pins to:
- Indicate that the RPi has power
- Allow the user to gracefully shutdown the RPi

### Dependencies
- Currently we utilize `pigpiod` to allow any process to access the GPIO pins, and to eg drive the power indication light with PWM 

### Hardware required
- We use a combo LED + SPST switch similar to the (E-Switch B200202)[http://spec_sheets.e-switch.com/specs/B200202.pdf]
- The switch is wired as follows:
  - LED is wired to +Vcc on one side and GPIO output pin on the other. Setting the GPIO pin low causes the LED to light up.
  - One side of the SPST switch is attached to a GPIO input pin. The other is also attached to V+ 
- See `main.rs` for the GPIO pins used.
- When the button is pressed, the GPIO input pin is driven high.

 
