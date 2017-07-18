extern crate sysfs_gpio;

use sysfs_gpio::{Direction, Pin};

pub struct Gpio {
    gpio_map : [bool;256],
}
impl Gpio {

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

    pub fn gpio_read(&self, pin:u64) -> u8
    {
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
    pub fn gpio_set(&self, pin:u64)
    {
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
    
    pub fn gpio_clear(&self, pin:u64)
    {
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
