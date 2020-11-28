use gettextrs::*;
use gio::prelude::*;
use gtk::prelude::*;

use libhandy::Column;

mod config;
mod window;
mod widgets;
use crate::window::Window;

fn main() {
    gtk::init().unwrap_or_else(|_| panic!("Failed to initialize GTK."));
    Column::new();

    setlocale(LocaleCategory::LcAll, "");
    bindtextdomain("devtools", config::LOCALEDIR);
    textdomain("devtools");

    let res = gio::Resource::load(config::PKGDATADIR.to_owned() + "/devtools.gresource")
        .expect("Could not load resources");
    gio::resources_register(&res);

    let app =
        gtk::Application::new(Some("com.simonhdickson.devtools"), Default::default()).unwrap();

    app.connect_activate(move |app| {
        let window = Window::new();

        window.widget.set_application(Some(app));
        app.add_window(&window.widget);
        window.widget.present();
    });

    let ret = app.run(&std::env::args().collect::<Vec<_>>());

    std::process::exit(ret);
}
