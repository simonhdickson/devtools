use std::{env::args, sync::{Arc, RwLock}};

use devtools_core::{encoding::{self, Kind, EncodingViewModel}, unix_time::{self, UnixTimeViewModel}};

use gtk::*;
use gtk::prelude::*;

pub fn build_ui(application: &Application) {
    let window = ApplicationWindowBuilder::new()
        .application(application)
        .title("devtools")
        .default_width(400)
        .default_height(600)
        .build();

    let view_selector = ComboBoxTextBuilder::new()
        .build();

    let encoding_box = build_encoding_ui();
    let unix_time_box = build_unix_time_ui();

    view_selector.append_text("Encoding");
    view_selector.append_text("Unix Time");
    view_selector.set_active(Some(0));

    window.set_child(Some(&encoding_box));

    view_selector.connect_changed(glib::clone!(@weak window => move |combo_box| {
        match combo_box.get_active_text().unwrap().as_str() {
            "Encoding" => window.set_child(Some(&encoding_box)),
            "Unix Time" => window.set_child(Some(&unix_time_box)),
            _ => (),
        }
    }));

    let title_bar = HeaderBarBuilder::new()
        .show_title_buttons(true)
        .build();

    title_bar.pack_start(&view_selector);

    window.set_titlebar(Some(&title_bar));
    
    window.show();
}

fn build_unix_time_ui() -> Box {
    let template = BoxBuilder::new()
        .orientation(Orientation::Vertical)
        .build();

    let now_button = ButtonBuilder::new()
        .label("Time Now")
        .build();

    let unix_time_view = TextViewBuilder::new()
        .wrap_mode(WrapMode::WordChar)
        .build();

    let utc_text_view = TextViewBuilder::new()
        .wrap_mode(WrapMode::WordChar)
        .editable(false)
        .build();

    template.append(&now_button);

    template.append(&unix_time_view);

    template.append(&utc_text_view);

    let m = unix_time::create();
    
    let m = Arc::new(RwLock::new(m));

    let m1 = m.clone();

    let input_buffer = unix_time_view.get_buffer();

    input_buffer.connect_changed(glib::clone!(@weak utc_text_view => move |input_buffer| {
        let input_str = input_buffer.get_text(&input_buffer.get_start_iter(), &input_buffer.get_end_iter(), true);
        let mut m = m1.write().unwrap();
        
        m.set_unix_time_string(input_str.as_str().to_owned());
        
        let output_buffer = utc_text_view.get_buffer();
        if let Ok(output) = m.get_utc_time() {
            output_buffer.set_text(&*output);
        } else {
            output_buffer.set_text("");
        }
    }));

    let m1 = m.clone();

    now_button.connect_clicked(glib::clone!(@weak input_buffer => move |now_button| {
        let new_text = {
            let mut m = m1.write().unwrap();      
            m.set_unix_time_to_now();
            &*m.get_unix_time().to_string()
        };
        
        input_buffer.set_text(new_text);
    }));

    template
}

fn build_encoding_ui() -> Box {
    let template = BoxBuilder::new()
        .orientation(Orientation::Vertical)
        .build();

    let plain_text_view = TextViewBuilder::new()
        .wrap_mode(WrapMode::WordChar)
        .build();

    let plain_text_window = ScrolledWindowBuilder::new()
        .hexpand(true)
        .vexpand(true)
        .child(&plain_text_view)
        .build();

    let encoded_text_view = TextViewBuilder::new()
        .wrap_mode(WrapMode::Char)
        .build();

    let encoded_text_window = ScrolledWindowBuilder::new()
        .hexpand(true)
        .vexpand(true)
        .child(&encoded_text_view)
        .build();

    let mut m = encoding::create();

    let encoding_selector = ComboBoxTextBuilder::new()
        .build();

    encoding_selector.append_text("Base64");
    encoding_selector.append_text("RFC4648 Base32");
    encoding_selector.append_text("Crockford Base32");
    encoding_selector.set_active(Some(0));

    m.set_kind(Kind::Base64);
    
    let m = Arc::new(RwLock::new(m));

    let m1 = m.clone();

    encoding_selector.connect_changed(glib::clone!(@weak encoded_text_view => move |combo_box| {
        let mut m = m1.write().unwrap();

        match combo_box.get_active_text().unwrap().as_str() {
            "Base64" => m.set_kind(Kind::Base64),
            "RFC4648 Base32" => m.set_kind(Kind::RFC4648Base32),
            "Crockford Base32" => m.set_kind(Kind::CrockfordBase32),
            _ => unreachable!(),
        }
        
        let output_buffer = encoded_text_view.get_buffer();
        output_buffer.set_text(&*m.encoded_text().unwrap());
    }));

    template.append(&encoding_selector);

    template.append(&plain_text_window);

    template.append(&encoded_text_window);

    let m1 = m.clone();

    let input_buffer = plain_text_view.get_buffer();

    input_buffer.connect_changed(glib::clone!(@weak encoded_text_view => move |input_buffer| {
        let input_str = input_buffer.get_text(&input_buffer.get_start_iter(), &input_buffer.get_end_iter(), true);
        let mut m = m1.write().unwrap();
        
        m.set_plain_text(input_str.as_str());
        
        let output_buffer = encoded_text_view.get_buffer();
        output_buffer.set_text(&*m.encoded_text().unwrap());
    }));

    template
}

fn main() {
    let application = Application::new(
        Some("com.simonhdickson.devtools"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
