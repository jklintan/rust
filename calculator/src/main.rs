#![windows_subsystem = "windows"]
/**
    A simple calculator in Rust with GUI
*/
use native_windows_gui as nwg;
use std::rc::Rc;

const N_BUTTONS: usize = 16;

struct Calculator {
    var_x: f64,
    var_y: f64,
    ans: f64,
    operator: String,
    text_field: String,

    width: i32,
    height: i32,
}

impl Calculator {
    // Static method, constructor
    fn new(w: i32 , h: i32) -> Calculator {
        Calculator {
            var_x: 0.0, 
            var_y: 0.0, 
            ans: 0.0, 
            operator: String::from(""),
            text_field: String::from(""),
            width: w, 
            height: h
        }
    }

    // Input detected
    fn update_x(&mut self, x: f64) {
        self.var_x = x;
    }

    // Input detected
    fn update_y(&mut self, y: f64) {
        self.var_y = y;
    }

    fn calculate(&mut self) {
        if self.var_x != 0.0 && self.var_y != 0.0 {
            if self.operator != "" {
                if self.operator == "+" {
                    self.ans = add(self.var_x, self.var_y);
                } else if self.operator == "-" {
                    self.ans = subtract(self.var_x, self.var_y);
                }
                // TODO: Add other operators
            }
        }
    }


}

fn add(x: f64, y: f64) -> f64 {
    return x + y;
}

fn subtract(x: f64, y: f64) -> f64 {
    return x - y
}

fn create_button(text: &str, parent: &nwg::Window) -> nwg::Button{
    let mut btn = Default::default();
    nwg::Button::builder()
        .text(text)
        .parent(parent)
        .build(&mut btn)
        .unwrap();
    btn
}

fn create_buttons(btn_texts: Vec<&str>, parent: &nwg::Window) -> [nwg::Button; N_BUTTONS]{
    let mut buttons: [nwg::Button; N_BUTTONS] = Default::default();
    let count = 0;
    for label in btn_texts.iter(){
        buttons[count] = create_button(label, parent);
    };
    buttons
}

fn create_textfield(text: &str, parent: &nwg::Window, readonly: bool) -> nwg::TextInput{
    let mut textfield = Default::default();
    nwg::TextInput::builder()
        .text(text)
        .parent(parent)
        .readonly(readonly)
        .build(&mut textfield);
    textfield
}

fn main() {
    // Initialize the GUI
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    // Create window
    let mut window = Default::default();
    let layout = Default::default();

    // Create calculator
    let calculator = Calculator {var_x: 0.0, var_y: 0.0, ans: 0.0, operator: String::from(""), text_field: String::from(""), width: 300, height: 400};

    // Create the GUI
    nwg::Window::builder()
        .size((calculator.width, calculator.height))
        .position((300, 300))
        .title("Calculator")
        .build(&mut window)
        .unwrap();

    // Create input field
    let mut input_field = create_textfield("", &window, true);

    // Create buttons
    let btn_texts = vec!["1", "2", "3", "+", "4", "5", "6", "-", "7", "8", "9", "*", "0", "C", "=", "/"];

    let buttons: [nwg::Button; N_BUTTONS] = create_buttons(btn_texts, &window);

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
        .child_item(nwg::GridLayoutItem::new(&input_field, 0, 0, 4, 1))
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
                    nwg::modal_info_message(&events_window.handle, "Hello", &format!("Hello {}", input_field.text()));
                },
            _ => {}
        }
    });

    nwg::dispatch_thread_events();
    nwg::unbind_event_handler(&handler);
}