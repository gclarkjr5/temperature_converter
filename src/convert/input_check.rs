pub fn parse_temp_type(temp_type: String) -> Result<String, &'static str> {

    match temp_type.as_str() {
        "c" => Ok(temp_type.to_string()),
        "f" => Ok(temp_type.to_string()),
        _ => Err("Invalid temperature type. Only 'f' or 'c' will work.")
    }

}

pub fn parse_temp_value(temp_value: String) -> Result<f32, &'static str> {

    match temp_value.parse::<f32>() {
        Ok(input) => Ok(input),
        Err(_) => Err("Error parsing the temperature value. Make sure the value entered is a valid float.")
    }

}