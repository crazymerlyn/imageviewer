extern crate gtk;
extern crate gdk_pixbuf;
extern crate gdk;

use std::env;
use std::cell::RefCell as Rc;

use gtk::prelude::*;

use gdk::RGBA;
use gdk::enums::modifier_type;
use gdk::enums::key;

use gtk::{ApplicationWindow, ScrolledWindow, Builder};
use gtk::STATE_FLAG_NORMAL;

mod buf_manager;
use buf_manager::BufManager;

fn main() {
    if gtk::init().is_err()  {
        println!("Failed to initialize GTK.");
        return;
    }
    let glade_src = include_str!("../im.glade");
    let builder = Builder::new();
    builder.add_from_string(glade_src).unwrap();

    let file = env::args().nth(1).unwrap_or("test.jpg".to_string());
    let buf_manager = Rc::new(BufManager::new_from_file(&file));
    
    let image: gtk::Image = builder.get_object("image").unwrap();
    image.set_from_pixbuf(buf_manager.borrow().get_buf());

    let window: ApplicationWindow = builder.get_object("window").unwrap();
    let black = RGBA {red: 0.0, green:0.0, blue:0.0, alpha: 1.0};
    window.override_background_color(STATE_FLAG_NORMAL, &black);

    let scroll_window: ScrolledWindow = builder.get_object("scrolledwindow").unwrap();

    scroll_window.connect_key_press_event(move |s, k| {
        let val = k.as_ref().keyval;
        let state = k.as_ref().state;
        match val {
            key::q => gtk::main_quit(),
            key::Right => {
                if !state.intersects(modifier_type::ControlMask) {
                    buf_manager.borrow_mut().go_right();
                    image.set_from_pixbuf(buf_manager.borrow().get_buf());
                }
            },
            key::Left => {
                if !state.intersects(modifier_type::ControlMask) {
                    buf_manager.borrow_mut().go_left();
                    image.set_from_pixbuf(buf_manager.borrow().get_buf());
                }
            },
            key::Up => {
                println!("{:?}", s.get_vadjustment());
                if !state.intersects(modifier_type::ControlMask) {
                    buf_manager.borrow_mut().zoom_in();
                    image.set_from_pixbuf(buf_manager.borrow().get_buf());
                }
            },
            key::Down => {
                if !state.intersects(modifier_type::ControlMask) {
                    buf_manager.borrow_mut().zoom_out();
                    image.set_from_pixbuf(buf_manager.borrow().get_buf());
                }
            },
            _ => println!("other")
        }
        Inhibit(false)
    });

    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}
