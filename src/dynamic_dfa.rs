use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;
    
fn get_states(json: &Value) -> Vec<String> {
        let input_states: &Vec<Value> = json["states"].as_array().unwrap();
    
        let mut states: Vec<String> = Vec::new();
    
        for state in input_states {
            states.push(state.as_str().unwrap().to_string());
        }
    
        states
}

fn get_tokens(json: &Value) -> Vec<String> {
    let input_tokens: &Vec<Value> = json["tokens"].as_array().unwrap();

    let mut tokens: Vec<String> = Vec::new();

    for token in input_tokens {
        tokens.push(token.as_str().unwrap().to_string());
    }

    tokens
}

fn get_transitions(
    json: &Value,
    states: &Vec<String>,
    tokens: &Vec<String>,
) -> HashMap<String, HashMap<String, String>> {
    let mut dfa: HashMap<String, HashMap<String, String>> = HashMap::new();

    for state in states {
        let mut transition_for_state: HashMap<String, String> = HashMap::new();

        for token in tokens {
            let transition = json["transitions"][state.as_str()][token.as_str()]
                .as_str()
                .unwrap()
                .to_string();
            transition_for_state.insert(token.to_string(), transition);
        }

        dfa.insert(state.to_string(), transition_for_state);
    }

    dfa
}

fn get_json() -> Value {
    // Read input_dfa.json
    let file = File::open("src/input_dfa.json").unwrap();
    let reader = BufReader::new(file);
    // Parse the json file
    let json: Value = serde_json::from_reader(reader).unwrap();

    json
}

fn get_accepted_states(json: &Value) -> HashSet<String> {
    let input_accepted_states: &Vec<Value> = json["accepted_states"].as_array().unwrap();

    let mut accepted_states: HashSet<String> = HashSet::new();

    for state in input_accepted_states {
        accepted_states.insert(state.as_str().unwrap().to_string());
    }

    accepted_states
}

fn get_input_token_array(s: &str, tokens: &Vec<String>) -> Result<Vec<String>, ()> {
    let mut input_token_array: Vec<String> = Vec::new();
    let mut current_token = String::new();

    for char in s.chars() {
        if char == ' ' {
            if current_token != "" {
                input_token_array.push(current_token);
                current_token = String::new();
            }
            continue;
        }
        current_token.push(char);
        if tokens.contains(&current_token) {
            input_token_array.push(current_token);
            current_token = String::new();
        } else {
            current_token.push(char);
        }
    }

    if current_token == "" {
        Ok(input_token_array)
    } else {
        Err(())
    }
}

fn get_name(state: &str) -> String {
    match state{
        "identifier" => "id".to_string(),
        "number" => "number".to_string(),
        "operator+" => "+".to_string(),
        "operator-" => "-".to_string(),
        "operator/" => "/".to_string(),
        "operator*" => "*".to_string(),
        "o_symbol" => "(".to_string(),
        "c_symbol" => ")".to_string(),
        _ => "".to_string()
    }
}

fn analyze_input(
    input_token_array: &Vec<String>,
    dfa: &HashMap<String, HashMap<String, String>>,
    initial_state: &String,
    accepted_states: &HashSet<String>,
) ->    Vec<String> 
{

    let mut current_state = initial_state.to_string();
    let mut buffer = String::new();
    let mut data = Vec::new();

    for token in input_token_array {
        let next_state = &dfa[&current_state][token];
            if (!next_state.contains(current_state.as_str()) && !buffer.is_empty()) ||  (!accepted_states.contains(&current_state) && !buffer.is_empty()){ 
                println!("token: {}     token_name: {}", buffer, get_name(&current_state));
                data.push(get_name(&current_state));
                buffer = String::new();
            }
            buffer.push_str(token);
            current_state = next_state.to_string();
    }
    println!("token: {}     token_name: {}", buffer, get_name(&current_state));
    data.push(get_name(&current_state));
    data.push("$".to_string());
    data

}

fn grammar_check(buffer: &Vec<String>) -> Result<(), ()>{
    let mut token: String = "".to_string();
    let mut pos = 0;

    if (e(&mut pos,&buffer,&mut token) == 1) && (token == "") {
        return Ok(());
    }
    Err(())
}

fn ex(
    pos : &mut i32,
    buffer:&Vec<String>,
    token: &mut String)->
    i32{
    let actual_pos = *pos;
    if read(pos,buffer,token) == 1{
        if t(pos,buffer,token) == 1{
            return ex(pos,buffer,token)
        } else {return 0}
    }
    *pos = actual_pos;
    if read(pos,buffer,token) == 2{
        if t(pos,buffer,token) == 1{
            return ex(pos,buffer,token)
        } else {return 0}
    }
    *pos = actual_pos;
    return 1
}

fn tx(
    pos : &mut i32,
    buffer:&Vec<String>,
    token: &mut String)->
    i32{
    let actual_pos = *pos;
    if read(pos,buffer,token) == 3{
        if f(pos,buffer,token) == 1{
            return tx(pos,buffer,token)
        } else {return 0}
    }
    *pos = actual_pos;
    if read(pos,buffer,token) == 4{
        if f(pos,buffer,token) == 1{
            return tx(pos,buffer,token)
        } else {return 0}
    }
    *pos = actual_pos;
    return 1
}

fn f(
    pos : &mut i32,
    buffer:&Vec<String>,
    token: &mut String)->
    i32 {
    let actual_pos = *pos;
    if read(pos,buffer,token) == 5 {
      if e(pos,buffer,token) == 1 {
        if read(pos,buffer,token) == 6 {
          return 1
        }
      }
    }
    *pos = actual_pos;
    if read(pos,buffer,token) == 7 {
      return 1
    }
    *pos = actual_pos;
    if read(pos,buffer,token) == 8{
      return 1
    }
    return 0
  }

  fn t(
    pos : &mut i32,
    buffer:&Vec<String>,
    token: &mut String)->
    i32{
        if f(pos,buffer,token) == 1 {
            return tx(pos,buffer,token) 
          }
    return 0
    }

fn e(
    pos : &mut i32,
    buffer:&Vec<String>,
    token: &mut String)->
    i32{
        if t(pos,buffer,token) == 1 {
            if ex(pos,buffer,token) == 1 {
              return 1
            }
          }
        return 0
    }

fn read(    
    pos : &mut i32,
    buffer:&Vec<String>,
    token: &mut String)-> 
    i32{
    if buffer[*pos as usize] == "+"{
        *pos = *pos + 1;
        *token = "+".to_string();
        return 1
    }else if buffer[*pos as usize] == "-"{
        *pos = *pos + 1;
        *token = "-".to_string();
        return 2
    }else if buffer[*pos as usize] == "/"{
        *pos = *pos + 1;
        *token = "/".to_string();
        return 3
    }else  if buffer[*pos as usize] == "*"{
        *pos = *pos + 1;
        *token = "*".to_string();
        return 4
    } else if buffer[*pos as usize] == "("{
        *pos = *pos + 1;
        *token = "(".to_string();
        return 5
    } else if buffer[*pos as usize] == ")"{
        *pos = *pos + 1;
        *token = ")".to_string();
        return 6
    } else if buffer[*pos as usize] == "number"{
        *pos = *pos + 1;
        *token = "number".to_string();
        return 7
    } else if buffer[*pos as usize] == "id"{
        *pos = *pos + 1;
        *token = "id".to_string();
        return 8
    }
    *token = "".to_string();
    return 0
}  

pub fn validate(s: &str) -> Result<(), ()> {
    let json = get_json();
    let states = get_states(&json);
    let tokens = get_tokens(&json);
    let accepted_states: HashSet<String> = get_accepted_states(&json);
    let dfa = get_transitions(&json, &states, &tokens);
    let initial_state = json["initial_state"].as_str().unwrap().to_string();
    let input_token_array: Vec<String> = get_input_token_array(s, &tokens)?;
    let token_vector: Vec<String> = analyze_input(&input_token_array, &dfa, &initial_state, &accepted_states);
    //let token_table: indexmap::IndexMap<String, (String,i32)> = validate_input_and_print_symbol_table(&input_token_array, &dfa, &initial_state, &accepted_states);
    grammar_check(&token_vector)?;
    return Ok(());
}