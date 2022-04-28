
use super::*;

#[test]
fn f_to_c() {

    let temp = Temperature {
        temp_type: TemperatureType::Fahrenheit,
        temp_value: 41.0
    };

    assert_eq!(Temperature::convert(&temp).temp_value, 5.0);
}

#[test]
fn c_to_f() {

    let temp = Temperature {
        temp_type: TemperatureType::Celsius,
        temp_value: 5.0
    };

    assert_eq!(Temperature::convert(&temp).temp_value, 41.0);
}