# rusty_rpi_powerbutt
Power button for Raspberry Pi 3 using Rust

Use a hardware power button attached to two GPIO pins to:
- Indicate that the RPi has power
- Allow the user to gracefully shutdown the RPi

### Dependencies
- Currently we utilize `pigpiod` to allow any process to access the GPIO pins, and to eg drive the power indication light with PWM 

