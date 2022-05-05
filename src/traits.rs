use super::SubmitError;
use wayland_client::{protocol::wl_seat::WlSeat, EventQueue, Main};

use zwp_virtual_keyboard::virtual_keyboard_unstable_v1::zwp_virtual_keyboard_manager_v1::ZwpVirtualKeyboardManagerV1;

use super::{KeyCode, KeyState};

/// All input methods must be able to handle these functions
/// This helps write test cases, because they can be generic
pub trait VirtualKeyboard {
    fn new(
        event_queue: EventQueue,
        seat: &WlSeat,
        vk_manager: Main<ZwpVirtualKeyboardManagerV1>,
    ) -> Self;

    fn send_key(&self, keycode: KeyCode, desired_key_state: KeyState) -> Result<(), SubmitError>;

    //fn toggle_shift(&mut self) -> Result<(), SubmitError>;
}
