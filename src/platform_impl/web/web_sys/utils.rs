use crate::dpi::LogicalPosition;
use crate::event::{ModifiersState, MouseButton, ScanCode, VirtualKeyCode};
use crate::platform_impl::platform;

use std::convert::TryInto;
use web_sys::{Gamepad, GamepadButton, GamepadMappingType, KeyboardEvent, MouseEvent, WheelEvent};

pub fn mouse_button(event: &MouseEvent) -> MouseButton {
    match event.button() {
        0 => MouseButton::Left,
        1 => MouseButton::Middle,
        2 => MouseButton::Right,
        i => MouseButton::Other((i - 3).try_into().expect("very large mouse button value")),
    }
}

pub fn mouse_modifiers(event: &MouseEvent) -> ModifiersState {
    ModifiersState {
        shift: event.shift_key(),
        ctrl: event.ctrl_key(),
        alt: event.alt_key(),
        logo: event.meta_key(),
    }
}

pub fn mouse_position(event: &MouseEvent) -> LogicalPosition {
    LogicalPosition {
        x: event.offset_x() as f64,
        y: event.offset_y() as f64,
    }
}

pub fn mouse_scroll_delta(event: &WheelEvent) -> (f64, f64) {
    let x = event.delta_x();
    let y = event.delta_y();
    (x, y)
}

pub fn scan_code(event: &KeyboardEvent) -> ScanCode {
    match event.key_code() {
        0 => event.char_code(),
        i => i,
    }
}

pub fn virtual_key_code(event: &KeyboardEvent) -> Option<VirtualKeyCode> {
    Some(match &event.code()[..] {
        "Digit1" => VirtualKeyCode::Key1,
        "Digit2" => VirtualKeyCode::Key2,
        "Digit3" => VirtualKeyCode::Key3,
        "Digit4" => VirtualKeyCode::Key4,
        "Digit5" => VirtualKeyCode::Key5,
        "Digit6" => VirtualKeyCode::Key6,
        "Digit7" => VirtualKeyCode::Key7,
        "Digit8" => VirtualKeyCode::Key8,
        "Digit9" => VirtualKeyCode::Key9,
        "Digit0" => VirtualKeyCode::Key0,
        "KeyA" => VirtualKeyCode::A,
        "KeyB" => VirtualKeyCode::B,
        "KeyC" => VirtualKeyCode::C,
        "KeyD" => VirtualKeyCode::D,
        "KeyE" => VirtualKeyCode::E,
        "KeyF" => VirtualKeyCode::F,
        "KeyG" => VirtualKeyCode::G,
        "KeyH" => VirtualKeyCode::H,
        "KeyI" => VirtualKeyCode::I,
        "KeyJ" => VirtualKeyCode::J,
        "KeyK" => VirtualKeyCode::K,
        "KeyL" => VirtualKeyCode::L,
        "KeyM" => VirtualKeyCode::M,
        "KeyN" => VirtualKeyCode::N,
        "KeyO" => VirtualKeyCode::O,
        "KeyP" => VirtualKeyCode::P,
        "KeyQ" => VirtualKeyCode::Q,
        "KeyR" => VirtualKeyCode::R,
        "KeyS" => VirtualKeyCode::S,
        "KeyT" => VirtualKeyCode::T,
        "KeyU" => VirtualKeyCode::U,
        "KeyV" => VirtualKeyCode::V,
        "KeyW" => VirtualKeyCode::W,
        "KeyX" => VirtualKeyCode::X,
        "KeyY" => VirtualKeyCode::Y,
        "KeyZ" => VirtualKeyCode::Z,
        "Escape" => VirtualKeyCode::Escape,
        "F1" => VirtualKeyCode::F1,
        "F2" => VirtualKeyCode::F2,
        "F3" => VirtualKeyCode::F3,
        "F4" => VirtualKeyCode::F4,
        "F5" => VirtualKeyCode::F5,
        "F6" => VirtualKeyCode::F6,
        "F7" => VirtualKeyCode::F7,
        "F8" => VirtualKeyCode::F8,
        "F9" => VirtualKeyCode::F9,
        "F10" => VirtualKeyCode::F10,
        "F11" => VirtualKeyCode::F11,
        "F12" => VirtualKeyCode::F12,
        "F13" => VirtualKeyCode::F13,
        "F14" => VirtualKeyCode::F14,
        "F15" => VirtualKeyCode::F15,
        "F16" => VirtualKeyCode::F16,
        "F17" => VirtualKeyCode::F17,
        "F18" => VirtualKeyCode::F18,
        "F19" => VirtualKeyCode::F19,
        "F20" => VirtualKeyCode::F20,
        "F21" => VirtualKeyCode::F21,
        "F22" => VirtualKeyCode::F22,
        "F23" => VirtualKeyCode::F23,
        "F24" => VirtualKeyCode::F24,
        "PrintScreen" => VirtualKeyCode::Snapshot,
        "ScrollLock" => VirtualKeyCode::Scroll,
        "Pause" => VirtualKeyCode::Pause,
        "Insert" => VirtualKeyCode::Insert,
        "Home" => VirtualKeyCode::Home,
        "Delete" => VirtualKeyCode::Delete,
        "End" => VirtualKeyCode::End,
        "PageDown" => VirtualKeyCode::PageDown,
        "PageUp" => VirtualKeyCode::PageUp,
        "ArrowLeft" => VirtualKeyCode::Left,
        "ArrowUp" => VirtualKeyCode::Up,
        "ArrowRight" => VirtualKeyCode::Right,
        "ArrowDown" => VirtualKeyCode::Down,
        "Backspace" => VirtualKeyCode::Back,
        "Enter" => VirtualKeyCode::Return,
        "Space" => VirtualKeyCode::Space,
        "Compose" => VirtualKeyCode::Compose,
        "Caret" => VirtualKeyCode::Caret,
        "NumLock" => VirtualKeyCode::Numlock,
        "Numpad0" => VirtualKeyCode::Numpad0,
        "Numpad1" => VirtualKeyCode::Numpad1,
        "Numpad2" => VirtualKeyCode::Numpad2,
        "Numpad3" => VirtualKeyCode::Numpad3,
        "Numpad4" => VirtualKeyCode::Numpad4,
        "Numpad5" => VirtualKeyCode::Numpad5,
        "Numpad6" => VirtualKeyCode::Numpad6,
        "Numpad7" => VirtualKeyCode::Numpad7,
        "Numpad8" => VirtualKeyCode::Numpad8,
        "Numpad9" => VirtualKeyCode::Numpad9,
        "AbntC1" => VirtualKeyCode::AbntC1,
        "AbntC2" => VirtualKeyCode::AbntC2,
        "NumpadAdd" => VirtualKeyCode::Add,
        "Quote" => VirtualKeyCode::Apostrophe,
        "Apps" => VirtualKeyCode::Apps,
        "At" => VirtualKeyCode::At,
        "Ax" => VirtualKeyCode::Ax,
        "Backslash" => VirtualKeyCode::Backslash,
        "Calculator" => VirtualKeyCode::Calculator,
        "Capital" => VirtualKeyCode::Capital,
        "Semicolon" => VirtualKeyCode::Semicolon,
        "Comma" => VirtualKeyCode::Comma,
        "Convert" => VirtualKeyCode::Convert,
        "NumpadDecimal" => VirtualKeyCode::Decimal,
        "NumpadDivide" => VirtualKeyCode::Divide,
        "Equal" => VirtualKeyCode::Equals,
        "Backquote" => VirtualKeyCode::Grave,
        "Kana" => VirtualKeyCode::Kana,
        "Kanji" => VirtualKeyCode::Kanji,
        "AltLeft" => VirtualKeyCode::LAlt,
        "BracketLeft" => VirtualKeyCode::LBracket,
        "ControlLeft" => VirtualKeyCode::LControl,
        "ShiftLeft" => VirtualKeyCode::LShift,
        "MetaLeft" => VirtualKeyCode::LWin,
        "Mail" => VirtualKeyCode::Mail,
        "MediaSelect" => VirtualKeyCode::MediaSelect,
        "MediaStop" => VirtualKeyCode::MediaStop,
        "Minus" => VirtualKeyCode::Minus,
        "NumpadMultiply" => VirtualKeyCode::Multiply,
        "Mute" => VirtualKeyCode::Mute,
        "LaunchMyComputer" => VirtualKeyCode::MyComputer,
        "NavigateForward" => VirtualKeyCode::NavigateForward,
        "NavigateBackward" => VirtualKeyCode::NavigateBackward,
        "NextTrack" => VirtualKeyCode::NextTrack,
        "NoConvert" => VirtualKeyCode::NoConvert,
        "NumpadComma" => VirtualKeyCode::NumpadComma,
        "NumpadEnter" => VirtualKeyCode::NumpadEnter,
        "NumpadEquals" => VirtualKeyCode::NumpadEquals,
        "OEM102" => VirtualKeyCode::OEM102,
        "Period" => VirtualKeyCode::Period,
        "PlayPause" => VirtualKeyCode::PlayPause,
        "Power" => VirtualKeyCode::Power,
        "PrevTrack" => VirtualKeyCode::PrevTrack,
        "AltRight" => VirtualKeyCode::RAlt,
        "BracketRight" => VirtualKeyCode::RBracket,
        "ControlRight" => VirtualKeyCode::RControl,
        "ShiftRight" => VirtualKeyCode::RShift,
        "MetaRight" => VirtualKeyCode::RWin,
        "Slash" => VirtualKeyCode::Slash,
        "Sleep" => VirtualKeyCode::Sleep,
        "Stop" => VirtualKeyCode::Stop,
        "NumpadSubtract" => VirtualKeyCode::Subtract,
        "Sysrq" => VirtualKeyCode::Sysrq,
        "Tab" => VirtualKeyCode::Tab,
        "Underline" => VirtualKeyCode::Underline,
        "Unlabeled" => VirtualKeyCode::Unlabeled,
        "AudioVolumeDown" => VirtualKeyCode::VolumeDown,
        "AudioVolumeUp" => VirtualKeyCode::VolumeUp,
        "Wake" => VirtualKeyCode::Wake,
        "WebBack" => VirtualKeyCode::WebBack,
        "WebFavorites" => VirtualKeyCode::WebFavorites,
        "WebForward" => VirtualKeyCode::WebForward,
        "WebHome" => VirtualKeyCode::WebHome,
        "WebRefresh" => VirtualKeyCode::WebRefresh,
        "WebSearch" => VirtualKeyCode::WebSearch,
        "WebStop" => VirtualKeyCode::WebStop,
        "Yen" => VirtualKeyCode::Yen,
        _ => return None,
    })
}

pub fn keyboard_modifiers(event: &KeyboardEvent) -> ModifiersState {
    ModifiersState {
        shift: event.shift_key(),
        ctrl: event.ctrl_key(),
        alt: event.alt_key(),
        logo: event.meta_key(),
    }
}

pub fn codepoint(event: &KeyboardEvent) -> char {
    // `event.key()` always returns a non-empty `String`. Therefore, this should
    // never panic.
    // https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key
    event.key().chars().next().unwrap()
}

pub fn create_mapping(raw: &Gamepad) -> platform::device::gamepad::Mapping {
    match raw.mapping() {
        GamepadMappingType::Standard => {
            let mut buttons = [false; 17];
            let mut axes = [0.0; 4];

            let gbuttons = raw.buttons();
            for index in 0..buttons.len() {
                let button: GamepadButton = gbuttons.get(index as u32).into();
                buttons[index] = button.pressed();
            }

            let gaxes = raw.axes();
            for index in 0..axes.len() {
                let axe: f64 = gaxes.get(index as u32).as_f64().unwrap_or(0.0);
                axes[index] = axe;
            }

            platform::device::gamepad::Mapping::Standard { buttons, axes }
        }
        _ => {
            let mut buttons: Vec<bool> = Vec::new();
            let mut axes: Vec<f64> = Vec::new();

            let gbuttons = raw.buttons();
            for index in 0..gbuttons.length() {
                let button: GamepadButton = gbuttons.get(index as u32).into();
                buttons.push(button.pressed());
            }

            let gaxes = raw.axes();
            for index in 0..gaxes.length() {
                let axe: f64 = gaxes.get(index as u32).as_f64().unwrap_or(0.0);
                axes.push(axe);
            }

            platform::device::gamepad::Mapping::NoMapping { buttons, axes }
        }
    }
}
