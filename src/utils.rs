use super::*;

pub fn load_definitions_for<T>(path: &str) -> Result<Vec<T>, Box<dyn std::error::Error>>
    where T: serde::de::DeserializeOwned,
{
    let mut items: Vec<T> = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let data = fs::read_to_string(path)?;
            let structure: T = serde_json::from_str(&data).unwrap();
            items.push(structure);
        } else {
            continue;
        }
    }
    Ok(items)
}