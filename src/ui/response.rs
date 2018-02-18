use gtk::{self, Orientation};
use gtk::prelude::*;
use sourceview::View as SourceView;
use relm::{Relm, Update, Widget};

#[derive(Msg)]
pub enum Msg {
}

pub struct Response {
    hbox: gtk::Box,
    //response_view: SourceView,
    //relm: Relm<Response>,
}

impl Update for Response {
    type Model = ();
    type ModelParam = ();
    type Msg = Msg;

    fn model(_: &Relm<Self>, _: ()) -> () {
        ()
    }

    fn update(&mut self, _event: Msg) {}
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
        let response_view = SourceView::new();
        response_view.set_editable(false);
        response_view.set_margin_left(10);
        response_view.set_hexpand(true);
        response_view.set_vexpand(true);

        hbox.add(&response_view);

        hbox.show_all();
        Response {
            hbox: hbox,
            //response_view: response_view,
            //relm: relm.clone(),
        }
    }
}