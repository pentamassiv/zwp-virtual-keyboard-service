use super::SubmitError;
use wayland_client::{protocol::wl_seat::WlSeat, Dispatch, EventQueue};

use wayland_protocols_misc::zwp_virtual_keyboard_v1::client::zwp_virtual_keyboard_manager_v1::ZwpVirtualKeyboardManagerV1;
use wayland_protocols_misc::zwp_virtual_keyboard_v1::client::zwp_virtual_keyboard_v1::ZwpVirtualKeyboardV1;

use super::{KeyCode, KeyState};

/// All input methods must be able to handle these functions
/// This helps write test cases, because they can be generic
pub trait VirtualKeyboard {
    fn new(
        event_queue: EventQueue<dyn Dispatch<ZwpVirtualKeyboardV1>>,
        seat: &WlSeat,
        vk_manager: ZwpVirtualKeyboardManagerV1,
    ) -> Self;

    fn send_key(&self, keycode: KeyCode, desired_key_state: KeyState) -> Result<(), SubmitError>;

    //fn toggle_shift(&mut self) -> Result<(), SubmitError>;
}
