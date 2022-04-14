// Don't warn for non-snakecase variables
#![allow(non_snake_case)]
pub struct IfacialmocapData {
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

impl IfacialmocapData {
    pub fn new() -> IfacialmocapData {
        IfacialmocapData {
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
    pub fn get(self, key: &str) {
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
            _ => panic!("No such key"),
        };
    }
    pub fn get_vec(self, key: &str) {
        match key {
            "head" => self.head,
            "rightEye" => self.rightEye,
            "leftEye" => self.leftEye,
            _ => panic!("No such key"),
        };
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
            _ => panic!("No such key"),
        };
    }
    pub fn set_vec(&mut self, key: &str, value: Vec<f32>) {
        match key {
            "=head" => self.head = value,
            "rightEye" => self.rightEye = value,
            "leftEye" => self.leftEye = value,
            _ => panic!("No such key"),
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
            if let Some(pair) = pair.split_once("#") {
                let key = pair.0;
                let value = pair.1.split(",").collect::<Vec<&str>>();
                // convert to Vec<f32>
                let mut vec = Vec::new();
                for i in value {
                    vec.push(i.parse::<f32>().unwrap());
                }
                ifm_d.set_vec(key, vec);
            }
        });
        // Return the data
        ifm_d
    }
}
use std::fmt::Debug;
impl Debug for IfacialmocapData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IfacialmocapData")
            .field("mouthSmile_R", &self.mouthSmile_R)
            .field("eyeLookOut_L", &self.eyeLookOut_L)
            .field("mouthUpperUp_L", &self.mouthUpperUp_L)
            .field("eyeWide_R", &self.eyeWide_R)
            .field("mouthClose", &self.mouthClose)
            .field("mouthPucker", &self.mouthPucker)
            .field("mouthRollLower", &self.mouthRollLower)
            .field("eyeBlink_R", &self.eyeBlink_R)
            .field("eyeLookDown_L", &self.eyeLookDown_L)
            .field("cheekSquint_R", &self.cheekSquint_R)
            .field("eyeBlink_L", &self.eyeBlink_L)
            .field("tongueOut", &self.tongueOut)
            .field("jawRight", &self.jawRight)
            .field("eyeLookIn_R", &self.eyeLookIn_R)
            .field("cheekSquint_L", &self.cheekSquint_L)
            .field("mouthDimple_L", &self.mouthDimple_L)
            .field("mouthPress_L", &self.mouthPress_L)
            .field("eyeSquint_L", &self.eyeSquint_L)
            .field("mouthRight", &self.mouthRight)
            .field("mouthShrugLower", &self.mouthShrugLower)
            .field("eyeLookUp_R", &self.eyeLookUp_R)
            .field("eyeLookOut_R", &self.eyeLookOut_R)
            .field("mouthPress_R", &self.mouthPress_R)
            .field("cheekPuff", &self.cheekPuff)
            .field("jawForward", &self.jawForward)
            .field("mouthLowerDown_L", &self.mouthLowerDown_L)
            .field("mouthFrown_L", &self.mouthFrown_L)
            .field("mouthShrugUpper", &self.mouthShrugUpper)
            .field("browOuterUp_L", &self.browOuterUp_L)
            .field("browInnerUp", &self.browInnerUp)
            .field("mouthDimple_R", &self.mouthDimple_R)
            .field("browDown_R", &self.browDown_R)
            .field("mouthUpperUp_R", &self.mouthUpperUp_R)
            .field("mouthRollUpper", &self.mouthRollUpper)
            .field("mouthFunnel", &self.mouthFunnel)
            .field("mouthStretch_R", &self.mouthStretch_R)
            .field("mouthFrown_R", &self.mouthFrown_R)
            .field("eyeLookDown_R", &self.eyeLookDown_R)
            .field("jawOpen", &self.jawOpen)
            .field("jawLeft", &self.jawLeft)
            .field("browDown_L", &self.browDown_L)
            .field("mouthSmile_L", &self.mouthSmile_L)
            .field("noseSneer_R", &self.noseSneer_R)
            .field("mouthLowerDown_R", &self.mouthLowerDown_R)
            .field("noseSneer_L", &self.noseSneer_L)
            .field("eyeWide_L", &self.eyeWide_L)
            .field("mouthStretch_L", &self.mouthStretch_L)
            .field("browOuterUp_R", &self.browOuterUp_R)
            .field("eyeLookIn_L", &self.eyeLookIn_L)
            .field("eyeSquint_R", &self.eyeSquint_R)
            .field("eyeLookUp_L", &self.eyeLookUp_L)
            .field("mouthLeft", &self.mouthLeft)
            .field("head", &self.head)
            .field("rightEye", &self.rightEye)
            .field("leftEye", &self.leftEye)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::ifm_data::IfacialmocapData;

    #[test]
    fn test_ifm_data() {
        let test_data = "mouthSmile_R-0|eyeLookOut_L-0|mouthUpperUp_L-11|eyeWide_R-0|mouthClose-8|mouthPucker-4|mouthRollLower-9|eyeBlink_R-7|eyeLookDown_L-17|cheekSquint_R-11|eyeBlink_L-7|tongueOut-0|jawRight-0|eyeLookIn_R-6|cheekSquint_L-11|mouthDimple_L-10|mouthPress_L-4|eyeSquint_L-11|mouthRight-0|mouthShrugLower-9|eyeLookUp_R-0|eyeLookOut_R-0|mouthPress_R-5|cheekPuff-2|jawForward-11|mouthLowerDown_L-9|mouthFrown_L-6|mouthShrugUpper-26|browOuterUp_L-4|browInnerUp-20|mouthDimple_R-10|browDown_R-0|mouthUpperUp_R-10|mouthRollUpper-8|mouthFunnel-12|mouthStretch_R-21|mouthFrown_R-13|eyeLookDown_R-17|jawOpen-12|jawLeft-0|browDown_L-0|mouthSmile_L-0|noseSneer_R-18|mouthLowerDown_R-8|noseSneer_L-21|eyeWide_L-0|mouthStretch_L-21|browOuterUp_R-4|eyeLookIn_L-4|eyeSquint_R-11|eyeLookUp_L-0|mouthLeft-1|=head#-21.488958,-6.038993,-6.6019735,-0.030653415,-0.10287084,-0.6584072|rightEye#6.0297494,2.4403017,0.25649446|leftEye#6.034903,-1.6660284,-0.17520553|";
        let ifm_data = IfacialmocapData::from_str(test_data);
        println!("{:?}", &ifm_data);
    }
}