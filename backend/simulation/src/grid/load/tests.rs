use crate::grid::load::Consumer;
use crate::grid::{Resistance, ToJson, Voltage};

#[test]
fn consumer_to_json_after_initialising() {
    let consumer = Consumer {
        id: 0,
        resistance: Resistance(0.0),
        transmission_line: 0,
        voltage: Voltage(0.0, 0.0, 0.0),
    };
    assert_eq!(consumer.to_json(),"{\"Connected Transmission Line\":0,\"ID\":0,\"Resistance\":0.0,\"Voltage\":{\"Phase 1\":0.0,\"Phase 2\":0.0,\"Phase 3\":0.0}}")
}

#[test]
fn consumer_to_json_after_setting_voltages() {
    let mut consumer = Consumer {
        id: 0,
        resistance: Resistance(0.0),
        transmission_line: 0,
        voltage: Voltage(0.0, 0.0, 0.0),
    };
    consumer.voltage = Voltage(230.0, 105.0, -80.0);

    assert_eq!(consumer.to_json(),"{\"Connected Transmission Line\":0,\"ID\":0,\"Resistance\":0.0,\"Voltage\":{\"Phase 1\":230.0,\"Phase 2\":105.0,\"Phase 3\":-80.0}}")
}

#[test]
fn consumer_to_json_after_setting_transmission_line() {
    let mut consumer = Consumer {
        id: 0,
        resistance: Resistance(0.0),
        transmission_line: 0,
        voltage: Voltage(0.0, 0.0, 0.0),
    };
    consumer.transmission_line = 5;
    assert_eq!(consumer.to_json(),"{\"Connected Transmission Line\":5,\"ID\":0,\"Resistance\":0.0,\"Voltage\":{\"Phase 1\":0.0,\"Phase 2\":0.0,\"Phase 3\":0.0}}")
}

#[test]
fn consumer_to_json_after_setting_resistance() {
    let mut consumer = Consumer {
        id: 0,
        resistance: Resistance(0.0),
        transmission_line: 0,
        voltage: Voltage(0.0, 0.0, 0.0),
    };
    consumer.resistance = Resistance(1000.0);
    assert_eq!(consumer.to_json(),"{\"Connected Transmission Line\":0,\"ID\":0,\"Resistance\":1000.0,\"Voltage\":{\"Phase 1\":0.0,\"Phase 2\":0.0,\"Phase 3\":0.0}}")
}
