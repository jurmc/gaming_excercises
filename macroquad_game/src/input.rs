use macroquad::prelude::*;
use gilrs::{Gilrs, Button, Event};

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
}

pub fn new() -> Input {
    Input {
        gilrs: Gilrs::new().unwrap(),
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

        // TODO: Examine new events
        let mut active_gamepad = None;
        while let Some(Event { id, event, time, .. }) = self.gilrs.next_event() {
            let text = format!("{:?} New event from {}: {:?}", time, id, event);
            active_gamepad = Some(id);

//            if let Some(id) = active_gamepad {
//                println!("We have gamepad: {}", text);
//            }
        }

        // You can also use cached gamepad state
        if let Some(gamepad) = active_gamepad.map(|id| self.gilrs.gamepad(id)) {
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
        }

        InputState {
            direction,
            trigger,
            escape,
        }
    }
}
