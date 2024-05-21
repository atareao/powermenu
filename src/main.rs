mod utils;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::CssProvider;
use gdk::Display;

use utils::set_full_screen;

const APP_ID: &str = "es.atareao.Powermenu";


// https://github.com/wmww/gtk-layer-shell/blob/master/examples/simple-example.c
fn build_ui(application: &gtk::Application) {


    let display = gdk::Display::default().unwrap();
    let monitors = display.monitors();
    println!("{:?}", monitors);
    for monitor in monitors.iter::<gdk::Monitor>(){
        println!("{:?}", monitor.unwrap().description());
    }
    let mut selected_monitors = monitors.iter::<gdk::Monitor>()
        .filter(|monitor| monitor.as_ref().unwrap().connector().unwrap() == "HDMI-A-1");
    let first = selected_monitors.next().unwrap().unwrap();

    println!("{:?}", first);


    // Create a normal GTK window however you like
    let window = gtk::ApplicationWindow::new(application);


    let geometry = &first.geometry();

    let margin_h = (geometry.width() - 300)/2;
    let margin_v = (geometry.height() - 300)/2;

    // Before the window is first realized, set it up to be a layer surface
    //window.init_layer_shell();

    // Set up a widget
    set_full_screen(&window, -1, true);
    let content_box = build_powermenu();
    window.set_child(Some(&content_box));
    window.present()
}

fn load_css() {
     // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_string(include_str!("style.css"));

    // Add the provider to the default screen
    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
fn build_powermenu() -> gtk::Box {
    let content_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(10)
        .homogeneous(true)
        .baseline_position(gtk::BaselinePosition::Center)
        .build();
    let lock_button = gtk::Button::builder()
        .name("lock_button")
        .build();
    content_box.append(&lock_button);
    let logout_button = gtk::Button::builder()
        .name("logout_button")
        .build();
    content_box.append(&logout_button);
    let suspend_button = gtk::Button::builder()
        .icon_name("suspend")
        .build();
    content_box.append(&suspend_button);
    let hibernate_button = gtk::Button::builder()
        .icon_name("hibernate")
        .build();
    content_box.append(&hibernate_button);
    let shutdown_button = gtk::Button::builder()
        .icon_name("shutdown")
        .build();
    shutdown_button.connect_clicked(move |_| { println!("Hola"); });
    content_box.append(&shutdown_button);
    let reboot_button = gtk::Button::builder()
        .icon_name("reboot")
        .build();
    content_box.append(&reboot_button);
    content_box
}

fn main() {
    //let application = gtk::Application::new(Some("sh.wmww.gtk-layer-example"), Default::default());
    let app = gtk::Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);
    app.run();
}
