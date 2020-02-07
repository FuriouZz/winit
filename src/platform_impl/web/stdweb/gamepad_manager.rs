use super::gamepad;
use super::utils;
use crate::event::device;
use std::{cell::RefCell, rc::Rc};

pub struct Shared(Rc<GamepadManager>);

pub struct GamepadManager {
    gamepads: RefCell<Vec<gamepad::Gamepad>>,
}

impl Shared {
    pub fn create() -> Shared {
        Shared(Rc::new(GamepadManager {
            gamepads: RefCell::new(Vec::new()),
        }))
    }

    pub fn manager(&self) -> Rc<GamepadManager> {
        self.0.clone()
    }
}

impl Clone for Shared {
    fn clone(&self) -> Self {
        Shared(self.0.clone())
    }
}

impl GamepadManager {
    pub fn register(&self, gamepad: stdweb::web::Gamepad) -> gamepad::Gamepad {
        let mut gamepads = self.gamepads.borrow_mut();
        let w = gamepad::Gamepad::new(gamepad);
        if !gamepads.contains(&w) {
            gamepads.push(w.clone());
        }
        w
    }

    pub fn collect_new(&self) -> Vec<gamepad::Gamepad> {
        let gamepads = self.gamepads.borrow();

        gamepads
            .iter()
            .map(|g| gamepad::Gamepad::new(g.raw()))
            .collect()
    }

    pub fn collect_events(&self, events: &mut Vec<(gamepad::Gamepad, device::GamepadEvent)>) {
        let old_gamepads = self.gamepads.borrow().clone();
        let new_gamepads = self.collect_new();

        match (old_gamepads.get(0), new_gamepads.get(0)) {
            (Some(old), Some(new)) => {
                // Button events
                let buttons = old.mapping.buttons().zip(new.mapping.buttons()).enumerate();
                for (btn_index, (old_button, new_button)) in buttons {
                    match (old_button, new_button) {
                        (false, true) => {
                            events.push((new.clone(), utils::gamepad_button(btn_index, true)))
                        }
                        (true, false) => {
                            events.push((new.clone(), utils::gamepad_button(btn_index, false)))
                        }
                        _ => (),
                    }
                }

                // Axis events
                let axes = old.mapping.axes().zip(new.mapping.axes()).enumerate();
                for (axis_index, (old_axis, new_axis)) in axes {
                    if old_axis != new_axis {
                        events.push((new.clone(), utils::gamepad_axis(axis_index, new_axis)))
                    }
                }

                // Stick events
                let mut old_axes = old.mapping.axes();
                let mut new_axes = new.mapping.axes();

                let old_left = (old_axes.next(), old_axes.next());
                let new_left = (new_axes.next(), new_axes.next());
                if old_left != new_left {
                    if let (Some(x), Some(y)) = (new_left.0, new_left.1) {
                        events.push((
                            new.clone(),
                            utils::gamepad_stick(0, 1, x, y, device::Side::Left),
                        ));
                    }
                }

                let old_right = (old_axes.next(), old_axes.next());
                let new_right = (new_axes.next(), new_axes.next());
                if old_right != new_right {
                    if let (Some(x), Some(y)) = (new_right.0, new_right.1) {
                        events.push((
                            new.clone(),
                            utils::gamepad_stick(2, 3, x, y, device::Side::Right),
                        ));
                    }
                }
            }
            _ => {}
        }

        self.gamepads.replace(new_gamepads);
        // super::log(&format!("{:?}", events).to_string());
    }
}
