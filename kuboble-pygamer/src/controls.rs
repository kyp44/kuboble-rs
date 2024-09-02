use embedded_graphics::prelude::Point;
use embedded_graphics::text::renderer;
use heapless::String;
use kuboble_core::Direction;
use pygamer::adc::Adc;
use pygamer::hal::hal::blocking;
use pygamer::pac::ADC1;
use pygamer::pins::JoystickReader;
use pygamer::prelude::*;

const JOYSTICK_THRESH: i16 = 5;

trait JoystickReaderExt {
    fn direction(&mut self, adc: &mut Adc<ADC1>) -> Option<Direction>;
}
impl JoystickReaderExt for JoystickReader {
    fn direction(&mut self, adc: &mut Adc<ADC1>) -> Option<Direction> {
        let raw = self.read(adc);
        let (x, y) = (raw.0 as i16 - 2048, raw.1 as i16 - 2048);

        if y < -JOYSTICK_THRESH {
            Some(Direction::Up)
        } else if y > JOYSTICK_THRESH {
            Some(Direction::Down)
        } else if x < -JOYSTICK_THRESH {
            Some(Direction::Left)
        } else if x > JOYSTICK_THRESH {
            Some(Direction::Right)
        } else {
            None
        }
    }
}

pub enum ControlAction {
    Move(Direction),
    A,
    B,
    Start,
    Select,
}

pub struct Controller {
    joystick_adc: Adc<ADC1>,
    joystick_reader: JoystickReader,
}
impl Controller {
    pub fn new(joystick_adc: Adc<ADC1>, joystick_reader: JoystickReader) -> Self {
        Self {
            joystick_adc,
            joystick_reader,
        }
    }
    pub fn wait_for_action<D: _embedded_hal_blocking_delay_DelayMs<u32>>(
        &mut self,
        delay: &mut D,
    ) -> ControlAction {
        loop {
        delay.delay_ms(50);
         if let Soem( self.joystick_reader.read(&mut self.joystick_adc)
        }
    }
}
