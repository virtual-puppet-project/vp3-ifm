//! IFacialMocap tracker interface.
//! This is called upon by VP3 Puppeteer.

use crate::ifm_data::*;
use gdnative::prelude::*;
use std::{net::UdpSocket, thread};

const RUN_FACE_TRACKER_TEXT: &str = "run_face_tracker";
const STOP_FACE_TRACKER_TEXT: &str = "stop_face_tracker";
#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register_signals)]
//#[no_constructor] // Just kidding, We're just hiding the constructor from Godot so we can use it as a Singleton.
pub struct Ifacialmocap {
    /// The iFacialMocap object is the main entrypoint for the iFacialMocap/Facemotion3D module.
    /// This will listen to incoming UDP packets from the iFacialMocap/Facemotion3D client on your iOS device.
    /// To use this module, you need to import this module to your Godot project.
    server: Option<UdpSocket>,
    data_map: Dictionary, // -> IfacialmocapData

    // TODO: Replace this with Godot's UDPServer class, which is another level of hell.
}

#[methods]
impl Ifacialmocap {
    fn new(_owner: &Node) -> Self {
        Ifacialmocap {
            server: None,
            // Shared dictionary
            data_map: Dictionary::new_shared(),
        }
    }

    #[export]
    fn _ready(&mut self, _owner: &Node) {
        godot_print!("hello, world.");
        // Start tracker in a separate thread
        // The tracker requires a mutable self
        let mut tracker = Ifacialmocap::new(_owner);
        let handle = thread::spawn(move || {
            tracker._start_tracker();
        });
        handle.join().unwrap();
    }
    fn register_signals(builder: &ClassBuilder<Self>) {
        builder.signal(&RUN_FACE_TRACKER_TEXT)
            .done();
        builder.signal(&STOP_FACE_TRACKER_TEXT)
            .done();
    }

    fn _start_tracker(&mut self) {
        // TODO: Why does this cause a Buffer Overrun?
        self.server = Some(UdpSocket::bind("0.0.0.0:49983").unwrap());
        loop {
            let mut buf = [0; 2048];
            let (size, _origin) = self.server.as_ref().unwrap().recv_from(&mut buf).unwrap();
            let _packet = self.server.as_ref().unwrap().recv_from(&mut buf).unwrap();
            self.data_map =
                IfacialmocapData::from_str(std::str::from_utf8(&buf[..size]).unwrap())
                .as_dict();
            godot_dbg!("{:#?}", &self.data_map);
        }
    }
    fn _stop_tracker(&mut self) {
        self.server.take();
    }
    #[export]
    fn _recieve(&self, _owner: &Node) {
        todo!();
    }
    #[export]
    fn _perform_reception(&self, _owner: &Node) {
        todo!();
    }

    // Public Functions
    #[export]
    pub fn is_listening(&self, _owner: &Node) -> bool {
        // Get the server
        if let Some(_server) = &self.server {
            // Check if the server is listening
            true
        } else {
            false
        }
    }
    #[export]
    pub fn start_reciever(&mut self, _owner: &Node) {
        self._start_tracker();
    }
    #[export]
    pub fn stop_reciever(&mut self, _owner: &Node) {
        self._stop_tracker();
    }
    #[export]
    pub fn get_data(&mut self, _owner: &Node) -> Dictionary {
        // Return the data_map

        self.data_map.new_ref()
    }
}

#[cfg(test)]
mod tests {
    use crate::{ifm::*, ifm_data};
}
