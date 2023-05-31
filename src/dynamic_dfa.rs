use serde_json::{Value};
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

fn get_grammar_table()
-> HashMap<String, HashMap<String,String>>{
    let mut grammar_table: HashMap<String, HashMap<String,String>> = HashMap::new();
    let mut row: HashMap<String,String> = HashMap::new();
//  |                       id                         |                                     +                                          |                               *                                    |                                       (                                             |                                           )                                        |                                       $                                        |
    row.insert("i".to_string(),"XT".to_string());       row.insert("n".to_string(),"XT".to_string());       row.insert("+".to_string(),"Error: Expected 'id' before '+' token".to_string());  row.insert("-".to_string(),"Error: Expected 'id' before '-' token".to_string());  row.insert("*".to_string(),"Error: Expected 'id' before '*' token".to_string()); row.insert("/".to_string(),"Error: Expected 'id' before '/' token".to_string());       row.insert("(".to_string(),"XT".to_string());                                        row.insert(")".to_string(),"Error: missing '(' token or expression".to_string());    row.insert("$".to_string(),"Error: missing expressions".to_string());
    grammar_table.insert("E".to_string(),row);
    row = HashMap::new();
    row.insert("i".to_string(),"Error 1".to_string());  row.insert("n".to_string(),"Error 1".to_string());  row.insert("+".to_string(),"XT+".to_string());                                    row.insert("-".to_string(),"XT-".to_string());                                    row.insert("*".to_string(),"Error 6".to_string());                               row.insert("/".to_string(),"Error 6".to_string());                                     row.insert("(".to_string(),"Error 10".to_string());                                  row.insert(")".to_string(),"".to_string());                                          row.insert("$".to_string(),"".to_string());
    grammar_table.insert("X".to_string(),row);
    row = HashMap::new();
    row.insert("i".to_string(),"YF".to_string());       row.insert("n".to_string(),"YF".to_string());       row.insert("+".to_string(),"Error: double '+' token".to_string());                row.insert("-".to_string(),"Error: double '-' token".to_string());                row.insert("*".to_string(),"Error 7".to_string());                               row.insert("/".to_string(),"Error 7".to_string());                                     row.insert("(".to_string(),"YF".to_string());                                        row.insert(")".to_string(),"Error: remaining ')' after '+' token".to_string());      row.insert("$".to_string(),"Error: Expected 'id' after '+' or '-' token".to_string());
    grammar_table.insert("T".to_string(),row);
    row = HashMap::new();
    row.insert("i".to_string(),"Error 2".to_string());  row.insert("n".to_string(),"Error 2".to_string());  row.insert("+".to_string(),"".to_string());                                       row.insert("-".to_string(),"".to_string());                                       row.insert("*".to_string(),"YF*".to_string());                                   row.insert("/".to_string(),"YF/".to_string());                                         row.insert("(".to_string(),"Error: missing operator before '(' token".to_string());  row.insert(")".to_string(),"".to_string());                                          row.insert("$".to_string(),"".to_string());
    grammar_table.insert("Y".to_string(),row);
    row = HashMap::new();
    row.insert("i".to_string(),"i".to_string());        row.insert("n".to_string(),"n".to_string());        row.insert("+".to_string(),"Error 4".to_string());                                row.insert("-".to_string(),"Error 20".to_string());                               row.insert("*".to_string(),"Error: double '*' token".to_string());               row.insert("/".to_string(),"Error: double '/' token".to_string());                     row.insert("(".to_string(),")E(".to_string());                                       row.insert(")".to_string(),"Error: remaining ')' after '*' token".to_string());      row.insert("$".to_string(),"Error: Expected 'id' after '*' or '/' token".to_string());
    grammar_table.insert("F".to_string(),row);
    grammar_table
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
                //println!("token: {}     token_name: {}", buffer, current_state);
                data.push(current_state);
                buffer = String::new();
            }
            buffer.push_str(token);
            current_state = next_state.to_string();
    }
    //println!("token: {}     token_name: {}", buffer, current_state);
    data.push(current_state);
    data.push("$".to_string());
    data

}

fn grammar_check(
    buffer: &Vec<String>,
    grammar_table: &HashMap<String, HashMap<String,String>>
) -> Result<(), ()>{
    let mut pila = Vec::<String>::new();
    let mut index = 0;
    let terminal = vec!["+".to_string(),"*".to_string(),"(".to_string(),")".to_string(),"i".to_string(), "$".to_string(), "-".to_string(), "/".to_string(), "n".to_string()];
    pila.push("$".to_string());
    pila.push("E".to_string());

    while !(pila.is_empty()){
        let top = pila.pop().unwrap();
        let token = &buffer[index];
        //println!("top: {}     token: {}    pila: {:?}     ", top, token, pila);
        if terminal.contains(&top) {
            if top.eq(token){
                index = index + 1;
            } else{
                println!("Error: missing ')' or '(' token or expression");
                return Err(());
            }
        } else{
            let action = &grammar_table[&top][token]; 
            if action.contains("Error"){
                println!("{}",action);
                return Err(());
            }else{
                for chars in action.chars(){
                    pila.push(chars.to_string());
                }
            }
        }
    }
    Ok(())
}

pub fn validate(s: &str) -> Result<(), ()> {
    let json = get_json();
    let states = get_states(&json);
    let tokens = get_tokens(&json);
    let accepted_states: HashSet<String> = get_accepted_states(&json);
    let dfa = get_transitions(&json, &states, &tokens);
    let initial_state = json["initial_state"].as_str().unwrap().to_string();
    let grammar_table = get_grammar_table();
    let input_token_array: Vec<String> = get_input_token_array(s, &tokens)?;
    let token_vector: Vec<String> = analyze_input(&input_token_array, &dfa, &initial_state, &accepted_states);
    //println!("{:?}", token_vector);
    grammar_check(&token_vector,&grammar_table)?;
    return Ok(());
}