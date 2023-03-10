use std::sync::{Arc, Mutex};

use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Grid, Button, Label};
use crate::communicator::Communicator;

pub struct ElpriceMainWindow;

pub trait MainWindow {
    fn init(app: &Application);
}

const IP_ADDRESS: (u8, u8, u8, u8) = (192, 168, 87, 139);
const SOCKET: u16 = 35000;

impl MainWindow for ElpriceMainWindow {

    fn init(app: &Application) {

        let server_communicator = Arc::new(Mutex::new(Communicator::new(IP_ADDRESS, SOCKET)));
       
        let get_current_price_button = Button::builder()
                                            .width_request(40)
                                            .height_request(20)
                                            .label("Get Current Price")
                                            .build();

        let current_price_label = Label::builder()
                                        .label("Unknown")
                                        .build();



        let grid = Grid::new();
        
        get_current_price_button.connect_clicked(glib::clone!(@strong server_communicator, @weak current_price_label => move |_| {
            match server_communicator.lock() {
                Ok(s) => {
                    if let Ok(price) = s.get_current_price() {
                        current_price_label.set_text(&format!("{}", price));
                    }
                },
                Err(e) => {

                }
            }
        }));

        grid.attach(&get_current_price_button, 0, 0, 40, 20);
        grid.attach(&current_price_label, 30, 30, 10, 10);

        let main_app_window = ApplicationWindow::builder()
                                            .name("Main Window")
                                            .title("Elprice Viewer")
                                            .application(app)
                                            .height_request(480)
                                            .width_request(640)
                                            .child(&grid)
                                            .build();

        main_app_window.present();
    }
}

