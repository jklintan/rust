#![windows_subsystem = "windows"]
/**
    A simple calculator in Rust with GUI
*/
use native_windows_gui as nwg;
use std::rc::Rc;

fn create_button(text: &str, parent: &nwg::Window) -> nwg::Button{
    let mut btn = Default::default();
    nwg::Button::builder()
        .text(text)
        .parent(parent)
        .build(&mut btn)
        .unwrap();
    btn
}

fn main() {
    // Initialize the GUI
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    // Create window
    let mut window = Default::default();
    let mut input = Default::default();
    let layout = Default::default();

    // Create the GUI
    nwg::Window::builder()
        .size((300, 400))
        .position((300, 300))
        .title("Calculator")
        .build(&mut window)
        .unwrap();

    nwg::TextInput::builder()
        .text("")
        .parent(&window)
        .readonly(true)
        .build(&mut input);

    // Create buttons
    let mut one_btn = create_button("1", &window);
    let mut two_btn = create_button("2", &window);
    let mut three_btn = create_button("3", &window);
    let mut plus_btn = create_button("+", &window);
    let mut four_btn = create_button("4", &window);
    let mut five_btn = create_button("5", &window);
    let mut six_btn = create_button("6", &window);
    let mut sev_btn = create_button("7", &window);
    let mut eight_btn = create_button("8", &window);
    let mut nine_btn = create_button("9", &window);
    let mut zero_btn = create_button("0", &window);
    let mut min_btn = create_button("-", &window);
    let mut div_btn = create_button("/", &window);
    let mut mult_btn = create_button("*", &window);
    let mut eq_btn = create_button("=", &window);
    let mut erase_btn = create_button("C", &window);

    // Build window
    nwg::GridLayout::builder()
        .parent(&window)
        .spacing(1)
        .child_item(nwg::GridLayoutItem::new(&input, 0, 0, 4, 1))
        .child_item(nwg::GridLayoutItem::new(&one_btn, 0, 1, 1, 1))
        .child_item(nwg::GridLayoutItem::new(&two_btn, 1, 1, 1, 1))
        .child_item(nwg::GridLayoutItem::new(&three_btn, 2, 1, 1, 1))
        .child_item(nwg::GridLayoutItem::new(&plus_btn, 3, 1, 1, 1))
        .child_item(nwg::GridLayoutItem::new(&four_btn, 0, 2, 1, 1))
        .child_item(nwg::GridLayoutItem::new(&five_btn, 1, 2, 1, 1))
        .child_item(nwg::GridLayoutItem::new(&six_btn, 2, 2, 1, 1))
        .child_item(nwg::GridLayoutItem::new(&min_btn, 3, 2, 1, 1))
        .child_item(nwg::GridLayoutItem::new(&sev_btn, 0, 3, 1, 1))
        .child_item(nwg::GridLayoutItem::new(&eight_btn, 1, 3, 1, 1))
        .child_item(nwg::GridLayoutItem::new(&nine_btn, 2, 3, 1, 1))
        .child_item(nwg::GridLayoutItem::new(&mult_btn, 3, 3, 1, 1))
        .child_item(nwg::GridLayoutItem::new(&zero_btn, 0, 4, 1, 1))
        .child_item(nwg::GridLayoutItem::new(&erase_btn, 1, 4, 1, 1))
        .child_item(nwg::GridLayoutItem::new(&eq_btn, 2, 4, 1, 1))
        .child_item(nwg::GridLayoutItem::new(&div_btn, 3, 4, 1, 1))
        .build(&layout)
        .unwrap();

    let window = Rc::new(window);
    let events_window = window.clone();

    let handler = nwg::full_bind_event_handler(&window.handle, move |evt, _evt_data, handle| {
        use nwg::Event as E;

        match evt {
            E::OnWindowClose => 
                if &handle == &events_window as &nwg::Window {
                    nwg::stop_thread_dispatch();
                },
            E::OnButtonClick => 
                if &handle == &one_btn {
                    nwg::modal_info_message(&events_window.handle, "Hello", &format!("Hello {}", input.text()));
                },
            _ => {}
        }
    });

    nwg::dispatch_thread_events();
    nwg::unbind_event_handler(&handler);
}