use axum::Json;
use serde::{Deserialize, Serialize};
use crate::customerror::CustomError;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Door {
    isbn: String,
    name: String
}

pub async fn get_doors() -> Result<Json<Vec<Door>>, CustomError> {
    let vector: Vec<Door> = vec![
        Door {isbn: "isbn::local_01".to_string(), name: "local".to_string()},
        Door {isbn: "isbn::local_10".to_string(), name: "local".to_string()},
        Door {isbn: "isbn::local_11".to_string(), name: "local".to_string()},
    ];
    Ok(Json(vector))
}