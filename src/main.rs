use clap::Parser;

use f_to_c::temperature::convert;
use f_to_c::temperature::convert::Temperature;

/// Simple temperature converter between celsius and fahrenheit
#[derive(Parser)]
#[clap(author, about, long_about = None)]
struct Args {
    /// Temperature type to be converted
    #[clap(arg_enum, short, long)]
    r#type: convert::TemperatureType,

    /// Value of temp to be converted
    #[clap(short, long)]
    value: f32,
}

fn main() {

    // parse the arguments
    let args = Args::parse();

    // construct the original temp
    let from_temp = Temperature {
        temp_type: args.r#type,
        temp_value: args.value
    };

    // convert
    let to_temp = Temperature::convert(&from_temp);

    println!("{} in {:?} is equal to {} in {:?}", from_temp.temp_value, from_temp.temp_type, to_temp.temp_value, to_temp.temp_type)

}