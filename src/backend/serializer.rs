use serde_json::{Map, Value};

use super::{mapping::MapError, profile::ProfileCompositionMapping};

// Same as in assets/root_composition_script.js
const APPLY_CHECKBOX_KEY: &str = "__APPLY_CHECKBOX";

/// Serialize the mappings to a JSON object.
///
/// # Panics
///
/// Panics if the timestamp cannot be converted to a JSON number.
pub fn serialize_mappings(
    mappings: &Vec<ProfileCompositionMapping>,
    source: &Value,
    exclude_incomplete_data: bool,
    timestamp: Option<i64>,
) -> Result<Value, MapError> {
    let mut map = Map::with_capacity(mappings.len() + matches!(timestamp, Some(_)) as usize);
    if let Some(timestamp) = timestamp {
        map.insert(
            String::from("__TIMESTAMP"),
            Value::Number(
                serde_json::Number::from_f64(timestamp as f64)
                    .expect("failed to convert timestamp to JSON number"),
            ),
        );
    }
    for comp_mapping in mappings {
        let mut mapped_obj = comp_mapping.mapping.map(&source, exclude_incomplete_data)?;
        if let Some(ref key) = comp_mapping.enabled_checkbox_name {
            mapped_obj
                .as_object_mut()
                .unwrap()
                .insert(APPLY_CHECKBOX_KEY.to_owned(), Value::String(key.clone()));
        }
        map.insert(comp_mapping.subcomp_name.clone(), mapped_obj);
    }
    Ok(Value::Object(map))
}
