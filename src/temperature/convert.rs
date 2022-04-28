#[cfg(test)]
#[path="test_convert.rs"]
mod test_convert;

#[derive(Debug)]
pub enum TemperatureType {
    Celsius,
    Fahrenheit
}

#[derive(Debug)]
pub struct Temperature {
    pub temp_type: TemperatureType,
    pub temp_value: f32
}

impl Temperature {
    pub fn convert(&self) -> Temperature {
        match self.temp_type {
            TemperatureType::Celsius => Temperature {
                temp_type: TemperatureType::Fahrenheit,
                temp_value: celsius_to_fahrenheit(self.temp_value)
            },
            TemperatureType::Fahrenheit => Temperature {
                temp_type: TemperatureType::Celsius,
                temp_value: fahrenheit_to_celsius(self.temp_value)
            }
        }
    }
}

fn celsius_to_fahrenheit(val: f32) -> f32 {
    (val * 9.0/5.0) + 32.0
}

fn fahrenheit_to_celsius(val: f32) -> f32 {
    (val - 32.0) * (5.0/9.0)
}

pub fn read_input(message: &'static str) -> String {

    println!("{}", message);

    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()

}

pub fn parse_temp_type(temp_type: String) -> Result<TemperatureType, &'static str> {

    match temp_type.as_str() {
        "c" => Ok(TemperatureType::Celsius),
        "f" => Ok(TemperatureType::Fahrenheit),
        _ => Err("Invalid temperature type. Only 'f' or 'c' will work.")
    }

}

pub fn parse_temp_value(temp_value: String) -> Result<f32, &'static str> {

    match temp_value.parse::<f32>() {
        Ok(input) => Ok(input),
        Err(_) => Err("Error parsing the temperature value. Make sure the value entered is a valid float.")
    }

}