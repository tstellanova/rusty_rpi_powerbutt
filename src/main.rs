extern crate rustgpio;
extern crate runas;

use rustgpio::pigpio;

use std::thread::sleep;
use std::time::Duration;
use runas::Command;

// We use pigpio to generate and monitor signals on GPIO pins


// the pins where the power LED and switch are attached
const LED_GPIO_PIN: u32 = 18;
const BUTT_IN_PIN: u32 = 17;

const PWM_FULL_RANGE: u32 = 1000;
const LED_PWM_FREQ_HZ: u32 = 2000;
const FADE_STEP_DELAY_MS: u64 = 100;
const FADE_STEPS: u32 = 25;
const FADE_STEP_VAL: u32 = (PWM_FULL_RANGE / FADE_STEPS);

pub struct BoardController {
    pi: pigpio::Pi,
}

impl BoardController {

pub fn new() -> BoardController {
    BoardController {
      pi: pigpio::Pi::new() 
    }
}

fn led_on(&self, duration_secs: u64) {
  // the LED safety switch we're using is already tied to +VDC: set GPIO pin low to turn on
  self.pi.write(LED_GPIO_PIN, 0);
  sleep(Duration::from_secs(duration_secs));
}

fn led_off(&self, duration_secs: u64) {
  // the LED safety switch I'm using is already tied to +VDC:  set GPIO pin high to turn off
  self.pi.write(LED_GPIO_PIN, 1);
  sleep(Duration::from_secs(duration_secs));
}

fn fade_led_down(&self, pin: u32 ) {
  self.pi.set_pwm_frequency(pin, LED_PWM_FREQ_HZ); 
  self.pi.set_pwm_range(pin, PWM_FULL_RANGE); // Set range to 1000. 1 range = 2 us;

  for x in 0..FADE_STEPS {
    let duty_cycle = x * FADE_STEP_VAL;
    self.pi.set_pwm_dutycycle(pin, duty_cycle); 
    sleep(Duration::from_millis(FADE_STEP_DELAY_MS))
  }
}

fn fade_led_up(&self, pin: u32) {
  self.pi.set_pwm_frequency(pin, LED_PWM_FREQ_HZ);
  self.pi.set_pwm_range(pin, PWM_FULL_RANGE); // Set range to 1000. 1 range = 2 us;

  for x in 0..FADE_STEPS {
    let duty_cycle = PWM_FULL_RANGE - x * FADE_STEP_VAL;
    self.pi.set_pwm_dutycycle(pin, duty_cycle);  
    sleep(Duration::from_millis(FADE_STEP_DELAY_MS))
  }
}


/*
   gpio: see description
PWMfreq: 0 (off) or 1-125000000 (125M)
PWMduty: 0 (off) to 1000000 (1M)(fully on)
*/

fn led_fade_cycle(&self, count: u32) {
  for _x in 0..count {
    self.fade_led_up( LED_GPIO_PIN);
    self.fade_led_down( LED_GPIO_PIN);
  }
}

fn wait_for_shutdown(&self, cb_fn: pigpio::CallbackFn) {

  self.pi.callback(17, 0, cb_fn); 
  loop {
    self.led_fade_cycle(1);
  }

}

}//BoardController

fn button_press_cb(_gpio: u32,
    _edge: u32,
    _bit: u32) {

  println!("Shutdown button pressed!");	

  let status = Command::new("shutdown").arg("-h").arg("now").status().expect("failed to shut down!");
  //it's unlikely the following will ever be printed
  println!("shutdown exited with: {}", status);
}

fn main() {
  let bc = BoardController::new();
  println!("Initialized pigpiod. ");

  bc.pi.set_mode(LED_GPIO_PIN,  pigpio::OUTPUT );
  // GPIO 17 set up as an input, pulled down, connected to 3V3 on button press
  bc.pi.set_mode(BUTT_IN_PIN,  pigpio::PUD_DOWN);

  //TODO: use event_detect instead?
  // rpi_gpio.add_event_detect(17, rpi_gpio.RISING, callback=int_callback, bouncetime=300)

  bc.led_on(1);
  bc.led_off(1);

  bc.wait_for_shutdown(button_press_cb)
}
