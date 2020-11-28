use glib::clone;
use gtk::prelude::*;
use gtk_macros::get_widget;
use libhandy::{SqueezerExt, ViewSwitcherBarExt};

pub struct Window {
    pub widget: gtk::ApplicationWindow,
}

impl Window {
    pub fn new() -> Self {
        let builder = gtk::Builder::new_from_resource("/com/simonhdickson/devtools/window.ui");

        get_widget!(builder, gtk::ApplicationWindow, window);

        get_widget!(builder, libhandy::Squeezer, squeezer);

        get_widget!(builder, libhandy::ViewSwitcher, headerbar_switcher);

        get_widget!(builder, libhandy::ViewSwitcherBar, bottom_switcher);

        squeezer.connect_property_visible_child_notify(
            clone!(@weak squeezer, @weak headerbar_switcher, @weak bottom_switcher => move |_| {
                if let Some(child) = squeezer.get_visible_child() {
                    bottom_switcher.set_reveal(child != headerbar_switcher);
                }
            }),
        );

        Self { widget: window }
    }
}
