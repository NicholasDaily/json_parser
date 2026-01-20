use std::collections::HashMap;

#[derive(Clone)]
pub struct Json{
    pub value : JsonValue,
    pub key : Option<String>,
    pub children : Option<HashMap<String,Box<Json>>>
}


#[derive(Clone)]
pub enum JsonValue {
    Object,
    Array(Vec<JsonValue>),
    String(String),
    Number(f32),
    Bool(bool),
    Null,
}


impl Json {
    pub fn new() -> Json {
        Json{
            value : JsonValue::Null,
            key :  None,
            children : None,
        }
    }

    pub fn parse_json(input_data : &[u8]) -> Vec<Json>{
        let parsed_json = Vec::new(); //not all JSON will be written perfectly, but we
                                                 //should scoop up what we can
        let mut local_json_object = Json::new();
        
        for mut i in 0..input_data.len() {
            let input_char = input_data[i] as char;
            match input_char {
                '{' => {
                   //start object 
                },

                '}' => {
                    //end object
                },

                '[' => {
                    //start array
                },

                ']' => {
                    // end array
                },
                '"' => {
                    //start String or Key
                    let (string, offset) = Self::read_str(&input_data[i..]);
                    i+=offset;
                    
                    

                    continue;
                },

                ':' => {
                    //Next item will be a value with the preceding item being the key

                },

                ',' => {
                    //after every member definition and between items in arrays
                },

                _ => {
                    //white space or some value we need that holds actual information
                    
                    if input_char.is_ascii_whitespace() {
                        continue;
                    }
                },
            }
        }

        parsed_json
    }

    fn read_str(input_data :  &[u8]) -> (String, usize)  {
        let offset : usize = 0;
        
    }
}
