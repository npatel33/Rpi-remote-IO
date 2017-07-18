pub use sysfs_gpio::{Direction, Pin};

/// GPIO module implementation
///
/// # Variables
///
/// * `gpio_map` : stores valid gpio numbers
///
/// # Methods
///
/// * `fn gpio_init_map()` : Initializes Rpi-2 valid GPIOs
/// * `fn gpio_read()` : Reads status of indicated GPIO pin
/// * `fn gpio_set()` : Sends logic high to corresponding GPIO pin
/// * `fn gpio_clear()` : Sets logic low to corresponding GPIO pin

pub struct Gpio {
    gpio_map : [bool;256],
}
impl Gpio {

    /// Initialize valid GPIO map
    ///
    /// # Description
    /// 
    /// It is necessary to check valid GPIO number
    /// before operating it. Otherwise, it can damage
    /// Raspberry Pi if wrong number is provided. This map
    /// holds valid GPIOs.
    
    pub fn gpio_init_map(&mut self)
    {
        self.gpio_map[2] = true; self.gpio_map[3] = true;
        self.gpio_map[4] = true; self.gpio_map[14] = true;
        self.gpio_map[15] = true; self.gpio_map[17] = true;
        self.gpio_map[18] = true; self.gpio_map[27] = true;
        self.gpio_map[22] = true; self.gpio_map[23] = true;
        self.gpio_map[24] = true; self.gpio_map[10] = true;
        self.gpio_map[9] = true; self.gpio_map[25] = true;
        self.gpio_map[11] = true; self.gpio_map[8] = true;
        self.gpio_map[7] = true; self.gpio_map[5] = true;
        self.gpio_map[6] = true; self.gpio_map[12] = true;
        self.gpio_map[13] = true; self.gpio_map[19] = true;
        self.gpio_map[16] = true; self.gpio_map[26] = true;
        self.gpio_map[20] = true; self.gpio_map[21] = true;
    }

    /// Read status/logic of GPIO
    ///
    /// # Arguments
    ///
    /// * `pin` : GPIO pin number
    ///
    /// # Return value
    ///
    /// * `u8` : 0 if low and 1 if high
    
    pub fn gpio_read(&self, pin:u64) -> u8
    {
        /*
         * Check if GPIO num is valid
         */
        if pin < 256 && self.gpio_map[pin as usize] != true {
            println!("Bad GPIO pin!");
            return 0;
        }

        let gpio = Pin::new(pin);

        if gpio.is_exported() != true {
            gpio.export().unwrap();
        }

        gpio.set_direction(Direction::In).unwrap();
        
        return gpio.get_value().unwrap();

    }
    
    /// Set logic of GPIO high
    ///
    /// # Arguments
    ///
    /// * `pin` : GPIO pin number
    ///
    
    pub fn gpio_set(&self, pin:u64)
    {
        /*
         * Check if GPIO num is valid
         */
        if pin < 256 && self.gpio_map[pin as usize] != true {
            println!("Bad GPIO pin!");
            return;
        }

        let gpio = Pin::new(pin);

        if gpio.is_exported() != true {
            gpio.export().unwrap();
        }

        gpio.set_direction(Direction::Out).unwrap();
        gpio.set_value(1).unwrap();
    }
    
    /// Sets logic of GPIO low
    ///
    /// # Arguments
    ///
    /// * `pin` : GPIO pin number
    ///
    
    pub fn gpio_clear(&self, pin:u64)
    {
        /*
         * Check if GPIO num is valid
         */
        if pin < 256 && self.gpio_map[pin as usize] != true {
            println!("Bad GPIO pin!");
            return;
        }

        let gpio = Pin::new(pin);

        if gpio.is_exported() != true {
            gpio.export().unwrap();
        }

        gpio.set_direction(Direction::Out).unwrap();
        gpio.set_value(0).unwrap();
    }
}
