mod input_check;

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

pub fn get_from_temp() -> Temperature {
    let from_temp_type = {
        let input = read_input("What temperature do you want to convert from? Type 'c' or 'f'.");

        let temp_type = input_check::parse_temp_type(input).unwrap();

        match temp_type.as_str() {
            "c" => TemperatureType::Celsius,
            "f" => TemperatureType::Fahrenheit,
            _ => panic!("Not possible")
        }
    };

    let from_value = {
        let input = read_input("What value temperature do you want to convert?");

        input_check::parse_temp_value(input).unwrap()
        
    };

    Temperature {
        temp_type: from_temp_type,
        temp_value: from_value
    }
}

pub fn convert_temperature(from_temp: &Temperature) -> Temperature {

    match from_temp.temp_type {
        TemperatureType::Celsius => Temperature {
            temp_type: TemperatureType::Fahrenheit,
            temp_value: celsius_to_fahrenheit(from_temp.temp_value)
        },
        TemperatureType::Fahrenheit => Temperature {
            temp_type: TemperatureType::Celsius,
            temp_value: fahrenheit_to_celsius(from_temp.temp_value)
        }
    }

}

fn celsius_to_fahrenheit(val: f32) -> f32 {
    (val * 9.0/5.0) + 32.0
}

fn fahrenheit_to_celsius(val: f32) -> f32 {
    (val - 32.0) * (5.0/9.0)
}

fn read_input(message: &'static str) -> String {

    println!("{}", message);

    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn f_to_c() {

        let temp = Temperature {
            temp_type: TemperatureType::Fahrenheit,
            temp_value: 41.0
        };

        assert_eq!(convert_temperature(&temp).temp_value, 5.0);
    }

    #[test]
    fn c_to_f() {

        let temp = Temperature {
            temp_type: TemperatureType::Celsius,
            temp_value: 5.0
        };

        assert_eq!(convert_temperature(&temp).temp_value, 41.0);
    }
}
