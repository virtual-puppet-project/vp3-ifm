use std::net::UdpSocket;
use crate::ifm_data::*;
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Reference)]
pub struct Ifacialmocap{
    server: Option<UdpSocket>,
    data_map: Dictionary, // -> IfacialmocapData
}

#[methods]
impl Ifacialmocap {
    fn new(_owner: &Reference) -> Self {
        Ifacialmocap {
            server: None,
            // Shared dictionary
            data_map: Dictionary::new_shared(),
        }
    }

    #[export]
    fn _ready(&self, _owner: &Reference) {
        godot_print!("hello, world.");
    }
    #[export]
    fn _start_tracker(&mut self, _owner: &Reference) {
        self.server = Some(UdpSocket::bind("0.0.0.0:49983").unwrap());
        loop{
            let data_dict = Dictionary::new();
            let mut buf = [0; 1024];
            let _packet = self.server.as_ref().unwrap().recv_from(&mut buf).unwrap();
            let data = &String::from_utf8((&buf).to_vec()).unwrap() as &str;
            let ifm_data = IfacialmocapData::from_str(data).as_json();

            // We use the very hacky way by using a JSON
            // Then we convert this JSON into a Godot Dictionary that we need. Fun!
            for (key, value) in ifm_data.as_object().unwrap().iter() {
                println!("{}, {}", key, value);
                match key.as_str() {
                    // if head
                    "head" => {
                        // Vector3Array[]
                        let head_data = value.as_array().unwrap().iter().map(|x| x.as_f64().unwrap() as f32).collect::<Vec<f32>>();
                        // Turn the value into f32
                        // Head rotation is the first 3 values in the Vec
                        let head_rotation = Vector3::new(head_data[0], head_data[1], head_data[2]);
                        // Head position is the last 3 values in the Vec
                        let head_position = Vector3::new(head_data[3], head_data[4], head_data[5]);

                        let mut vecarray = Vector3Array::new();
                        // &PoolArray<gdnative::prelude::Vector3>
                        vecarray.push(head_rotation);
                        vecarray.push(head_position);
                        data_dict.insert(key, &vecarray);
                    }

                    "rightEye" | "leftEye" =>{
                        // Vector3
                        let eye_data = value.as_array().unwrap().iter().map(|x| x.as_f64().unwrap() as f32).collect::<Vec<f32>>();
                        let eye_position = Vector3::new(eye_data[0], eye_data[1], eye_data[2]);
                        data_dict.insert(key, &eye_position);
                    }


                    _ => {
                        // float
                        let blendshape = value.as_f64().unwrap() as f32;
                        data_dict.insert(key, &blendshape);
                    }

                }
            }
            // Set the data_map to the data_dict
            let dm = data_dict.into_shared();
            self.data_map = dm;
        }
    }
    #[export]
    fn _stop_tracker(&mut self, _owner: &Reference) {
        self.server.take();
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
        // Get the server
        if let Some(_server) = &self.server {
            // Check if the server is listening
            true
        } else {
            false
        }
    }
    #[export]
    pub fn start_reciever(&mut self, _owner: &Reference) {
        self._start_tracker(_owner);
    }
    #[export]
    pub fn stop_reciever(&mut self, _owner: &Reference) {
        self._stop_tracker(_owner);
    }
    #[export]
    pub fn get_data(&mut self, _owner: &Reference) -> Dictionary {
        // Return the data_map

        self.data_map.new_ref()
    }
}

#[cfg(test)]
mod tests {
    use crate::{ifm::*, ifm_data};
    #[test]
    fn listen_test(){
        let server = UdpSocket::bind("0.0.0.0:49983").unwrap();
        let mut buf = [0; 1024];
        loop{
            let _packet = server.recv_from(&mut buf).unwrap();
            let data = &String::from_utf8((&buf).to_vec()).unwrap();
            println!("{}", data);
            let ifm_data = IfacialmocapData::from_str(data);
            println!("{:?}", ifm_data);
        }
    }
}