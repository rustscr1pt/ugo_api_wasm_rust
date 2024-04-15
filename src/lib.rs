mod structs;

use wasm_bindgen::prelude::*;
use crate::structs::ObjectDefinition;

#[wasm_bindgen]
pub fn process_filtration(to_filter : Vec<ObjectDefinition>, filter_type : String, filter_query : String) -> Vec<ObjectDefinition> {
    if filter_type == "ID" {
        to_filter.into_iter().filter(|object| object.id.to_lowercase().contains(&filter_query.to_lowercase())).collect::<Vec<ObjectDefinition>>()
    }
    else if filter_type == "Статус" {
        to_filter.into_iter().filter(|object| object.request_status.to_lowercase().contains(&filter_query.to_lowercase())).collect::<Vec<ObjectDefinition>>()
    }
    else if filter_type == "Имя" {

    }
}
