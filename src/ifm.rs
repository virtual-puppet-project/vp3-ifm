//! IFacialMocap tracker interface.
//! This is called upon by VP3 Puppeteer.

use crate::ifm_data::*;
use gdnative::prelude::*;
use std::net::UdpSocket;

#[derive(NativeClass)]
#[inherit(Node)]
#[no_constructor] // Just kidding, We're just hiding the constructor from Godot so we can use it as a Singleton.
pub struct Ifacialmocap {
    /// The iFacialMocap object is the main entrypoint for the iFacialMocap/Facemotion3D module.
    /// This will listen to incoming UDP packets from the iFacialMocap/Facemotion3D client on your iOS device.
    /// To use this module, you need to import this module to your Godot project.
    server: Option<UdpSocket>,
    data_map: Dictionary, // -> IfacialmocapData
}

#[methods]
impl Ifacialmocap {
    fn new() -> Self {
        Ifacialmocap {
            server: None,
            // Shared dictionary
            data_map: Dictionary::new_shared(),
        }
    }

    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("hello, world.");
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
            println!("{:?}", self.data_map);
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
    #[test]
    fn listen_test() {
        let mut ifm = Ifacialmocap::new();
        ifm._start_tracker();
    }
}
