use gtk::prelude::*;
use zwp_virtual_keyboard_service::{KeyState, VKService, VirtualKeyboard};

mod wayland;

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("First GTK+ Program");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 70);

    let button = gtk::Button::with_label("Click me!");

    window.add(&button);

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

    // Release K
    let desired_key_state = KeyState::Released;
    println!("Release {}", keycode);
    let submission_result = vk_service.send_key(keycode, desired_key_state);
    if submission_result.is_err() {
        println!("Error: {:?}", submission_result);
    };

    window.show_all();
}

fn main() {
    let application =
        gtk::Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default());

    application.connect_activate(build_ui);

    application.run();
}
