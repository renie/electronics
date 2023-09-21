use serde::{Serialize, Deserialize};
use serde_json::{Map, Value, Error};
use std::fmt::Display;

#[derive(Serialize, Deserialize)]
pub struct Gate {
    pub id: u8,
    pub name: String,
    pub next_components: Vec<u8>,
    pub inputs: Vec<u8>,
    pub inputs_map: Map<String, Value>
}

impl Display for Gate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Gate:\n\tid: {}\n\tname: {}", self.id, self.name)
    }
}

impl Gate {
    pub fn parse_json_list(json_list: &str) -> Result<Vec<Gate>, Error> {
        serde_json::from_str(json_list)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn parse_json_list_valid () {
        let data = json!([
            {
                "id": 0,
                "next_components": [1],
                "name": "AND",
                "inputs": [
                    1,1
                ],
                "inputs_map": {}
            },
            {
                "id": 1,
                "next_components": [],
                "name": "OR",
                "inputs": [],
                "inputs_map": {
                    "0": 0,
                    "1": 0
                }
            }
        ]);

        let list: Vec<Gate> = Gate::parse_json_list(data.to_string().as_str()).expect("Error parsing JSON");

        assert_eq!("OR", list.last().unwrap().name)
    }
    
    #[test]
    fn parse_json_list_invalid () -> Result<(), &'static str> {
        let data = r#"[
            {
                "id": 0,
                "next_components": [1],
                "name": "AND",
                "inputs": [
                    1,1
                ],
                "inputs_map": {}
            },
            {
                "id": 1,
                "next_components": [],
                "name": "OR",
                "inputs": [],
                "inputs_map": {
                    "0": 0,
                    "1": 0
                }
            
        ]"#;

        match Gate::parse_json_list(data) {
            Ok(_)=> Err("It parsed a JSON that should not be parsed."),
            Err(_)=> Ok(())
        }
    }
}
