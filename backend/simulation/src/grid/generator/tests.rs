use crate::grid::generator::Generator;
use crate::grid::{ToJson, Voltage};

#[test]
fn generator_to_json_after_initialising() {
    let generator = Generator {
        id: 0,
        voltage: Voltage(0.0, 0.0, 0.0),
        max_voltage: 0.0,
        frequency: 0.0,
        transmission_line: 0,
    };

    assert_eq!(generator.to_json(),"{\"Connected Transmission Line\":0,\"Frequency\":0.0,\"ID\":0,\"Max Voltage\":0.0,\"Voltage\":{\"Phase 1\":0.0,\"Phase 2\":0.0,\"Phase 3\":0.0}}")
}
#[test]
fn generator_to_json_after_setting_voltage() {
    let mut generator = Generator {
        id: 0,
        voltage: Voltage(0.0, 0.0, 0.0),
        max_voltage: 0.0,
        frequency: 0.0,
        transmission_line: 0,
    };
    generator.voltage = Voltage(230.0, 105.0, -80.0);
    assert_eq!(generator.to_json(),"{\"Connected Transmission Line\":0,\"Frequency\":0.0,\"ID\":0,\"Max Voltage\":0.0,\"Voltage\":{\"Phase 1\":230.0,\"Phase 2\":105.0,\"Phase 3\":-80.0}}")
}
#[test]
fn generator_to_json_after_setting_max_voltage() {
    let mut generator = Generator {
        id: 0,
        voltage: Voltage(0.0, 0.0, 0.0),
        max_voltage: 0.0,
        frequency: 0.0,
        transmission_line: 0,
    };
    generator.max_voltage = 240.0;
    assert_eq!(generator.to_json(),"{\"Connected Transmission Line\":0,\"Frequency\":0.0,\"ID\":0,\"Max Voltage\":240.0,\"Voltage\":{\"Phase 1\":0.0,\"Phase 2\":0.0,\"Phase 3\":0.0}}")
}
#[test]
fn generator_to_json_after_setting_max_frequency() {
    let mut generator = Generator {
        id: 0,
        voltage: Voltage(0.0, 0.0, 0.0),
        max_voltage: 0.0,
        frequency: 0.0,
        transmission_line: 0,
    };
    generator.frequency = 50.0;
    assert_eq!(generator.to_json(),"{\"Connected Transmission Line\":0,\"Frequency\":50.0,\"ID\":0,\"Max Voltage\":0.0,\"Voltage\":{\"Phase 1\":0.0,\"Phase 2\":0.0,\"Phase 3\":0.0}}")
}
#[test]
fn generator_to_json_after_setting_max_transmission_line() {
    let mut generator = Generator {
        id: 0,
        voltage: Voltage(0.0, 0.0, 0.0),
        max_voltage: 0.0,
        frequency: 0.0,
        transmission_line: 0,
    };
    generator.transmission_line = 5;
    assert_eq!(generator.to_json(),"{\"Connected Transmission Line\":5,\"Frequency\":0.0,\"ID\":0,\"Max Voltage\":0.0,\"Voltage\":{\"Phase 1\":0.0,\"Phase 2\":0.0,\"Phase 3\":0.0}}")
}
