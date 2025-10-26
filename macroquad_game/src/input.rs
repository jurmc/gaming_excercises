use macroquad::prelude::*;
use gilrs::{Gilrs, GamepadId, Button, Axis};

pub struct InputState {
    pub direction: Vec2,
    pub trigger: bool,
    pub escape: bool,
}

pub struct Input {
    gilrs: Gilrs,
    active_gamepad_id: Option<GamepadId>,
}

pub fn new() -> Input {
    let gilrs = Gilrs::new().unwrap();
    let connected_gamepad = gilrs.gamepads().next();
    let active_gamepad_id =
        if let Some((id, _)) = connected_gamepad {
            Some(id)
        } else {
            None
        };

    Input {
        gilrs,
        active_gamepad_id,
    }
}

impl Input {
    pub fn get(&mut self) -> InputState {
        let mut direction = Vec2::new(0.0, 0.0);
        let mut trigger = false;
        let mut escape = false;

        if is_key_pressed(KeyCode::Up) {
            direction.y = -1.0;
        }
        if is_key_pressed(KeyCode::Down) {
            direction.y = 1.0;
        }
        if is_key_pressed(KeyCode::Left) {
            direction.x = -1.0;
        }
        if is_key_pressed(KeyCode::Right) {
            direction.x = 1.0;
        }
        if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
            trigger = true;
        }
        if is_key_pressed(KeyCode::Escape) {
            escape = true;
        }

        if let Some(id) = self.active_gamepad_id {
            let gamepad = self.gilrs.gamepad(id);

            if gamepad.is_pressed(Button::DPadUp) {
                direction.y = -1.0;
            }
            if gamepad.is_pressed(Button::DPadDown) {
                direction.y = 1.0;
            }
            if gamepad.is_pressed(Button::DPadLeft) {
                direction.x = -1.0;
            }
            if gamepad.is_pressed(Button::DPadRight) {
                direction.x = 1.0;
            }
            if gamepad.is_pressed(Button::RightTrigger2) || gamepad.is_pressed(Button::South) {
                trigger = true;
            }

            let left_stick_x = gamepad.value(Axis::LeftStickX);
            let left_stick_y = gamepad.value(Axis::LeftStickY);
            let stick_sensitivity = 0.5;

            if left_stick_x < -stick_sensitivity || left_stick_x > stick_sensitivity {
                direction.x = left_stick_x;
            }

            if left_stick_y < -stick_sensitivity || left_stick_y > stick_sensitivity {
                direction.y = -left_stick_y;
            }
        }

        // Flush all events before next iteration
        while let Some(_) = self.gilrs.next_event() {}

        InputState {
            direction,
            trigger,
            escape,
        }
    }
}
