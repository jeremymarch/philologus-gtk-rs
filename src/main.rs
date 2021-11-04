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

use serde::Deserialize;
use reqwest::Error;

extern crate webkit2gtk;
use webkit2gtk::{traits::WebViewExt, WebView};

#[derive(Debug, Deserialize,Clone)]
pub struct GreekWords { 
    pub i: i32, 
    pub r: (String,u32,u32)
}

#[derive(Debug, Deserialize, Clone)]
struct JsonResponse {
    error: String,
    wtprefix: String,
    nocache: String,
    container: String,
    #[serde(rename(deserialize = "requestTime"))]
    request_time: String,
    #[serde(rename(deserialize = "selectId"))]
    select_id: String,
    page: String,
    #[serde(rename(deserialize = "lastPage"))]
    last_age: String,
    #[serde(rename(deserialize = "lastpageUp"))]
    last_page_up: Option<String>,
    scroll: String,
    query: String,
    #[serde(rename(deserialize = "arrOptions"))]
    arr_options: Vec<GreekWords>
}

#[derive(Debug)]
#[repr(i32)]
enum Columns {
    Id = 0,
    Word,
}

fn main() -> Result<(), Error> {
    let application = gtk::Application::new(
        Some("com.github.jeremymarch.philologus-gtk-rs"),
        Default::default(),
    );

    application.connect_activate(build_ui);

    application.run();
    Ok(())
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("philolog.us");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(600, 300);

    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    let web_view = WebView::new();
    web_view.load_uri("https://philolog.us");

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 8);
    hbox.pack_start(&vbox, false, false, 8);
    hbox.pack_start(&web_view, true, true, 8);
    window.add(&hbox);

    let entry = gtk::Entry::new();
    vbox.add(&entry);

    let sw = gtk::ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
    sw.set_shadow_type(gtk::ShadowType::EtchedIn);
    sw.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);
    vbox.add(&sw);

    let model = Rc::new(create_model());
    get_words(&*model, "");

    let treeview = gtk::TreeView::with_model(&*model);
    treeview.set_vexpand(true);
    treeview.set_headers_visible(false);

    sw.add(&treeview);

    add_columns(&model, &treeview);

    entry.connect_changed(move | entry: &gtk::Entry | {
        let search = entry.text();

        get_words(&*model, &search);
    });

    window.show_all();
}

fn get_words(store:&gtk::ListStore, s:&str) {
    
    store.clear();
    let url = format!("https://philolog.us/wtgreekserv.php?n=101&idprefix=test1&x=0.045663999508706477&requestTime=1635983991202&page=0&mode=context&query={{\"regex\":\"0\",\"lexicon\":\"lsj\",\"tag_id\":\"0\",\"root_id\":\"0\",\"w\":\"{}\"}}", s);
    //println!("{}", request_url2);
    if let Ok(response) = reqwest::blocking::get(&url) {
        //println!("{:?}", response);

        if let Ok(words) = response.json::<JsonResponse>() {
            //println!("{:?}", words);

            for w in words.arr_options {

                let values: [(u32, &dyn ToValue); 2] = [
                    (0, &w.i),
                    (1, &w.r.0),
                ];
                store.set(&store.append(), &values);
            }
        }
    }
}

fn create_model() -> gtk::ListStore {
    let col_types: [glib::Type; 2] = [
        glib::Type::U32,
        glib::Type::STRING,
    ];

    gtk::ListStore::new(&col_types)
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

