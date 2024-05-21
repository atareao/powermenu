use gtk::prelude::*;
use gtk4_layer_shell::{Edge, Layer, LayerShell, KeyboardMode};

const EXCLUSIVE_ZONE: i32 = -1;

pub fn set_full_screen(window: &gtk::ApplicationWindow, monitor: i64, keyboard: bool) {
    let display = gdk::Display::default().unwrap();
    let surface = window.surface().unwrap();
    let active_monitor = display.monitor_at_surface(&surface).unwrap();

    window.init_layer_shell();
    window.set_layer(Layer::Overlay);
    window.set_namespace("logout_dialog");
    window.set_exclusive_zone(EXCLUSIVE_ZONE);

    for anchor in [Edge::Left, Edge::Right, Edge::Top, Edge::Bottom] {
        window.set_anchor(anchor, true);
    }
    
    window.set_monitor(&active_monitor);
    if keyboard {
        window.set_keyboard_mode(KeyboardMode::Exclusive);
    }
}

pub fn get_monitor(window: &gtk::Window, key_event: &gdk::KeyEvent) {
    let display = gdk::Display::default().unwrap();
    let surface = window.surface().unwrap();
    let active_monitor = display.monitor_at_surface(&surface).unwrap();
    let monitors = display.monitors();
    let mut windows = Vec::new();
    println!("{:?}", monitors);
    for monitor in monitors.iter::<gdk::Monitor>(){
        println!("{:?}", monitor.unwrap().description());
        window_box.connect_bu

        let window = gtk::Window::new();
        windows.push(window);

    }

}
