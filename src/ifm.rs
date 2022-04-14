use std::net::UdpSocket;
use crate::ifm_data::*;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Reference)]
pub struct Ifacialmocap{
    server: UdpSocket,
    data_map: Dictionary, // -> IfacialmocapData
}

#[methods]
impl Ifacialmocap {
    fn new(_owner: &Reference) -> Self {
        Ifacialmocap {
            server: UdpSocket::bind("0.0.0.0:49983").unwrap(),
            // Shared dictionary
            data_map: Dictionary::new_shared(),
        }
    }

    #[export]
    fn _ready(&self, _owner: &Reference) {
        godot_print!("hello, world.");
    }
    #[export]
    fn _start_tracker(&self, _owner: &Reference) {
        todo!();
    }
    #[export]
    fn _stop_tracker(&self, _owner: &Reference) {
        todo!();
    }
    #[export]
    fn _recieve(&self, _owner: &Reference) {
        todo!();
    }
    #[export]
    fn _perform_reception(&self, _owner: &Reference) {
        todo!();
    }



    // Public Functions
    #[export]
    pub fn is_listening(&self, _owner: &Reference) -> bool {
        todo!();
    }
    #[export]
    pub fn start_reciever(&self, _owner: &Reference) {
        todo!();
    }
    #[export]
    pub fn stop_reciever(&self, _owner: &Reference) {
        todo!();
    }
    #[export]
    pub fn get_data(&self, _owner: &Reference) {
        todo!();
    }
}
