use std::{cmp::PartialEq};
use crate::platform_impl::platform::device;
use super::utils;

#[derive(Debug)]
pub struct Gamepad {
    pub(crate) index: i32,
    pub(crate) raw: stdweb::web::Gamepad,
    pub(crate) mapping: device::gamepad::Mapping,
}

impl Gamepad {
    pub fn new(raw: stdweb::web::Gamepad) -> Self {
        let mapping = utils::create_mapping(&raw);

        Self {
            index: raw.index(),
            raw,
            mapping,
        }
    }

    // An integer that is auto-incremented to be unique for each device
    // currently connected to the system.
    // https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/index
    pub fn index(&self) -> i32 {
        self.raw.index()
    }

    // A string containing some information about the controller.
    // https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/id
    pub fn id(&self) -> String {
        self.raw.id()
    }

    // A boolean indicating whether the gamepad is still connected to the system.
    // https://developer.mozilla.org/en-US/docs/Web/API/Gamepad/connected
    pub fn connected(&self) -> bool {
        self.raw.connected()
    }

    // EXPERIMENTAL
    #[allow(dead_code)]
    pub fn vibrate(&self, _value: f64, _duration: f64) {
        unimplemented!()
    }
}

impl Clone for Gamepad {
    fn clone(&self) -> Self {
        Self {
            index: self.index,
            raw: self.raw.clone(),
            mapping: self.mapping.clone(),
        }
    }
}

impl PartialEq for Gamepad {
    #[inline(always)]
    fn eq(&self, othr: &Self) -> bool {
        self.raw.index() == othr.raw.index()
    }
}