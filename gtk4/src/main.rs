use std::{env::args, sync::{Arc, RwLock}};

use devtools_core::encoding::{self, Kind, ViewModel};

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

    view_selector.append_text("encoding");
    view_selector.set_active(Some(0));

    let title_bar = HeaderBarBuilder::new()
        .show_title_buttons(true)
        .build();

    title_bar.pack_start(&view_selector);

    window.set_titlebar(Some(&title_bar));

    let encoding_box = build_encoding_ui();

    window.set_child(Some(&encoding_box));
    
    window.show();
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
    encoding_selector.append_text("RFC4648Base32");
    encoding_selector.append_text("CrockfordBase32");
    encoding_selector.set_active(Some(0));

    m.set_kind(Kind::Base64);
    
    let m = Arc::new(RwLock::new(m));

    let m1 = m.clone();

    encoding_selector.connect_changed(glib::clone!(@weak encoded_text_view => move |combo_box| {
        let mut m = m1.write().unwrap();

        match combo_box.get_active_text().unwrap().as_str() {
            "Base64" => m.set_kind(Kind::Base64),
            "RFC4648Base32" => m.set_kind(Kind::RFC4648Base32),
            "CrockfordBase32" => m.set_kind(Kind::CrockfordBase32),
            _ => (),
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
