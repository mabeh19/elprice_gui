use gtk::prelude::*;
use gtk::Application;


pub mod main_window;
pub mod communicator;

use main_window::*;

fn main() {

    let app = Application::builder()
                        .application_id("org.elprice.gui")
                        .build();


    app.connect_activate(main_window::ElpriceMainWindow::init);

    app.run();        
}
