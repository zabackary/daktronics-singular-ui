use serde_json::{Map, Value};

use super::{mapping::MapError, profile::ProfileCompositionMapping};

// Same as in assets/root_composition_script.js
const APPLY_CHECKBOX_KEY: &str = "__APPLY_CHECKBOX";

pub fn serialize_mappings(
    mappings: &Vec<ProfileCompositionMapping>,
    source: &Value,
    exclude_incomplete_data: bool,
) -> Result<Value, MapError> {
    let mut map = Map::with_capacity(mappings.len());
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
