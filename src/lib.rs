use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ObjectDefinition {
    id : u16,
    request_status : String,
    customer_name : String,
    customer_email : String,
    customer_self_description : String,
    date_time_added : String
}
#[wasm_bindgen]
impl ObjectDefinition {
    #[wasm_bindgen(getter)]
    pub fn get_id(&self) -> String {self.id.to_string().clone()}

    #[wasm_bindgen(getter)]
    pub fn get_request_status(&self) -> String {self.request_status.clone().to_lowercase()}

    #[wasm_bindgen(getter)]
    pub fn get_customer_name(&self) -> String {self.customer_name.clone().to_lowercase()}

    #[wasm_bindgen(getter)]
    pub fn get_customer_email(&self) -> String {self.customer_email.clone().to_lowercase()}

    #[wasm_bindgen(getter)]
    pub fn get_customer_description(&self) -> String {self.customer_self_description.clone().to_lowercase()}

    #[wasm_bindgen(getter)]
    pub fn get_date_time(&self) -> String {self.date_time_added.clone()}
}

#[wasm_bindgen]
pub fn process_filtration(to_filter : Vec<ObjectDefinition>, filter_type : String, filter_query : String) -> Vec<ObjectDefinition> {
    if filter_type == "ID" {
        to_filter.into_iter().filter(|object| object.get_id().contains(&filter_query.to_lowercase())).collect::<Vec<ObjectDefinition>>()
    }
    else if filter_type == "Статус" {
        to_filter.into_iter().filter(|object| object.get_request_status().contains(&filter_query.to_lowercase())).collect::<Vec<ObjectDefinition>>()
    }
    else if filter_type == "Имя" {
        to_filter.into_iter().filter(|object| object.get_customer_name().contains(&filter_query.to_lowercase())).collect::<Vec<ObjectDefinition>>()
    }
    else if filter_type == "Почта" {
        to_filter.into_iter().filter(|object| object.get_customer_email().contains(&filter_query.to_lowercase())).collect::<Vec<ObjectDefinition>>()
    }
    else {
        to_filter.into_iter().filter(|object| object.get_customer_description().contains(&filter_query.to_lowercase())).collect::<Vec<ObjectDefinition>>()
    }
}
