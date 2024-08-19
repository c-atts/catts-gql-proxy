use jsonpath_rust::JsonPath;
use serde_json::Value;
use std::str::FromStr;

/// Applies a JSONPath filter to the response data and returns the filtered results.
pub fn filter_response_data(data: &Value, filter: &str) -> Result<Value, String> {
    let filter = JsonPath::from_str(filter).map_err(|e| e.to_string())?;
    let slice_of_data = filter.find_slice(data);

    let mut results = Vec::new();

    for filtered_value in slice_of_data {
        let filtered_data = filtered_value.to_data();
        results.push(filtered_data);
    }

    Ok(Value::Array(results))
}
