use zwp_virtual_keyboard_service::{KeyState, VKService, VirtualKeyboard};

mod wayland;

fn main() {
    println!("Start");

    let (event_queue, seat, vk_mgr) = wayland::init_wayland();
    let vk_service = VKService::new(event_queue, &seat, vk_mgr);

    // Press K
    let keycode = input_event_codes::KEY_K!();
    let desired_key_state = KeyState::Pressed;
    println!("Press {}", keycode);
    let submission_result = vk_service.send_key(keycode, desired_key_state);
    if submission_result.is_err() {
        println!("Error: {:?}", submission_result);
    };

    let desired_key_state = KeyState::Released;
    println!("Release {}", keycode);
    let submission_result = vk_service.send_key(keycode, desired_key_state);
    if submission_result.is_err() {
        println!("Error: {:?}", submission_result);
    };
}
