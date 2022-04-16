//! The data format for iFacialMocap/Facemotion3D tracking data.
//!
//! The [official](https://www.ifacialmocap.com/for-developer/) [docs](https://www.facemotion3d.info/) does a bad job explaining the data format itself, so there will be an explanation below.
//!
//! ## Data Format
//!
//! The data format is encoded in plain text, and is a list of key-value stores seperated by a pipe (|) character.
//!
//! Data types are then defined by another seperator, which can be `-` for a floating point blendshape value (or `&` for Facemotion3D data) that goes from 0-100, or `#` for a Vector3 value.
//!
//! The data is then fed into a compatible application, which then maps it to your model.
//!
//! ### Format Example (Pretty-printed)
//! ```ifm
//! key-0|
//! vec3#0.0,0.0,0.0|
//! // There's a special case for the head, which is a Vector3Array, defined by the `=` in front of the key
//! =vecArray#0.1,0.2,0.3,0.4,0.5,0.6|
//! // The keys will be cut in half, The first half is the pitch, roll, and yaw, and the second half is the x, y, and z position of the head.
//! ```
//! The following data will then be mapped to the model's blendshapes, and the head will be rotated and moved to the specified position.
//!
//! ### Actual Data Example
//! ```ifm
//! mouthSmile_R-0|
//! eyeLookOut_L-0|
//! mouthUpperUp_L-11|
//! eyeWide_R-0|mouthClose-8|
//! mouthPucker-4|
//! mouthRollLower-9|
//! eyeBlink_R-7|eyeLookDown_L-17|
//! cheekSquint_R-11|
//! eyeBlink_L-7|
//! tongueOut-0|
//! jawRight-0|
//! eyeLookIn_R-6|
//! cheekSquint_L-11|
//! mouthDimple_L-10|
//! mouthPress_L-4|
//! eyeSquint_L-11|
//! mouthRight-0|
//! mouthShrugLower-9|
//! eyeLookUp_R-0|
//! eyeLookOut_R-0|
//! mouthPress_R-5|
//! cheekPuff-2|
//! jawForward-11|
//! mouthLowerDown_L-9|
//! mouthFrown_L-6|
//! mouthShrugUpper-26|
//! browOuterUp_L-4|
//! browInnerUp-20|
//! mouthDimple_R-10|
//! browDown_R-0|
//! mouthUpperUp_R-10|
//! mouthRollUpper-8|
//! mouthFunnel-12|
//! mouthStretch_R-21|
//! mouthFrown_R-13|
//! eyeLookDown_R-17|
//! jawOpen-12|
//! jawLeft-0|
//! browDown_L-0|
//! mouthSmile_L-0|
//! noseSneer_R-18|
//! mouthLowerDown_R-8|
//! noseSneer_L-21|
//! eyeWide_L-0|
//! mouthStretch_L-21|
//! browOuterUp_R-4|
//! eyeLookIn_L-4|
//! eyeSquint_R-11|
//! eyeLookUp_L-0|
//! mouthLeft-1|
//! =head#-21.488958,-6.038993,-6.6019735,-0.030653415,-0.10287084,-0.6584072|
//! rightEye#6.0297494,2.4403017,0.25649446|
//! leftEye#6.034903,-1.6660284,-0.17520553|
//! ```
//! These blendshapes are also [Perfect Sync](https://malaybaku.github.io/VMagicMirror/en/tips/perfect_sync) blendshapes, so they can be applied directly to a Perfect Sync model.

use gdnative::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Don't warn for non-snakecase variables
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, NativeClass, PartialEq, Clone)]
#[no_constructor]
#[inherit(Reference)]
pub struct IfacialmocapData {
    /// iFaceMocap data object.
    /// Contains the list of Perfect Sync blendshapes outputted by iFacialMocap.
    mouthSmile_R: f32,
    eyeLookOut_L: f32,
    mouthUpperUp_L: f32,
    eyeWide_R: f32,
    mouthClose: f32,
    mouthPucker: f32,
    mouthRollLower: f32,
    eyeBlink_R: f32,
    eyeLookDown_L: f32,
    cheekSquint_R: f32,
    eyeBlink_L: f32,
    tongueOut: f32,
    jawRight: f32,
    eyeLookIn_R: f32,
    cheekSquint_L: f32,
    mouthDimple_L: f32,
    mouthPress_L: f32,
    eyeSquint_L: f32,
    mouthRight: f32,
    mouthShrugLower: f32,
    eyeLookUp_R: f32,
    eyeLookOut_R: f32,
    mouthPress_R: f32,
    cheekPuff: f32,
    jawForward: f32,
    mouthLowerDown_L: f32,
    mouthFrown_L: f32,
    mouthShrugUpper: f32,
    browOuterUp_L: f32,
    browInnerUp: f32,
    mouthDimple_R: f32,
    browDown_R: f32,
    mouthUpperUp_R: f32,
    mouthRollUpper: f32,
    mouthFunnel: f32,
    mouthStretch_R: f32,
    mouthFrown_R: f32,
    eyeLookDown_R: f32,
    jawOpen: f32,
    jawLeft: f32,
    browDown_L: f32,
    mouthSmile_L: f32,
    noseSneer_R: f32,
    mouthLowerDown_R: f32,
    noseSneer_L: f32,
    eyeWide_L: f32,
    mouthStretch_L: f32,
    browOuterUp_R: f32,
    eyeLookIn_L: f32,
    eyeSquint_R: f32,
    eyeLookUp_L: f32,
    mouthLeft: f32,

    head: Vec<f32>,

    rightEye: Vec<f32>,
    leftEye: Vec<f32>,
}
#[methods]
impl IfacialmocapData {
    // _owner is optional

    pub fn new() -> Self {
        Self {
            mouthSmile_R: 0.0,
            eyeLookOut_L: 0.0,
            mouthUpperUp_L: 0.0,
            eyeWide_R: 0.0,
            mouthClose: 0.0,
            mouthPucker: 0.0,
            mouthRollLower: 0.0,
            eyeBlink_R: 0.0,
            eyeLookDown_L: 0.0,
            cheekSquint_R: 0.0,
            eyeBlink_L: 0.0,
            tongueOut: 0.0,
            jawRight: 0.0,
            eyeLookIn_R: 0.0,
            cheekSquint_L: 0.0,
            mouthDimple_L: 0.0,
            mouthPress_L: 0.0,
            eyeSquint_L: 0.0,
            mouthRight: 0.0,
            mouthShrugLower: 0.0,
            eyeLookUp_R: 0.0,
            eyeLookOut_R: 0.0,
            mouthPress_R: 0.0,
            cheekPuff: 0.0,
            jawForward: 0.0,
            mouthLowerDown_L: 0.0,
            mouthFrown_L: 0.0,
            mouthShrugUpper: 0.0,
            browOuterUp_L: 0.0,
            browInnerUp: 0.0,
            mouthDimple_R: 0.0,
            browDown_R: 0.0,
            mouthUpperUp_R: 0.0,
            mouthRollUpper: 0.0,
            mouthFunnel: 0.0,
            mouthStretch_R: 0.0,
            mouthFrown_R: 0.0,
            eyeLookDown_R: 0.0,
            jawOpen: 0.0,
            jawLeft: 0.0,
            browDown_L: 0.0,
            mouthSmile_L: 0.0,
            noseSneer_R: 0.0,
            mouthLowerDown_R: 0.0,
            noseSneer_L: 0.0,
            eyeWide_L: 0.0,
            mouthStretch_L: 0.0,
            browOuterUp_R: 0.0,
            eyeLookIn_L: 0.0,
            eyeSquint_R: 0.0,
            eyeLookUp_L: 0.0,
            mouthLeft: 0.0,
            head: vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            rightEye: vec![0.0, 0.0, 0.0],
            leftEye: vec![0.0, 0.0, 0.0],
        }
    }
    // Phew that's long

    // Getter
    pub fn get(self, key: &str) -> f32 {
        match key {
            "mouthSmile_R" => self.mouthSmile_R,
            "eyeLookOut_L" => self.eyeLookOut_L,
            "mouthUpperUp_L" => self.mouthUpperUp_L,
            "eyeWide_R" => self.eyeWide_R,
            "mouthClose" => self.mouthClose,
            "mouthPucker" => self.mouthPucker,
            "mouthRollLower" => self.mouthRollLower,
            "eyeBlink_R" => self.eyeBlink_R,
            "eyeLookDown_L" => self.eyeLookDown_L,
            "cheekSquint_R" => self.cheekSquint_R,
            "eyeBlink_L" => self.eyeBlink_L,
            "tongueOut" => self.tongueOut,
            "jawRight" => self.jawRight,
            "eyeLookIn_R" => self.eyeLookIn_R,
            "cheekSquint_L" => self.cheekSquint_L,
            "mouthDimple_L" => self.mouthDimple_L,
            "mouthPress_L" => self.mouthPress_L,
            "eyeSquint_L" => self.eyeSquint_L,
            "mouthRight" => self.mouthRight,
            "mouthShrugLower" => self.mouthShrugLower,
            "eyeLookUp_R" => self.eyeLookUp_R,
            "eyeLookOut_R" => self.eyeLookOut_R,
            "mouthPress_R" => self.mouthPress_R,
            "cheekPuff" => self.cheekPuff,
            "jawForward" => self.jawForward,
            "mouthLowerDown_L" => self.mouthLowerDown_L,
            "mouthFrown_L" => self.mouthFrown_L,
            "mouthShrugUpper" => self.mouthShrugUpper,
            "browOuterUp_L" => self.browOuterUp_L,
            "browInnerUp" => self.browInnerUp,
            "mouthDimple_R" => self.mouthDimple_R,
            "browDown_R" => self.browDown_R,
            "mouthUpperUp_R" => self.mouthUpperUp_R,
            "mouthRollUpper" => self.mouthRollUpper,
            "mouthFunnel" => self.mouthFunnel,
            "mouthStretch_R" => self.mouthStretch_R,
            "mouthFrown_R" => self.mouthFrown_R,
            "eyeLookDown_R" => self.eyeLookDown_R,
            "jawOpen" => self.jawOpen,
            "jawLeft" => self.jawLeft,
            "browDown_L" => self.browDown_L,
            "mouthSmile_L" => self.mouthSmile_L,
            "noseSneer_R" => self.noseSneer_R,
            "mouthLowerDown_R" => self.mouthLowerDown_R,
            "noseSneer_L" => self.noseSneer_L,
            "eyeWide_L" => self.eyeWide_L,
            "mouthStretch_L" => self.mouthStretch_L,
            "browOuterUp_R" => self.browOuterUp_R,
            "eyeLookIn_L" => self.eyeLookIn_L,
            "eyeSquint_R" => self.eyeSquint_R,
            "eyeLookUp_L" => self.eyeLookUp_L,
            "mouthLeft" => self.mouthLeft,
            _ => panic!("No such key: {}", key),
        }
    }
    pub fn get_vec(self, key: &str) -> Vec<f32> {
        match key {
            "head" => self.head,
            "rightEye" => self.rightEye,
            "leftEye" => self.leftEye,
            _ => panic!("No such key: {}", key),
        }
    }
    // Setter
    pub fn set(&mut self, key: &str, value: f32) {
        match key {
            "mouthSmile_R" => self.mouthSmile_R = value,
            "eyeLookOut_L" => self.eyeLookOut_L = value,
            "mouthUpperUp_L" => self.mouthUpperUp_L = value,
            "eyeWide_R" => self.eyeWide_R = value,
            "mouthClose" => self.mouthClose = value,
            "mouthPucker" => self.mouthPucker = value,
            "mouthRollLower" => self.mouthRollLower = value,
            "eyeBlink_R" => self.eyeBlink_R = value,
            "eyeLookDown_L" => self.eyeLookDown_L = value,
            "cheekSquint_R" => self.cheekSquint_R = value,
            "eyeBlink_L" => self.eyeBlink_L = value,
            "tongueOut" => self.tongueOut = value,
            "jawRight" => self.jawRight = value,
            "eyeLookIn_R" => self.eyeLookIn_R = value,
            "cheekSquint_L" => self.cheekSquint_L = value,
            "mouthDimple_L" => self.mouthDimple_L = value,
            "mouthPress_L" => self.mouthPress_L = value,
            "eyeSquint_L" => self.eyeSquint_L = value,
            "mouthRight" => self.mouthRight = value,
            "mouthShrugLower" => self.mouthShrugLower = value,
            "eyeLookUp_R" => self.eyeLookUp_R = value,
            "eyeLookOut_R" => self.eyeLookOut_R = value,
            "mouthPress_R" => self.mouthPress_R = value,
            "cheekPuff" => self.cheekPuff = value,
            "jawForward" => self.jawForward = value,
            "mouthLowerDown_L" => self.mouthLowerDown_L = value,
            "mouthFrown_L" => self.mouthFrown_L = value,
            "mouthShrugUpper" => self.mouthShrugUpper = value,
            "browOuterUp_L" => self.browOuterUp_L = value,
            "browInnerUp" => self.browInnerUp = value,
            "mouthDimple_R" => self.mouthDimple_R = value,
            "browDown_R" => self.browDown_R = value,
            "mouthUpperUp_R" => self.mouthUpperUp_R = value,
            "mouthRollUpper" => self.mouthRollUpper = value,
            "mouthFunnel" => self.mouthFunnel = value,
            "mouthStretch_R" => self.mouthStretch_R = value,
            "mouthFrown_R" => self.mouthFrown_R = value,
            "eyeLookDown_R" => self.eyeLookDown_R = value,
            "jawOpen" => self.jawOpen = value,
            "jawLeft" => self.jawLeft = value,
            "browDown_L" => self.browDown_L = value,
            "mouthSmile_L" => self.mouthSmile_L = value,
            "noseSneer_R" => self.noseSneer_R = value,
            "mouthLowerDown_R" => self.mouthLowerDown_R = value,
            "noseSneer_L" => self.noseSneer_L = value,
            "eyeWide_L" => self.eyeWide_L = value,
            "mouthStretch_L" => self.mouthStretch_L = value,
            "browOuterUp_R" => self.browOuterUp_R = value,
            "eyeLookIn_L" => self.eyeLookIn_L = value,
            "eyeSquint_R" => self.eyeSquint_R = value,
            "eyeLookUp_L" => self.eyeLookUp_L = value,
            "mouthLeft" => self.mouthLeft = value,
            "hapihapi" => {}
            // Do nothing if no such key
            _ => {}
        };
    }
    pub fn set_vec(&mut self, key: &str, value: Vec<f32>) {
        match key {
            "=head" => self.head = value,
            "rightEye" => self.rightEye = value,
            "leftEye" => self.leftEye = value,
            _ => {}
        };
    }
    pub fn from_str(data: &str) -> IfacialmocapData {
        let mut ifm_d = IfacialmocapData::new();
        // This is where the real fun begins
        // parse iFM's weird key value store format

        // Format:
        /*BlendShape Name-Parameters (0 ~ 100) | BlendShape Names-Parameters (0 ~ 100) | .... | = head # Euler angles X (degree), Euler angles Y, Euler angles Z, Position values X, Position values Y, Position values Z | rightEye #Euler angles X, Euler angles Y, Euler angles Z | leftEye #Euler angles X, Euler angles Y, Euler angles Z | */

        // split everything by |
        data.split("|").for_each(|pair| {
            // If can split by - using if let Some = pair.split_once("-")
            // turn it into key value pair
            if let Some(pair) = pair.split_once("-") {
                if pair.0.contains("#") {
                    // Do nothing
                } else {
                    let key = pair.0;
                    let value = pair.1;
                    ifm_d.set(key, value.parse::<f32>().unwrap());
                }
                // special exception: Check if key contains #
            }
            if let Some(pair) = pair.split_once("&") {
                if pair.0.contains("#") {
                    // Do nothing
                } else {
                    let key = pair.0;
                    let value = pair.1;
                    ifm_d.set(key, value.parse::<f32>().unwrap_or_default());
                }
                // special exception: Check if key contains #
            }
            if let Some(pair) = pair.split_once("#") {
                let key = pair.0;
                let value = pair.1.split(",").collect::<Vec<&str>>();
                // convert to Vec<f32>
                let mut vec = Vec::new();
                for i in value {
                    vec.push(i.parse::<f32>()
                    .unwrap_or_default());
                }
                ifm_d.set_vec(key, vec);
            }
            // If it's invalid, ignore it
            else {
                // Do nothing
            }
        });
        // Return the data
        ifm_d
    }
    #[export]
    pub fn get_euler(&self, _owner: &Reference) -> Vector3 {
        Vector3 {
            x: self.head[0],
            y: self.head[1],
            z: self.head[2],
        }
    }

    #[export]
    pub fn read_from_packet(&self, _owner: &Reference, bytes: ByteArray) -> IfacialmocapData {
        // Convert to str
        let data = String::from_utf8(bytes.to_vec()).unwrap();
        // Parse the data
        IfacialmocapData::from_str(&data)
    }

    pub fn _read_from_packet(bytes: ByteArray) -> IfacialmocapData {
        // Convert to str
        let data = String::from_utf8(bytes.to_vec()).unwrap();
        // Parse the data
        IfacialmocapData::from_str(&data)
    }

    pub fn as_json(&self) -> Value {
        let serialized = serde_json::to_string(&self).unwrap();
        let s: Value = serde_json::from_str(&serialized).unwrap();
        // AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA
        // The whole reason of this existing so I don't have to write my own serializer
        // I'm going insane
        s
    }

    pub fn as_dict(&self) -> Dictionary {
        let data_dict = Dictionary::new();
        let ifm_data = self.as_json();
        for (key, value) in ifm_data.as_object().unwrap().iter() {
            println!("{}, {}", key, value);
            match key.as_str() {
                // if head
                "head" => {
                    // Vector3Array[]
                    let head_data = value
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|x| x.as_f64().unwrap() as f32)
                        .collect::<Vec<f32>>();
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

                "rightEye" | "leftEye" => {
                    // Vector3
                    let eye_data = value
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|x| x.as_f64().unwrap() as f32)
                        .collect::<Vec<f32>>();
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

        data_dict.into_shared()
    }
}
// implment ToVariant
impl ToVariant for IfacialmocapData {
    fn to_variant(&self) -> Variant {
        Variant::new(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::{ifm::Ifacialmocap, ifm_data::IfacialmocapData};
    static test_data: &str = "mouthSmile_R-0|eyeLookOut_L-0|mouthUpperUp_L-11|eyeWide_R-0|mouthClose-8|mouthPucker-4|mouthRollLower-9|eyeBlink_R-7|eyeLookDown_L-17|cheekSquint_R-11|eyeBlink_L-7|tongueOut-0|jawRight-0|eyeLookIn_R-6|cheekSquint_L-11|mouthDimple_L-10|mouthPress_L-4|eyeSquint_L-11|mouthRight-0|mouthShrugLower-9|eyeLookUp_R-0|eyeLookOut_R-0|mouthPress_R-5|cheekPuff-2|jawForward-11|mouthLowerDown_L-9|mouthFrown_L-6|mouthShrugUpper-26|browOuterUp_L-4|browInnerUp-20|mouthDimple_R-10|browDown_R-0|mouthUpperUp_R-10|mouthRollUpper-8|mouthFunnel-12|mouthStretch_R-21|mouthFrown_R-13|eyeLookDown_R-17|jawOpen-12|jawLeft-0|browDown_L-0|mouthSmile_L-0|noseSneer_R-18|mouthLowerDown_R-8|noseSneer_L-21|eyeWide_L-0|mouthStretch_L-21|browOuterUp_R-4|eyeLookIn_L-4|eyeSquint_R-11|eyeLookUp_L-0|mouthLeft-1|=head#-21.488958,-6.038993,-6.6019735,-0.030653415,-0.10287084,-0.6584072|rightEye#6.0297494,2.4403017,0.25649446|leftEye#6.034903,-1.6660284,-0.17520553|";
    static invalid_data: &str = "mouthLeft-0|browInnerUp-6|mouthLowerDown_L-4|mouthDimple_R-2|mouthFunnel-5|eyeSquint_L-12|browOuterUp_L-0|mouthUpperUp_L-4|mouthFrown_R-2|eyeLookOut_R-0|mouthShrugUpper-11|eyeSquint_R-12|eyeLookDown_R-15|mouthRollLower-6|eyeLookDown_L-16|cheekSquint_L-9|mouthSmile_L-0|mouthRight-0|mouthDimple_L-2|jawRight-0|mouthPucker-24|mouthRollUpper-1|mouthPress_L-8|eyeLookOut_L-0|browDown_R-13|cheekSquint_R-8|mouthFrown_L-3|tongueOut-0|mouthPress_R-10|browDown_L-12|mouthLowerDown_R-4|eyeWide_L-2|cheekPuff-7|mouthSmile_R-0|eyeLookIn_L-0|eyeLookUp_L-0|jawForward-3|jawLeft-4|noseSneer_L-13|jawOpen-2|mouthStretch_R-8|eyeLookUp_R-0|mouthClose-4|eyeWide_R-2|eyeBlink_L-2|eyeLookIn_R-12|noseSneer_R-9|eyeBlink_R-2|mouthUpperUp_R-4|browOuterUp_R-0|mouthStretch_L-9|mouthShrugLower-14|hapihapi-0|=head#25.409164,-5.085786,3.8090365,0.052303925,0.2366666,-0.0259732|rightEye#5.2707267,4.227702,0.41178665|leftEye#5.300755,0.32921365,0.03218361|-0.67254096|||||3|";

    static fm_data: &str = "browInnerUp&4|mouthPucker&6|eyeSquintRight&2|tongueOut&0|mouthLeft&0|mouthLowerDownRight&1|mouthDimpleRight&1|browDownRight&0|mouthUpperUpRight&3|mouthRollUpper&0|cheekSquintLeft&3|mouthFunnel0|browOuterUpLeft&0|noseSneerRight&9|mouthLowerDownLeft&1|mouthPucker&6|mouthStretchRight&4|mouthPressRight&7|eyeLookDownRight&21|eyeLookOutLeft&0|tongueOut&0|eyeLookDownLeft&21|jawOpen&0|mouthShrugLower&14|FM_SAD&0|FM_ANGRY&0|FM_VF&0|FM_TH&0|FM_browUpRight&0|FM_browUpLeft&0|hapihapi&0|=head#0.80720806,2.406476,1.3182178,-0.020704115,-0.054985482,-0.23076123|rightEye#0.15436459,0.543746,0.0|leftEye#0.16736937,0.5399485,0.0||0|.0|.0|||";
    static fm_data_v1: &str = "browInnerUp-4|mouthPucker-6|eyeSquint_R-2|tongueOut-0|mouth_L-0|mouthLowerDown_R-1|mouthDimple_R-1|browDown_R-0|mouthUpperUp_R-3|mouthRollUpper-0|cheekSquint_L-3|mouthFunnel0|browOuterUp_L-0|noseSneer_R-9|mouthLowerDown_L-1|mouthPucker-6|mouthStretch_R-4|mouthPress_R-7|eyeLookDown_R-21|eyeLookOut_L-0|tongueOut-0|eyeLookDown_L-21|jawOpen-0|mouthShrugLower-14|FM_SAD-0|FM_ANGRY-0|FM_VF-0|FM_TH-0|FM_browUp_R-0|FM_browUp_L-0|hapihapi-0|=head#0.80720806,2.406476,1.3182178,-0.020704115,-0.054985482,-0.23076123|_REye#0.15436459,0.543746,0.0|_LEye#0.16736937,0.5399485,0.0||0|.0|.0|||";

    #[test]
    fn test_ifm_data() {
        let ifm_data = IfacialmocapData::from_str(test_data);
        println!("{:?}", ifm_data);
    }
    #[test]
    fn test_fm_data() {
        let ifm_data = IfacialmocapData::from_str(fm_data);
        let ser = serde_json::to_string(&ifm_data).unwrap();

        let ifm_data_v1 = IfacialmocapData::from_str(fm_data_v1);
        assert_eq!(ifm_data, ifm_data_v1);
    }
    #[test]
    fn invalid_parse_test() {
        let ifm_data = IfacialmocapData::from_str(invalid_data);
        let ser = serde_json::to_string(&ifm_data).unwrap();
        println!("{}", ser);
    }
}
