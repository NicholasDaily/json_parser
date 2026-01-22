use std::collections::HashMap;

#[derive(Clone)]
pub struct Json {
    pub value: JsonValue,
    pub key: Option<String>,
    pub children: Option<HashMap<String, Box<Json>>>,
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
        Json {
            value: JsonValue::Null,
            key: None,
            children: None,
        }
    }

    pub fn parse_json(input_data: &[u8]) -> Vec<Json> {
        let parsed_json = Vec::new(); //not all JSON will be written perfectly, but we
        //should scoop up what we can
        let mut local_json_object = Json::new();

        for mut i in 0..input_data.len() {
            let input_char = input_data[i] as char;
            match input_char {
                '{' => {
                    //start object
                }

                '}' => {
                    //end object
                }

                '[' => {
                    //start array
                }

                ']' => {
                    // end array
                }
                '"' => {
                    //start String or Key
                    let (string, offset) = Self::read_str(&input_data[i..]);
                    let json_value = JsonValue::String(string);
                    i += offset;

                    continue;
                }

                ':' => {
                    //Next item will be a value with the preceding item being the key
                }

                ',' => {
                    //after every member definition and between items in arrays
                }

                _ => {
                    //white space or some value we need that holds actual information

                    if input_char.is_ascii_whitespace() {
                        continue;
                    }
                    if input_char.is_numeric() {
                        let (json_number, offset) = Self::read_number(&input_data[i..]);
                        let json_value = JsonValue::Number(json_number);
                    }
                }
            }
        }

        parsed_json
    }

    fn read_number(input_data: &[u8]) -> (f32, usize) {
        //increase outside index after reading cells here
        //  let mut offset : usize = 0;
        todo!()
    }

    fn read_str(input_data: &[u8]) -> (String, usize) {
        ("".to_owned(), 5)
        //we want to increase the outside index to account for reading cells here
        /* let mut offset : usize = 0;

         let mut string = "".to_owned();
         let mut last_character_backslash = false;
         while offset < input_data.len() {
             offset += 1;

             let input_char = input_data[offset] as char;
             match input_char {
                 '\\' => {
                     if last_character_backslash {
                         last_character_backslash = false;
                         string.push(input_char);
                     }else{
                         last_character_backslash = true;
                     }
                 },
                 '"' => {
                    if last_character_backslash {
                        string.push('"');
                         last_character_backslash = false;
                     }else{
                         break;
                     }
                 }
                 _ => {
                     if last_character_backslash {
                         last_character_backslash = false;
                         match input_char {
                            '/' => {
                             string.push(input_char);
                            },
                            'b' => {
                                 string.push('\x08');
                            },
                            'f' => {
                             string.push('\x0c');
                            },
                            'n' => {
                             string.push('\n');
                            },
                            'r' => {
                             string.push('\r');
                            }
                            't' => {
                                string.push('\t')
                            },
                            'u' => {
                                 if offset + 4 < input_data.len() {
                                     string.push(Json::hex_to_dec(&input_data[offset..offset+4]) as char);
                                     offset += 4;
                                 }
                            }
                         }
                     }else{
                         string.push(input_char);
                     }
                 }
             }

         }
         (string.to_owned(), offset)
        */
    }

    pub fn hex_to_dec_32_bit(input_data: &[u8]) -> Option<u32> {
        let mut total: u32 = 0;
        let input_size = input_data.len();
        for i in 0..input_data.len() {
            println!("========Loop Start========");
            let mut input_data = input_data[i];
            println!("Input Data:  {input_data}  ||  {}", {input_data as char});
            let offset_correction = {
                //ASCII char 0 starts at 30 in decimal
                if (input_data as char).is_numeric() {
                    48_u8
                } else if input_data.is_ascii_alphabetic() {
                    if (input_data as char).is_uppercase() {
                        //ASCII uppercase range starts at 65, but we want that to
                        //be in the range of A-F (10-15)
                        65 - 10
                    } else {
                        //ASCII lowercase range starts at 97
                        97 - 10
                    }
                } else {
                    return None;
                }
            };
            let power =  input_size as u32  - 1 - i as u32;
            println!("let power = input_size({input_size}) - 1 - i({i}); //power = {power}");
            input_data = input_data - offset_correction;
            println!("input_data = input_data({input_data}) - offset_correction({offset_correction}); // input_data = {input_data}");

            total += input_data as u32 * 16_u32.pow(power);
            println!("total += input_data({input_data}) * 16_u32^power({});//total = {total}", {16_u32.pow(power)});
            println!("========Loop End========");
        }
        Some(total)

   }

    /*
           #[cfg(test)]
           fn read_str_can_parse_strings(){
           let characters="\"Hello World!\"".as_bytes();
           assert_eq!(Json::read_str(&characters), (String::from_utf8_lossy(&characters).into_owned(), characters.len()))
           }

    */
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn test_hex_to_dec_works_with_numeric_types() {
        assert_eq!(super::Json::hex_to_dec_32_bit("35".as_bytes()), Some(53));
    }

    #[test]
    pub fn test_htd_works_with_capital_hex_values() {
        assert_eq!(super::Json::hex_to_dec_32_bit("D3".as_bytes()), Some(211));
        assert_eq!(super::Json::hex_to_dec_32_bit("FF".as_bytes()), Some(255));
        assert_eq!(super::Json::hex_to_dec_32_bit("E621".as_bytes()), Some(58913));
    }
}
