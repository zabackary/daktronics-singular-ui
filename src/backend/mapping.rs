use std::{error::Error, fmt::Display};

use serde::{Deserialize, Serialize};
use transformation::{Transformation, TransformationError};

pub mod transformation {
    use std::{error::Error, fmt::Display, num::ParseIntError};

    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq)]
    pub enum Transformation {
        #[default]
        None,
        TimeMinutes,
        TimeSeconds,
        TimeTenths,
        AppendOrdinalSuffix,
    }

    impl Display for Transformation {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(match self {
                Transformation::None => "No transformation",
                Transformation::TimeMinutes => "Extract minutes from time",
                Transformation::TimeSeconds => "Extract seconds from time",
                Transformation::TimeTenths => "Extract tenths from time",
                Transformation::AppendOrdinalSuffix => "Append ordinal suffix to number",
            })
        }
    }

    impl Transformation {
        pub const ALL: [Transformation; 5] = [
            Transformation::None,
            Transformation::TimeMinutes,
            Transformation::TimeSeconds,
            Transformation::TimeTenths,
            Transformation::AppendOrdinalSuffix,
        ];

        pub fn transform(
            &self,
            value: &serde_json::Value,
        ) -> Result<serde_json::Value, TransformationError> {
            match self {
                Transformation::None => Ok(value.clone()),
                Transformation::TimeMinutes => {
                    let minute_seconds: Vec<_> = value
                        .as_str()
                        .ok_or_else(|| TransformationError::UnexpectedSourceType(value.clone()))?
                        .trim()
                        // Take off the tenths
                        .split(".")
                        .next()
                        .ok_or(TransformationError::DataExtractionFailed)?
                        .split(":")
                        .collect();
                    Ok(serde_json::Value::Number(if minute_seconds.len() < 2 {
                        // no minutes component exists, so 0
                        serde_json::Number::from(0)
                    } else {
                        serde_json::Number::from(
                            minute_seconds
                                // Take off the seconds
                                [0]
                            // Parse as a number
                            .parse::<i32>()
                            .map_err(TransformationError::ParseInt)?,
                        )
                    }))
                }
                Transformation::TimeSeconds => {
                    Ok(serde_json::Value::Number(serde_json::Number::from(
                        value
                            .as_str()
                            .ok_or_else(|| {
                                TransformationError::UnexpectedSourceType(value.clone())
                            })?
                            .trim()
                            // Take off the tenths
                            .split(".")
                            .next()
                            .ok_or(TransformationError::DataExtractionFailed)?
                            // Take off the seconds
                            .split(":")
                            .last() // there might not be a minutes component
                            .ok_or(TransformationError::DataExtractionFailed)?
                            // Parse as a number
                            .parse::<i32>()
                            .map_err(TransformationError::ParseInt)?,
                    )))
                }
                Transformation::TimeTenths => {
                    let split: Vec<_> = value
                        .as_str()
                        .ok_or_else(|| TransformationError::UnexpectedSourceType(value.clone()))?
                        .trim()
                        // Take off the tenths
                        .split(".")
                        .collect();
                    Ok(serde_json::Value::Number(if split.len() > 1 {
                        serde_json::Number::from(
                            split[2] // Parse as a number
                                .parse::<i32>()
                                .map_err(TransformationError::ParseInt)?,
                        )
                    } else {
                        // No tenths component exists
                        serde_json::Number::from(0)
                    }))
                }
                Transformation::AppendOrdinalSuffix => {
                    let mut s = value
                        .as_i64()
                        .ok_or_else(|| TransformationError::UnexpectedSourceType(value.clone()))?
                        .to_string();
                    s.push_str(if s.ends_with('1') && !s.ends_with("11") {
                        "st"
                    } else if s.ends_with('2') && !s.ends_with("12") {
                        "nd"
                    } else if s.ends_with('3') && !s.ends_with("13") {
                        "rd"
                    } else {
                        "th"
                    });
                    Ok(serde_json::Value::String(s))
                }
            }
        }
    }

    #[derive(Debug)]
    pub enum TransformationError {
        UnexpectedSourceType(serde_json::Value),
        DataExtractionFailed,
        ParseInt(ParseIntError),
    }

    impl Display for TransformationError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                TransformationError::UnexpectedSourceType(value) => write!(
                    f,
                    "unexpected source type: {}",
                    match value {
                        serde_json::Value::Array(_) => "array",
                        serde_json::Value::Null => "null",
                        serde_json::Value::Bool(_) => "bool",
                        serde_json::Value::Number(_) => "number",
                        serde_json::Value::String(_) => "string",
                        serde_json::Value::Object(_) => "object",
                    }
                ),
                TransformationError::DataExtractionFailed => write!(f, "data extraction failed"),
                TransformationError::ParseInt(err) => write!(f, "failed to parse int: {}", err),
            }
        }
    }

    impl Error for TransformationError {}
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Mapping {
    pub items: Vec<MappingItem>,
}

impl Mapping {
    pub fn map(&self, source: &serde_json::Value) -> Result<serde_json::Value, MapError> {
        let source_map = source.as_object().ok_or(MapError::SourceNotMap)?;
        // map could be underfilled if fields are disabled, but that's okay
        let mut destination = serde_json::Map::with_capacity(self.items.len());
        for item in &self.items {
            item.map(source_map, &mut destination)?;
        }
        Ok(serde_json::Value::Object(destination))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MappingItem {
    pub enabled: bool,
    pub source_field: String,
    pub transformation: Transformation,
    pub destination_field: String,
}

impl MappingItem {
    pub fn map(
        &self,
        source: &serde_json::Map<String, serde_json::Value>,
        destination: &mut serde_json::Map<String, serde_json::Value>,
    ) -> Result<(), MapError> {
        if self.enabled {
            let source_value = source
                .get(&self.source_field)
                .ok_or_else(|| MapError::SourceFieldNonExistent(self.source_field.clone()))?;
            let transformed = self
                .transformation
                .transform(source_value)
                .map_err(MapError::Transformation)?;
            let previous_value = destination.insert(self.destination_field.clone(), transformed);
            if let Some(_) = previous_value {
                Err(MapError::DestinationFieldAlreadyPresent(
                    self.destination_field.clone(),
                ))
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub enum MapError {
    SourceNotMap,
    SourceFieldNonExistent(String),
    DestinationFieldAlreadyPresent(String),
    Transformation(TransformationError),
}

impl Display for MapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MapError::SourceNotMap => write!(f, "source is not a map"),
            MapError::SourceFieldNonExistent(field) => {
                write!(f, "source field {} does not exist", field)
            }
            MapError::DestinationFieldAlreadyPresent(field) => {
                write!(f, "destination field {} is already present in the output; maybe there's a duplicate?", field)
            }
            MapError::Transformation(err) => write!(f, "transformation error: {}", err),
        }
    }
}

impl Error for MapError {}
