use std::fmt::format;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use log::log;

// We need to add Deserialize and Serialize, so we could convert data between languages.
#[derive(Debug, Deserialize, Serialize)]
pub struct ObjectDefinition {
    pub id : u16,
    pub request_status : String,
    pub customer_name : String,
    pub customer_email : String,
    pub customer_self_description : String,
    pub date_time_added : String,
    pub notes : Vec<NoteObject>
}
#[derive(Debug, Deserialize, Serialize)]
pub struct NoteObject {
    pub id : u16,
    pub text_info : String,
    pub date_time : String
}
// To pass a struct as argument from JS we stringify it and pass as &str. When we receive it here, we use serde_json::from_str to convert it to Rust's struct.
#[wasm_bindgen]
pub fn process_filtration(filter_action : &str, filter_type : String, filter_query : String) -> String {
    let to_filter : Vec<ObjectDefinition> = serde_json::from_str(filter_action).unwrap();
    if filter_type == "ID" {
        serde_json::to_string(
            &to_filter
                .into_iter()
                .filter(|object| object.id.to_string().contains(&filter_query.to_lowercase()))
                .collect::<Vec<ObjectDefinition>>()
        ).unwrap()
    }
    else if filter_type == "Статус" {
        serde_json::to_string(
            &to_filter
                .into_iter()
                .filter(|object| object.request_status.to_lowercase().contains(&filter_query.to_lowercase()))
                .collect::<Vec<ObjectDefinition>>()
        ).unwrap()
    }
    else if filter_type == "Имя" {
        serde_json::to_string(
            &to_filter
                .into_iter()
                .filter(|object| object.customer_name.to_lowercase().contains(&filter_query.to_lowercase()))
                .collect::<Vec<ObjectDefinition>>()
        ).unwrap()
    }
    else if filter_type == "Почта" {
        serde_json::to_string(
            &to_filter
                .into_iter()
                .filter(|object| object.customer_email.to_lowercase().contains(&filter_query.to_lowercase()))
                .collect::<Vec<ObjectDefinition>>()
        ).unwrap()
    }
    else {
        serde_json::to_string(
            &to_filter
                .into_iter()
                .filter(|object| object.customer_self_description.to_lowercase().contains(&filter_query.to_lowercase()))
                .collect::<Vec<ObjectDefinition>>()
        ).unwrap()
    }
}

// #[wasm_bindgen]
// pub fn process_filtration2(filter_action : &str, filter_type : String, filter_query : String) -> String {
//     wasm_logger::init(wasm_logger::Config::default());
//     match serde_json::from_str::<Vec<ObjectDefinition>>(&filter_action) {
//         Ok(value) => {
//             log::info!("{}", format!("{:#?}", value));
//             match serde_json::to_string(&value) {
//                 Ok(_) => {}
//                 Err(_) => {}
//             }
//         }
//         Err(e) => {
//             log::info!("{}", format!("{}", e))
//         }
//     }
// }

