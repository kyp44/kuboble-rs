use core::cell::RefCell;

use kuboble_core::level_run::Direction;
use pygamer::adc::Adc;
use pygamer::delay::Delay;
use pygamer::pac::ADC1;
use pygamer::pins::{ButtonReader, JoystickReader, Keys};
use pygamer::prelude::*;
use pygamer_engine::{ControlAction, Controller};

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

pub struct PyGamerController<'a> {
    delay: &'a RefCell<Delay>,
    joystick_adc: Adc<ADC1>,
    joystick_reader: JoystickReader,
    button_reader: ButtonReader,
    last_direction: Option<Direction>,
}
impl<'a> PyGamerController<'a> {
    pub fn new(
        delay: &'a RefCell<Delay>,
        joystick_adc: Adc<ADC1>,
        joystick_reader: JoystickReader,
        button_reader: ButtonReader,
    ) -> Self {
        Self {
            delay,
            joystick_adc,
            joystick_reader,
            button_reader,
            last_direction: None,
        }
    }
}
impl Controller for PyGamerController<'_> {
    fn wait_for_action(&mut self) -> Option<pygamer_engine::ControlAction> {
        loop {
            self.delay.borrow_mut().delay_ms(50u8);

            // Need to debounce the joystick
            let old_direction = self.last_direction;
            let new_direction = self.joystick_reader.direction(&mut self.joystick_adc);
            self.last_direction = new_direction;

            if new_direction != old_direction
                && let Some(dir) = new_direction
            {
                break Some(ControlAction::Move(dir));
            }
            for key in self.button_reader.events() {
                return Some(match key {
                    Keys::SelectDown => ControlAction::Select,
                    Keys::StartDown => ControlAction::Start,
                    Keys::BDown => ControlAction::B,
                    Keys::ADown => ControlAction::A,
                    _ => continue,
                });
            }
        }
    }
}
