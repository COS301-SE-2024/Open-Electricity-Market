use crate::grid::{Resistance, ToJson, Voltage};
use crate::grid::transmission_line::TransmissionLine;

#[test]
fn transmission_line_to_json_after_initialising(){
    let transmission_line = TransmissionLine {
        id: 0,
        resistance: Resistance(0.0),
        impedance: Resistance(0.0),
        voltage: Voltage(0.0,0.0,0.0),
    };
    assert_eq!(transmission_line.to_json(), "{\"ID\":0,\"Impedance\":0.0,\"Resistance\":0.0,\"Voltage\":{\"Phase 1\":0.0,\"Phase 2\":0.0,\"Phase 3\":0.0}}")
}

#[test]
fn transmission_line_to_json_after_setting_voltages(){
    let mut transmission_line = TransmissionLine {
        id: 0,
        resistance: Resistance(0.0),
        impedance: Resistance(0.0),
        voltage: Voltage(0.0,0.0,0.0),
    };
    transmission_line.voltage = Voltage(230.0,105.0,-80.0);
    assert_eq!(transmission_line.to_json(), "{\"ID\":0,\"Impedance\":0.0,\"Resistance\":0.0,\"Voltage\":{\"Phase 1\":230.0,\"Phase 2\":105.0,\"Phase 3\":-80.0}}")
}

#[test]
fn transmission_line_to_json_after_setting_resistance(){
    let mut transmission_line = TransmissionLine {
        id: 0,
        resistance: Resistance(0.0),
        impedance: Resistance(0.0),
        voltage: Voltage(0.0,0.0,0.0),
    };
    transmission_line.resistance = Resistance(1000.0);
    assert_eq!(transmission_line.to_json(), "{\"ID\":0,\"Impedance\":0.0,\"Resistance\":1000.0,\"Voltage\":{\"Phase 1\":0.0,\"Phase 2\":0.0,\"Phase 3\":0.0}}")
}

#[test]
fn transmission_line_to_json_after_setting_impedance(){
    let mut transmission_line = TransmissionLine {
        id: 0,
        resistance: Resistance(0.0),
        impedance: Resistance(0.0),
        voltage: Voltage(0.0,0.0,0.0),
    };
    transmission_line.impedance = Resistance(5000.0);
    assert_eq!(transmission_line.to_json(), "{\"ID\":0,\"Impedance\":5000.0,\"Resistance\":0.0,\"Voltage\":{\"Phase 1\":0.0,\"Phase 2\":0.0,\"Phase 3\":0.0}}")
}