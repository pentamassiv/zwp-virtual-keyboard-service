//! This crate provides an easy to use interface for the zwp_input_method_v2 protocol.
//! It allows a wayland client to serve as an input method for other wayland-clients. This could be used for virtual keyboards
//!
#[cfg(feature = "debug")]
#[macro_use]
extern crate log;

use std::sync::{Arc, Mutex};
use wayland_client::Dispatch;
use wayland_client::{protocol::wl_seat::WlSeat, EventQueue};
use wayland_protocols_misc::zwp_virtual_keyboard_v1::client::zwp_virtual_keyboard_manager_v1::ZwpVirtualKeyboardManagerV1;
use wayland_protocols_misc::zwp_virtual_keyboard_v1::client::zwp_virtual_keyboard_v1::ZwpVirtualKeyboardV1;

mod keymap;

mod traits;
pub use traits::*;

use arc_vk::*;
mod arc_vk;

pub type KeyCode = u32;

#[derive(Debug, Clone)]
/// Error when sending a request to the wayland-client
pub enum SubmitError {
    /// Input method was not activ
    NotAlive,
}

#[derive(Debug, Clone, Copy)]
pub enum KeyState {
    Pressed = 1,
    Released = 0,
}

#[derive(Clone, Debug)]
/// Manages the pending state and the current state of the input method.
pub struct VKService {
    vk_service_arc: Arc<Mutex<VKServiceArc>>, // provides an easy to use interface by hiding the Arc<Mutex<>>
}

impl VirtualKeyboard for VKService {
    fn new(
        event_queue: EventQueue<dyn Dispatch<ZwpVirtualKeyboardV1>>,
        seat: &WlSeat,
        vk_manager: ZwpVirtualKeyboardManagerV1,
    ) -> Self {
        let vk_service_arc = VKServiceArc::new(event_queue, seat, vk_manager);
        Self { vk_service_arc }
    }

    fn send_key(&self, keycode: KeyCode, desired_key_state: KeyState) -> Result<(), SubmitError> {
        self.vk_service_arc
            .lock()
            .unwrap()
            .send_key(keycode, desired_key_state)
    }
}
