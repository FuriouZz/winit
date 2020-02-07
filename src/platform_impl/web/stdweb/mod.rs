mod canvas;
mod utils;
mod timeout;
pub mod gamepad;

pub use self::canvas::Canvas;
pub use self::timeout::Timeout;

use crate::dpi::LogicalSize;
use crate::platform::web::WindowExtStdweb;
use crate::window::Window;

use stdweb::js;
use stdweb::web::event::BeforeUnloadEvent;
use stdweb::web::window;
use stdweb::web::IEventTarget;
use stdweb::web::{document, html_element::CanvasElement, Element};

pub fn throw(msg: &str) {
    js! { throw @{msg} }
}

pub fn exit_fullscreen() {
    document().exit_fullscreen();
}

pub fn on_unload(mut handler: impl FnMut() + 'static) {
    window().add_event_listener(move |_: BeforeUnloadEvent| handler());
}

impl WindowExtStdweb for Window {
    fn canvas(&self) -> CanvasElement {
        self.window.canvas().raw().clone()
    }
}

pub fn window_size() -> LogicalSize {
    let window = window();
    let width = window.inner_width() as f64;
    let height = window.inner_height() as f64;

    LogicalSize { width, height }
}

pub fn is_fullscreen(canvas: &CanvasElement) -> bool {
    match document().fullscreen_element() {
        Some(elem) => {
            let raw: Element = canvas.clone().into();
            raw == elem
        }
        None => false,
    }
}

pub fn get_gamepads() -> Vec<gamepad::Gamepad> {
    stdweb::web::Gamepad::get_all()
    .into_iter()
    .filter_map(|gamepad| gamepad.map(|gamepad| gamepad::Gamepad::new(gamepad)))
    .collect()
}