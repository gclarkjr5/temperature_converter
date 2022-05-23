use f_to_c::temperature::convert;
use f_to_c::temperature::convert::Temperature;

fn main() -> Result<(), &'static str> {

    // get the original temp type from the user (f or c)
    let from_temp_type = convert::read_input("What temperature do you want to convert from? Type 'c' or 'f'.");
    let tt = convert::parse_temp_type(from_temp_type)?;

    // get the original temp value of the above temp type
    let from_temp_value = convert::read_input("What value temperature do you want to convert?");
    let tv = convert::parse_temp_value(from_temp_value)?;

    // construct the original temp
    let from_temp = Temperature {
        temp_type: tt,
        temp_value: tv
    };

    // convert
    let to_temp = Temperature::convert(&from_temp);

    println!("{} in {:?} is equal to {} in {:?}", from_temp.temp_value, from_temp.temp_type, to_temp.temp_value, to_temp.temp_type);

    Ok(())
}