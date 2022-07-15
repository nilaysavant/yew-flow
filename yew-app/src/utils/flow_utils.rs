use serde::Serialize;
use anyhow::Result;
use serde_json::ser::PrettyFormatter;
use yew_flow::workspace::YewFlowValues;

/// # Values to flow json text
/// 
/// Convert state values to flow json text.
pub fn values_to_flow_json_text(values: &YewFlowValues) -> Result<String> {
    // pretty formatter with ident: ref: https://stackoverflow.com/questions/42722169/generate-pretty-indented-json-with-serde
    let mut ser =
        serde_json::Serializer::with_formatter(Vec::new(), PrettyFormatter::with_indent(b"    "));
    let json_value = serde_json::to_value(&(*values).clone())?;
    json_value.serialize(&mut ser)?;
    Ok(String::from_utf8(ser.into_inner())?)
}

/// # Parse Flow Json text to Values
/// 
/// Parse flow json text into values.
pub fn parse_flow_json_text_to_values(json_text: &str) -> Result<YewFlowValues> {
    let values = serde_json::from_str::<YewFlowValues>(&json_text)?;
    Ok(values)
}