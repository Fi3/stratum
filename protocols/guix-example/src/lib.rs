use serde_derive::{Deserialize, Serialize};
use serde_json;


#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[repr(C)]
pub struct Point {
    x: u32,
    y: u32,
}

impl Point {
    pub fn new(x: u32, y: u32) -> Self {
        Self {x, y}
    }

    pub fn from_str(s: &str) -> Option<Self> {
        serde_json::from_str(s).ok()
    }

    #[no_mangle]
    pub extern "C" fn sum(self, p: Self) -> Self {
        Self {
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }
}

#[test]
fn test() {
    let a = Point::from_str(r#"{"x": 10, "y": 10}"#).unwrap();
    let b = Point::from_str(r#"{"x": 10, "y": 10}"#).unwrap();
    let c = Point::from_str(r#"{"x": 20, "y": 20}"#).unwrap();
    assert_eq!(a.sum(b), c);
}
