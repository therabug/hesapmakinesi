use gtk4::prelude::*;
use libadwaita::{
    AboutDialog, Application, ApplicationWindow, HeaderBar, WindowTitle, prelude::AdwDialogExt,
};

const APP_ID: &str = "com.rabug.hesapmakinesi";

const WIDTH: i32 = 400;
const HEIGHT: i32 = 450;

const OPERATIONS: [&str; 4] = ["+", "-", "*", "/"];

fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run();
}

fn parse(text: &str) -> Vec<String> {
    let mut parsed_data: Vec<String> = Vec::new();
    let mut current_number = String::new();

    for ch in text.chars() {
        match ch {
            '0'..='9' | '.' => {
                current_number.push(ch);
            }
            '+' | '-' | '*' | '/' => {
                if !current_number.is_empty() {
                    parsed_data.push(std::mem::take(&mut current_number));
                }
                parsed_data.push(ch.to_string());
            }
            ' ' => {
                if !current_number.is_empty() {
                    parsed_data.push(std::mem::take(&mut current_number));
                }
            }
            _ => {
                if !current_number.is_empty() {
                    parsed_data.push(std::mem::take(&mut current_number));
                }
            }
        }
    }

    if !current_number.is_empty() {
        parsed_data.push(current_number);
    }

    parsed_data
}

fn calculate(tokens: Vec<String>) -> Result<f64, String> {
    let mut output_queue: Vec<String> = Vec::new();
    let mut operator_stack: Vec<String> = Vec::new();

    let precedence = |op: &str| -> i32 {
        match op {
            "+" | "-" => 1,
            "*" | "/" => 2,
            _ => 0,
        }
    };

    let is_operator = |token: &str| -> bool { matches!(token, "+" | "-" | "*" | "/") };

    let mut prev_token: Option<String> = None;

    for token in tokens {
        if let Ok(_) = token.parse::<f64>() {
            output_queue.push(token.clone());
        } else if is_operator(&token) {
            if let Some(prev) = &prev_token {
                if is_operator(prev) && token != "-" && token != "+" {
                    return Err(format!("İki operatör arka arkaya: {} {}", prev, token));
                }
            }

            let is_unary = match &prev_token {
                None => true,
                Some(t) if t == "(" || is_operator(t) => true,
                _ => false,
            };

            if is_unary && (token == "-" || token == "+") {
                operator_stack.push(format!("u{}", token));
            } else {
                while let Some(top) = operator_stack.last() {
                    if is_operator(top) && precedence(top) >= precedence(&token) {
                        output_queue.push(operator_stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                operator_stack.push(token.clone());
            }
        } else if token == "(" {
            operator_stack.push(token.clone());
        } else if token == ")" {
            while let Some(top) = operator_stack.pop() {
                if top == "(" {
                    break;
                } else {
                    output_queue.push(top);
                }
            }
        } else {
            return Err(format!("Beklenmeyen token: {}", token));
        }

        prev_token = Some(token);
    }

    while let Some(op) = operator_stack.pop() {
        if op == "(" || op == ")" {
            return Err("Parantez uyumsuzluğu!".to_string());
        }
        output_queue.push(op);
    }

    let mut stack: Vec<f64> = Vec::new();

    for token in output_queue {
        if let Ok(num) = token.parse::<f64>() {
            stack.push(num);
        } else if token == "u-" {
            let val = stack.pop().ok_or("Eksik operand (unary -)".to_string())?;
            stack.push(-val);
        } else if token == "u+" {
            let val = stack.pop().ok_or("Eksik operand (unary +)".to_string())?;
            stack.push(val);
        } else if is_operator(&token) {
            let b = stack.pop().ok_or("Eksik operand (b)".to_string())?;
            let a = stack.pop().ok_or("Eksik operand (a)".to_string())?;
            let result = match token.as_str() {
                "+" => a + b,
                "-" => a - b,
                "*" => a * b,
                "/" => a / b,
                _ => return Err(format!("Bilinmeyen operatör: {}", token)),
            };
            stack.push(result);
        } else {
            return Err(format!("Bilinmeyen token: {}", token));
        }
    }

    if stack.len() != 1 {
        return Err("Geçersiz ifade veya eksik operand!".to_string());
    }

    Ok(stack.pop().unwrap())
}

fn build_ui(app: &Application) {
    let header_bar = HeaderBar::builder()
        .title_widget(&WindowTitle::new("Hesap Makinesi", ""))
        .build();

    let info_button = gtk4::Button::builder()
        .icon_name("help-about-symbolic")
        .build();

    header_bar.pack_end(&info_button);

    let provider = gtk4::CssProvider::new();

    provider.load_from_data(include_str!("app.css"));

    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().unwrap(),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let app_content = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .build();

    let appco_clone = app_content.clone();

    info_button.connect_clicked(move |_| {
        let about_window = AboutDialog::builder()
            .application_name("Hesap Makinesi")
            .copyright("MIT License")
            .developer_name("therabug")
            .license_type(gtk4::License::MitX11)
            .website("https://rabug.space")
            .build();

        about_window.present(Some(&appco_clone));
    });

    let content = gtk4::Box::builder()
        .orientation(gtk4::Orientation::Vertical)
        .margin_start(10)
        .margin_bottom(10)
        .margin_end(10)
        .build();

    app_content.append(&header_bar);
    app_content.append(&content);

    let buttons = gtk4::Grid::builder()
        .row_spacing(5)
        .column_spacing(5)
        .hexpand(true)
        .vexpand(true)
        .halign(gtk4::Align::Fill)
        .valign(gtk4::Align::Fill)
        .build();

    let ekran = gtk4::Entry::builder()
        .editable(true)
        .margin_bottom(5)
        .margin_top(5)
        .hexpand(true)
        .build();

    ekran.set_css_classes(&["ekran"]);

    for i in 0..9 {
        let button = gtk4::Button::builder()
            .label(&format!("{}", i + 1))
            .hexpand(true)
            .vexpand(true)
            .build();

        let col = i % 3;
        let row = i / 3;

        buttons.attach(&button, col, row, 1, 1);

        let ekran_clone = ekran.clone();
        button.connect_clicked(move |_| {
            insert_text(&ekran_clone, &format!("{}", (i + 1)));
        });
    }

    let button_0 = gtk4::Button::builder()
        .label("0")
        .hexpand(true)
        .vexpand(true)
        .build();

    let ekran_clone = ekran.clone();
    button_0.connect_clicked(move |_| {
        insert_text(&ekran_clone, "0");
    });

    let button_equal = gtk4::Button::builder().label("=").build();

    let ekran_clone = ekran.clone();
    button_equal.connect_clicked(move |_| {
        let sonuc = calculate(parse(ekran_clone.text().as_str()));
        match sonuc {
            Ok(val) => ekran_clone.set_text(val.to_string().as_str()),
            Err(e) => ekran_clone.set_text(&format!("{}", e)),
        }
    });

    let button_c = gtk4::Button::builder().label("C").build();

    let ekran_clone = ekran.clone();
    button_c.connect_clicked(move |_| {
        ekran_clone.set_text("");
    });

    button_equal.set_css_classes(&["suggested-action", "title-2"]);
    button_c.set_css_classes(&["destructive-action"]);

    buttons.attach(&button_0, 1, 3, 1, 1);
    buttons.attach(&button_c, 0, 3, 1, 1);
    buttons.attach(&button_equal, 2, 3, 1, 1);

    for i in 0..OPERATIONS.len() {
        let button = gtk4::Button::builder()
            .label(OPERATIONS[i])
            .vexpand(true)
            .hexpand(true)
            .build();

        button.add_css_class("opaque");

        let ekran_clone = ekran.clone();
        button.connect_clicked(move |_| {
            insert_text(&ekran_clone, OPERATIONS[i]);
        });

        buttons.attach(&button, 4, i as i32, 1, 1);
    }

    content.append(&ekran);
    content.append(&buttons);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Hesap Makinesi")
        .default_width(WIDTH)
        .default_height(HEIGHT)
        .width_request(WIDTH)
        .height_request(HEIGHT)
        .content(&app_content)
        .build();

    window.present();
}

fn insert_text(entry: &gtk4::Entry, text: &str) {
    let current = entry.text();
    entry.set_text(&format!("{}{}", current, text));
}
