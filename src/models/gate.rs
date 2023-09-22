use serde::{Serialize, Deserialize};
use serde_json::{Map, Value, Error};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum GateType {
   AND, OR
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Gate {
    pub id: u8,
    pub name: GateType,
    pub next_components: Vec<u8>,
    pub inputs: Vec<i8>,
    pub inputs_map: Map<String, Value>
}

impl Gate {
    pub fn parse_json_list(json_list: &str) -> Result<Vec<Gate>, Error> {
        serde_json::from_str(json_list)
    }

    pub fn process_result(&self) -> bool {
        if self.inputs.len() < 2 {
            return false
        }

        let prepared_inputs = self.prepare_inputs();

        match self.name {
            GateType::AND => {
                return prepared_inputs[0] & prepared_inputs[1]
            },
            GateType::OR => {
                return prepared_inputs[0] | prepared_inputs[1]
            }
        }
    }

    fn prepare_inputs(&self) -> Vec<bool> {
        self.inputs.iter().map(|input| input > &0).collect()
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

        assert_eq!(GateType::OR, list.last().unwrap().name)
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

    #[test]
    fn process_result_andgate () {
        let data1 = json!([
            {
                "id": 0,
                "next_components": [],
                "name": "AND",
                "inputs": [
                    0,0
                ],
                "inputs_map": {}
            }
        ]);

        let data2 = json!([
            {
                "id": 0,
                "next_components": [],
                "name": "AND",
                "inputs": [
                    1,0
                ],
                "inputs_map": {}
            }
        ]);

        let data3 = json!([
            {
                "id": 0,
                "next_components": [],
                "name": "AND",
                "inputs": [
                    0,1
                ],
                "inputs_map": {}
            }
        ]);

        let data4 = json!([
            {
                "id": 0,
                "next_components": [],
                "name": "AND",
                "inputs": [
                    1,1
                ],
                "inputs_map": {}
            }
        ]);

        let list1: Vec<Gate> = Gate::parse_json_list(data1.to_string().as_str()).expect("Error parsing JSON");
        let list2: Vec<Gate> = Gate::parse_json_list(data2.to_string().as_str()).expect("Error parsing JSON");
        let list3: Vec<Gate> = Gate::parse_json_list(data3.to_string().as_str()).expect("Error parsing JSON");
        let list4: Vec<Gate> = Gate::parse_json_list(data4.to_string().as_str()).expect("Error parsing JSON");

        assert_eq!(false, list1.first().unwrap().process_result());
        assert_eq!(false, list2.first().unwrap().process_result());
        assert_eq!(false, list3.first().unwrap().process_result());
        assert_eq!(true, list4.first().unwrap().process_result());
    }
    
    #[test]
    fn process_result_orgate () {
        let data1 = json!([
            {
                "id": 0,
                "next_components": [],
                "name": "OR",
                "inputs": [
                    0,0
                ],
                "inputs_map": {}
            }
        ]);

        let data2 = json!([
            {
                "id": 0,
                "next_components": [],
                "name": "OR",
                "inputs": [
                    1,0
                ],
                "inputs_map": {}
            }
        ]);

        let data3 = json!([
            {
                "id": 0,
                "next_components": [],
                "name": "OR",
                "inputs": [
                    0,1
                ],
                "inputs_map": {}
            }
        ]);

        let data4 = json!([
            {
                "id": 0,
                "next_components": [],
                "name": "OR",
                "inputs": [
                    1,1
                ],
                "inputs_map": {}
            }
        ]);

        let list1: Vec<Gate> = Gate::parse_json_list(data1.to_string().as_str()).expect("Error parsing JSON");
        let list2: Vec<Gate> = Gate::parse_json_list(data2.to_string().as_str()).expect("Error parsing JSON");
        let list3: Vec<Gate> = Gate::parse_json_list(data3.to_string().as_str()).expect("Error parsing JSON");
        let list4: Vec<Gate> = Gate::parse_json_list(data4.to_string().as_str()).expect("Error parsing JSON");

        assert_eq!(false, list1.first().unwrap().process_result());
        assert_eq!(true, list2.first().unwrap().process_result());
        assert_eq!(true, list3.first().unwrap().process_result());
        assert_eq!(true, list4.first().unwrap().process_result());
    }
}
