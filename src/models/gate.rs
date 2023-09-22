use serde::{Serialize, Deserialize};
use serde_json::{Map, Value, Error};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum GateType {
   AND, OR
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Gate {
    pub id: u8,
    pub name: GateType,
    pub next_components: Vec<u8>,
    pub inputs: Vec<i8>,
    pub inputs_map: Map<String, Value>
}

impl Gate {
    pub fn process_circuit_result(circuit: Vec<Gate>) -> bool {
        Gate::process_component(circuit.first().unwrap(), circuit.clone().as_mut())
    }

    fn process_component(component: &Gate, circuit: &mut Vec<Gate>) -> bool {
        let result = component.process_result();
        if component.next_components.is_empty() {
            return result
        }

        for next_component_id in component.next_components.iter() {
            let mut i = 0;
            let mut next_component = Gate {
                id: 0,
                name: GateType::OR,
                inputs: Vec::new(),
                inputs_map: Map::new(),
                next_components: Vec::new()
            };
            while i < circuit.len() {
                if circuit[i].id == next_component_id.to_owned() {
                    let old_component = circuit.get(i).unwrap();
                    next_component.id = old_component.id;
                    next_component.name = old_component.name;


                    let index = format!("{}", old_component.inputs_map.get(format!("{}", component.id).as_str()).unwrap());
                    let idx = usize::from_str_radix(index.as_str(),10).unwrap();

                    next_component.inputs.insert(idx, if result { 1 } else { 0 } );

                    if old_component.inputs_map.len() < 2 {
                        next_component.inputs.insert(idx+1, 0);
                    }

                    circuit.push(next_component.clone());
                    circuit.swap_remove(i);
                    break;
                }
                i += 1;
            };

            return Gate::process_component(&next_component, circuit);
        }

        return false
    }


    pub fn parse_json_list(json_list: &str) -> Result<Vec<Gate>, Error> {
        serde_json::from_str(json_list)
    }

    pub fn process_result(&self) -> bool {
        if self.inputs.is_empty()  {
            return false
        }

        let prepared_inputs = self.prepare_inputs();

        match self.name {
            GateType::AND => {
                if self.inputs.len() < 2 {
                    return false
                }

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
