use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Guide {
    pub page_no: i32,
    pub pages: i32,
}
