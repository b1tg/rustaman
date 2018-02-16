use gdk;
use gdk::enums::key;
use gtk::{self, Orientation};
use gtk::prelude::*;

use relm::{Relm, Update, Widget};

pub struct Model {
    id: usize,
}

#[derive(Msg)]
pub enum Msg {
    ToggleRequest(usize, bool),
    RequestNameChanged(usize, String),
    SetActive(bool),
    EntryKeyPress(gdk::EventKey),
    RenameRequest,
}

pub struct MenuItem {
    hbox: gtk::Box,
    displaybox: gtk::Box,
    toggle_btn: gtk::ToggleButton,
    entry: gtk::Entry,
    relm: Relm<MenuItem>,
    model: Model,
}

impl Update for MenuItem {
    type Model = Model;
    type ModelParam = usize;
    type Msg = Msg;

    fn model(_: &Relm<Self>, request_id: usize) -> Model {
        Model { id: request_id }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::RenameRequest => {
                self.displaybox.hide();
                self.entry.show();
                self.entry.grab_focus();
            }

            Msg::SetActive(active) => {
                if self.entry.is_visible() {
                    self.entry.grab_focus();
                } else {
                    self.toggle_btn.set_active(active);
                }
            }

            Msg::EntryKeyPress(key) => {
                let keyval = key.get_keyval();
                match keyval {
                    key::Return => {
                        let text = self.entry.get_text();
                        if text.is_some() {
                            let name = text.unwrap();
                            self.toggle_btn.set_label(name.as_str());
                            self.entry.hide();
                            self.displaybox.show();
                            self.relm
                                .stream()
                                .emit(Msg::RequestNameChanged(self.model.id, name.to_owned()))
                        }
                    }
                    _ => {}
                }
            }

            _ => {}
        }
    }
}

impl Widget for MenuItem {
    type Root = gtk::Box;

    fn root(&self) -> Self::Root {
        self.hbox.clone()
    }

    fn view(relm: &Relm<Self>, model: Model) -> Self {
        let hbox = gtk::Box::new(Orientation::Horizontal, 0);
        hbox.set_hexpand(true);

        let name = format!("Req #{}", &model.id);
        let entry = gtk::Entry::new();
        entry.set_text(name.as_str());
        entry.set_can_focus(true);
        entry.select_region(0, name.len() as i32);
        connect!(
            relm,
            entry,
            connect_key_press_event(_, key),
            return (Msg::EntryKeyPress(key.clone()), Inhibit(false))
        );
        entry.set_hexpand(true);
        entry.show();
        hbox.add(&entry);

        let displaybox = gtk::Box::new(Orientation::Horizontal, 0);
        displaybox.set_hexpand(true);

        let toggle_btn = gtk::ToggleButton::new_with_label(name.as_str());
        toggle_btn.set_hexpand(true);
        toggle_btn.set_focus_on_click(false);
        toggle_btn.set_relief(gtk::ReliefStyle::Half);
        toggle_btn.show();
        displaybox.add(&toggle_btn);

        let menu = gtk::Menu::new();
        let rename = gtk::MenuItem::new_with_label("Rename");
        menu.append(&rename);
        let delete = gtk::MenuItem::new_with_label("Delete");
        menu.append(&delete);
        rename.show();
        delete.show();
        let combo_btn = gtk::MenuButton::new();
        combo_btn.set_popup(&menu);
        combo_btn.set_use_popover(true);
        combo_btn.show();
        displaybox.add(&combo_btn);
        hbox.add(&displaybox);

        connect!(relm, rename, connect_activate(_), Msg::RenameRequest);

        let model_id = model.id;
        connect!(
            relm,
            toggle_btn,
            connect_clicked(btn),
            Msg::ToggleRequest(model_id, btn.get_active())
        );

        hbox.show();
        MenuItem {
            hbox: hbox,
            displaybox: displaybox,
            toggle_btn: toggle_btn,
            entry: entry,
            relm: relm.clone(),
            model: model,
        }
    }
}