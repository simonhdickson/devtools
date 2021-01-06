use gtk::glib;
use gtk::prelude::*;

use std::{env::args, sync::{Arc, RwLock}};

use devtools_core::encoding::{self, Kind, ViewModel};

use gtk::{
    Application, ApplicationWindow, Builder, TextView
};

pub fn build_ui(application: &Application) {
    let ui_src = include_str!("../ui/base64.xml");
    let builder = Builder::new();

    builder
        .add_from_string(ui_src)
        .expect("Couldn't add from string");

    let window: ApplicationWindow = builder.get_object("window").expect("Couldn't get window");
    window.set_application(Some(application));

    let input_view: TextView = builder
        .get_object("input_view")
        .expect("Couldn't get input_view");
        
    let output_view: TextView = builder
        .get_object("output_view")
        .expect("Couldn't get output_view");

    let mut m = encoding::create();
    m.set_kind(Kind::Base64);
    
    let m = Arc::new(RwLock::new(m));

    let m1 = m.clone();

    let input_buffer = input_view.get_buffer();

    input_buffer.connect_changed(glib::clone!(@weak window, @weak output_view => move |input_buffer| {
        let input_str = input_buffer.get_text(&input_buffer.get_start_iter(), &input_buffer.get_end_iter(), true);
        let mut m = m1.write().unwrap();
        
        m.set_plain_text(input_str.as_str());
        
        let output_buffer = output_view.get_buffer();
        output_buffer.set_text(&*m.encoded_text().unwrap());
    }));

    window.show();
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
