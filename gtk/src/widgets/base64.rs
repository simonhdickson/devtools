use gtk::prelude::*;
use gtk_macros::get_widget;

pub struct Base64Widget {
    pub widget: libhandy::Column,
    pub builder: gtk::Builder,
}

impl Base64Widget {
    pub fn new() -> Self {
        let builder = gtk::Builder::new_from_resource("/com/belmoussaoui/ReadItLater/widgets/base64.ui");

        get_widget!(builder, libhandy::Column, login);

        Self { widget: login, builder }
    }
}
