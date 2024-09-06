use kuboble_core::board::Direction;
use pygamer::adc::Adc;
use pygamer::pac::ADC1;
use pygamer::pins::{ButtonReader, JoystickReader, Keys};
use pygamer::prelude::*;

const JOYSTICK_THRESH: i16 = 1024;

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
    button_reader: ButtonReader,
    last_direction: Option<Direction>,
}
impl Controller {
    // TODO: Use derive-new for this?
    pub fn new(
        joystick_adc: Adc<ADC1>,
        joystick_reader: JoystickReader,
        button_reader: ButtonReader,
    ) -> Self {
        Self {
            joystick_adc,
            joystick_reader,
            button_reader,
            last_direction: None,
        }
    }
    pub fn wait_for_action<D: _embedded_hal_blocking_delay_DelayMs<u32>>(
        &mut self,
        delay: &mut D,
    ) -> ControlAction {
        loop {
            delay.delay_ms(50);

            // Need to debounce the joystick
            let old_direction = self.last_direction;
            let new_direction = self.joystick_reader.direction(&mut self.joystick_adc);
            self.last_direction = new_direction;

            if new_direction != old_direction
                && let Some(dir) = new_direction
            {
                break ControlAction::Move(dir);
            }
            for key in self.button_reader.events() {
                return match key {
                    Keys::SelectDown => ControlAction::Select,
                    Keys::StartDown => ControlAction::Start,
                    Keys::BDown => ControlAction::B,
                    Keys::ADown => ControlAction::A,
                    _ => continue,
                };
            }
        }
    }

    pub fn wait_for_proceed<D: _embedded_hal_blocking_delay_DelayMs<u32>>(
        &mut self,
        delay: &mut D,
    ) {
        loop {
            match self.wait_for_action(delay) {
                ControlAction::A | ControlAction::Start => break,
                _ => {}
            }
        }
    }
}
