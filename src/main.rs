extern crate pigrust;
extern crate runas;
extern crate rpi3_firmware_access;

use pigrust::board_control::*;

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
const FADE_STEPS: u32 = 12;
const FADE_STEP_VAL: u32 = (PWM_FULL_RANGE / FADE_STEPS);

trait GpioLedTricks {

  fn setup_led_pin(&self);
  fn led_on(&self, duration_secs: u32);
  fn led_off(&self, duration_secs: u32);
  fn fade_led_down(&self, pin: u32 ); 
  fn fade_led_up(&self, pin: u32 );
  fn led_fade_cycle(&self, count: u32);
}


impl GpioLedTricks for BoardController {

  fn setup_led_pin(&self) {
     self.set_gpio_mode(LED_GPIO_PIN, GpioMode::Output);
  }

  fn led_on(&self, duration_secs: u32) {
    // the LED safety switch we're using is already tied to +VDC: set GPIO pin low to turn on
    self.gpio_write(LED_GPIO_PIN, 0);
    sleep(Duration::from_secs(duration_secs as u64));
  }

  fn led_off(&self, duration_secs: u32) {
    // the LED safety switch I'm using is already tied to +VDC:  set GPIO pin high to turn off
    self.gpio_write(LED_GPIO_PIN, 1);
    sleep(Duration::from_secs(duration_secs as u64));
  }

  fn fade_led_down(&self, pin: u32 ) {
    self.set_pwm_frequency(pin, LED_PWM_FREQ_HZ); 
    self.set_pwm_range(pin, PWM_FULL_RANGE); // Set range to 1000. 1 range = 2 us;

    for x in 0..FADE_STEPS {
      let duty_cycle = x * FADE_STEP_VAL;
      self.set_pwm_dutycycle(pin, duty_cycle); 
      sleep(Duration::from_millis(FADE_STEP_DELAY_MS))
    }
  }

  fn fade_led_up(&self, pin: u32) {
    self.set_pwm_frequency(pin, LED_PWM_FREQ_HZ);
    self.set_pwm_range(pin, PWM_FULL_RANGE); // Set range to 1000. 1 range = 2 us;

    for x in 0..FADE_STEPS {
      let duty_cycle = PWM_FULL_RANGE - x * FADE_STEP_VAL;
      self.set_pwm_dutycycle(pin, duty_cycle);  
      sleep(Duration::from_millis(FADE_STEP_DELAY_MS))
    }
  }

  fn led_fade_cycle(&self, count: u32) {
    for _x in 0..count {
      self.fade_led_up( LED_GPIO_PIN);
      self.fade_led_down( LED_GPIO_PIN);
    }
  }


}//GpioLedTricks for BoardController

fn perform_shutdown() {
  println!("shutting down...");
  let status = Command::new("shutdown").arg("-h").arg("now").status().expect("failed to shut down!");
  //it's unlikely the following will ever be printed
  println!("shutdown exited with: {}", status);
}


#[no_mangle]
pub extern fn butt_press_cb_fn(_daemon_id: i32, gpio: u32, _level: u32, _tick: u32, _userdata: u32 ) {
  if BUTT_IN_PIN == gpio {
    println!("Shutdown button pressed!");
    perform_shutdown();
  }
}

fn main() {
  let bc  = BoardController::new();
  println!("Initialized pigpiod. ");

  // GPIO 17 set up as an input, pulled down, connected to 3V3 on button press
  bc.set_gpio_mode(BUTT_IN_PIN,  GpioMode::Input);
  bc.set_pull_up_down(BUTT_IN_PIN, GpioPullOption::Down);

  // setup pigpio waiting for the input GPIO pin to change
  bc.add_edge_detector_closure(BUTT_IN_PIN, GpioEdgeDetect::FallingEdge,
      |gpio, level| {
          println!("main closure! with {} {} ", gpio, level);
	  perform_shutdown();
      }
  );

  bc.setup_led_pin();
  loop {
    bc.led_fade_cycle(1);
    let val = rpi3_firmware_access::utils::get_power_status();
    if 1 != val {
      println!("shutting down on low power!");
      perform_shutdown();
    }
  }


}
