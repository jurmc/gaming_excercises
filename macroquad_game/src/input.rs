use macroquad::prelude::*;
use gilrs::{Gilrs, GamepadId, Button, Axis};

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct InputState {
    pub direction: Option<Direction>,
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
        let mut direction = None;
        let mut trigger = false;
        let mut escape = false;

        if is_key_pressed(KeyCode::Up) {
            direction = Some(Direction::Up);
        }
        if is_key_pressed(KeyCode::Down) {
            direction = Some(Direction::Down);
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
                direction = Some(Direction::Up);
            }
            if gamepad.is_pressed(Button::DPadDown) {
                direction = Some(Direction::Down);
            }
            if gamepad.is_pressed(Button::DPadLeft) {
                direction = Some(Direction::Left);
            }
            if gamepad.is_pressed(Button::DPadRight) {
                direction = Some(Direction::Right);
            }
            if gamepad.is_pressed(Button::RightTrigger2) || gamepad.is_pressed(Button::South) {
                trigger = true;
            }

            let left_stick_x = gamepad.value(Axis::LeftStickX);
            let left_stick_y = gamepad.value(Axis::LeftStickY);
            let stick_sensitivity = 0.5;

            if left_stick_x < -stick_sensitivity {
                direction = Some(Direction::Left);
            } else if left_stick_x > stick_sensitivity {
                direction = Some(Direction::Right);
            }

            if left_stick_y < -stick_sensitivity {
                direction = Some(Direction::Down);
            } else if left_stick_y > stick_sensitivity {
                direction = Some(Direction::Up);
            }

            println!("LX: {}, LY: {}", gamepad.value(Axis::LeftStickX), gamepad.value(Axis::LeftStickY));
            let code = gamepad.axis_code(Axis::LeftStickX);
            if let Some(code) = code {
                println!("deadzone: LX: {:?}", gamepad.deadzone(code));
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
