 /*
philologus-gtk-rs - a desktop version of the website philolog.us

Copyright (C) 2021  Jeremy March

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>. 
*/

use gtk::glib;
use gtk::prelude::*;
use std::rc::Rc;

extern crate webkit2gtk;
use webkit2gtk::{traits::WebViewExt, WebView};

fn main() {
    let application = gtk::Application::new(
        Some("com.github.jeremymarch.philologus-gtk-rs"),
        Default::default(),
    );

    application.connect_startup(build_ui);

    application.run();
}

#[derive(Debug)]
#[repr(i32)]
enum Columns {
    Id = 0,
    Word,
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("philolog.us");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(580, 250);

    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    let web_view = WebView::new();
    web_view.load_uri("https://philolog.us");

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 8);
    hbox.pack_start(&vbox, false, false, 8);
    hbox.pack_start(&web_view, true, true, 8);
    window.add(&hbox);

    let label = gtk::Entry::new();
    vbox.add(&label);

    let sw = gtk::ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
    sw.set_shadow_type(gtk::ShadowType::EtchedIn);
    sw.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);
    vbox.add(&sw);

    let model = Rc::new(create_model());
    let treeview = gtk::TreeView::with_model(&*model);
    treeview.set_vexpand(true);
    treeview.set_headers_visible(false);
    //treeview.set_search_column(Columns::Description as i32);

    sw.add(&treeview);

    add_columns(&model, &treeview);

    window.show_all();
}

struct Data {
    id: u32,
    word: String,
}

fn create_model() -> gtk::ListStore {
    let col_types: [glib::Type; 2] = [
        glib::Type::U32,
        glib::Type::STRING,
    ];

    let data: [Data; 3] = [
        Data {
            id: 1,
            word: "test1".to_string(),
        },
        Data {
            id: 2,
            word: "test2".to_string(),
        },
        Data {
            id: 3,
            word: "test3".to_string(),
        },

    ];

    let store = gtk::ListStore::new(&col_types);

    for d in data.iter() {


        let values: [(u32, &dyn ToValue); 2] = [
            (0, &d.id),
            (1, &d.word),
        ];
        store.set(&store.append(), &values);
    }

    store
}

fn add_columns(_model: &Rc<gtk::ListStore>, treeview: &gtk::TreeView) {  
    let renderer = gtk::CellRendererText::new();
    let column = gtk::TreeViewColumn::new();
    column.pack_start(&renderer, true);
    //column.set_title("Word");
    column.add_attribute(&renderer, "text", Columns::Word as i32);
    column.set_sort_column_id(Columns::Word as i32);
    treeview.append_column(&column);
}

