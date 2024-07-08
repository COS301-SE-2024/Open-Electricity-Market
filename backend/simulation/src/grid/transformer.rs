use crate::grid::ToJson;
use rocket::serde::json::json;

#[cfg(test)]
mod tests;

pub struct Transformer {
    pub(crate) id: u32,
    pub(crate) ratio: f32,
    pub(crate) primary: u32,
    pub(crate) secondary: u32,
}

impl ToJson for Transformer {
    fn to_json(&self) -> String {
        json!({ "ID" : self.id,
                "Ratio" : self.ratio,
                "Primary Transmission Line" : self.primary,
                "Secondary Transmission Line" : self.secondary
        })
        .to_string()
    }
}

