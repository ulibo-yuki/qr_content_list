use core::fmt;
use std::{fs, path::PathBuf};
use serde::{Serialize, Deserialize};

use super::super::qr_generator;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TwoDBoxList {
    pub id: i32,
    pub title: String,
    pub last_edit_time: String,
    pub qr_img_path: PathBuf,
    pub content_list: Vec<String>,
}

impl fmt::Display for TwoDBoxList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str = "".to_string();
        for i in self.content_list.clone() {
            str.push_str(&i);
        }
        write!(f, "{}", str)
    }
}

static DATA_FILENAME: &str = "data.json";

pub fn get_all() -> Vec<TwoDBoxList> {
    let file = fs::read_to_string(DATA_FILENAME).unwrap();
    let mut json_data: Vec<TwoDBoxList> = serde_json::from_str(&file).unwrap();
    json_data.sort_by(|a, b| b.last_edit_time.cmp(&a.last_edit_time));
    json_data
}

pub fn create(mut content: TwoDBoxList) -> TwoDBoxList {
    let file = fs::read_to_string(DATA_FILENAME).unwrap();
    let mut json_data: Vec<TwoDBoxList> = serde_json::from_str(&file).unwrap();
    let mut max = 0;
    for item in &json_data {
        max = std::cmp::max(item.id, max);
    }
    content.id = max + 1;
    let _url_dev = "http://localhost:8080/qr_list/";
    let url = "http://192.168.1.100:8080/qr_list/";
    content.qr_img_path = qr_generator::make_qr_img(format!("{}{}",url, content.id), content.id);
    json_data.push(content);
    let json_str = serde_json::to_string(&json_data).unwrap();
    let _ = fs::write(DATA_FILENAME, json_str);
    json_data.pop().unwrap()
}

pub fn get(id: i32) -> TwoDBoxList {
    let file = fs::read_to_string(DATA_FILENAME).unwrap();
    let jsno_data: Vec<TwoDBoxList> = serde_json::from_str(&file).unwrap();
    let mut content = TwoDBoxList {
        id: 0,
        title: "".to_string(),
        last_edit_time: "".to_string(),
        qr_img_path: PathBuf::new(),
        content_list: vec!["".to_string()],
    };
    if let Some(index) = jsno_data.iter().position(|item| item.id == id) {
        content = jsno_data[index].clone();
    }
    content
}