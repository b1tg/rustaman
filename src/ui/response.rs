use gtk::prelude::*;
use gtk::{self, Orientation};
use relm::{Relm, Update, Widget};
use sourceview::{self, prelude::*, LanguageManager, StyleSchemeManager, View as SourceView};

use super::super::helpers::path;

#[derive(Msg)]
pub enum Msg {
    RequestExecuted(String),
}

pub struct Response {
    hbox: gtk::Box,
    response_view: SourceView,
    //relm: Relm<Response>,
}

impl Update for Response {
    type Model = ();
    type ModelParam = ();
    type Msg = Msg;

    fn model(_: &Relm<Self>, _: ()) -> () {
        ()
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::RequestExecuted(response) => {
                let buffer = self.response_view.get_buffer().unwrap();
                buffer.set_text(response.as_str());
            }
        }
    }
}

impl Widget for Response {
    type Root = gtk::Box;

    fn root(&self) -> Self::Root {
        self.hbox.clone()
    }

    fn view(_relm: &Relm<Self>, _model: ()) -> Self {
        info!("Creating Response widget");
        let hbox = gtk::Box::new(Orientation::Horizontal, 0);
        hbox.set_hexpand(true);
        hbox.set_vexpand(true);

        let langmngr = LanguageManager::get_default().unwrap();
        let mut search_path = langmngr.get_search_path();
        search_path.push(path::config_dir().unwrap().to_str().unwrap().to_owned());
        let path2: Vec<&str> = search_path.iter().map(|path| path.as_str()).collect();
        langmngr.set_search_path(path2.as_slice());
        let lang = langmngr.get_language("rustaman-response").unwrap();

        let stylemngr = StyleSchemeManager::get_default().unwrap();
        let style = stylemngr.get_scheme("solarized-dark").unwrap();

        let buffer = sourceview::Buffer::new_with_language(&lang);
        buffer.set_style_scheme(&style);

        let response_view = SourceView::new_with_buffer(&buffer);

        response_view.set_editable(false);
        response_view.set_margin_left(10);
        response_view.set_hexpand(true);
        response_view.set_vexpand(true);

        hbox.add(&response_view);

        hbox.set_vexpand(true);
        hbox.set_hexpand(false);
        hbox.set_size_request(800, 0);
        hbox.show_all();
        Response {
            hbox: hbox,
            response_view: response_view,
            //relm: relm.clone(),
        }
    }
}
