use crate::grid::generator::Generator;
use crate::grid::load::Connection::{Parallel, Series};
use crate::grid::load::{Consumer, Load, LoadType};
use crate::grid::location::Location;
use crate::grid::transformer::Transformer;
use crate::grid::{
    Circuit, Current, CurrentWrapper, OscilloscopeDetail, Resistance, Voltage, VoltageWrapper,
};
use std::sync::Arc;
use std::sync::Mutex;

fn create_sample_circuit() -> Circuit {
    Circuit {
        id: 0,
        loads: vec![
            Load {
                load_type: LoadType::Consumer(Consumer {
                    id: 0,
                    resistance: Resistance(10.0),
                    voltage: VoltageWrapper {
                        voltage: Voltage(0.0, 0.0, 0.0),
                        oscilloscope_detail: OscilloscopeDetail {
                            frequency: 0.0,
                            amplitude: 0.0,
                            phase: 0.0,
                        },
                    },
                    location: Location {
                        latitude: 0.0,
                        longitude: 0.0,
                    },
                }),
                id: 0,
            },
            Load {
                load_type: LoadType::Consumer(Consumer {
                    id: 1,
                    resistance: Resistance(15.0),
                    voltage: VoltageWrapper {
                        voltage: Voltage(0.0, 0.0, 0.0),
                        oscilloscope_detail: OscilloscopeDetail {
                            frequency: 0.0,
                            amplitude: 0.0,
                            phase: 0.0,
                        },
                    },
                    location: Location {
                        latitude: 0.0,
                        longitude: 0.0,
                    },
                }),
                id: 1,
            },
            Load {
                load_type: LoadType::Consumer(Consumer {
                    id: 2,
                    resistance: Resistance(30.0),
                    voltage: VoltageWrapper {
                        voltage: Voltage(0.0, 0.0, 0.0),
                        oscilloscope_detail: OscilloscopeDetail {
                            frequency: 0.0,
                            amplitude: 0.0,
                            phase: 0.0,
                        },
                    },
                    location: Location {
                        latitude: 0.0,
                        longitude: 0.0,
                    },
                }),
                id: 2,
            },
            Load {
                load_type: LoadType::Consumer(Consumer {
                    id: 3,
                    resistance: Resistance(10.0),
                    voltage: VoltageWrapper {
                        voltage: Voltage(0.0, 0.0, 0.0),
                        oscilloscope_detail: OscilloscopeDetail {
                            frequency: 0.0,
                            amplitude: 0.0,
                            phase: 0.0,
                        },
                    },
                    location: Location {
                        latitude: 0.0,
                        longitude: 0.0,
                    },
                }),
                id: 3,
            },
            Load {
                load_type: LoadType::Consumer(Consumer {
                    id: 4,
                    resistance: Resistance(18.0),
                    voltage: VoltageWrapper {
                        voltage: Voltage(0.0, 0.0, 0.0),
                        oscilloscope_detail: OscilloscopeDetail {
                            frequency: 0.0,
                            amplitude: 0.0,
                            phase: 0.0,
                        },
                    },
                    location: Location {
                        latitude: 0.0,
                        longitude: 0.0,
                    },
                }),
                id: 4,
            },
            Load {
                load_type: LoadType::Consumer(Consumer {
                    id: 5,
                    resistance: Resistance(15.0),
                    voltage: VoltageWrapper {
                        voltage: Voltage(0.0, 0.0, 0.0),
                        oscilloscope_detail: OscilloscopeDetail {
                            frequency: 0.0,
                            amplitude: 0.0,
                            phase: 0.0,
                        },
                    },
                    location: Location {
                        latitude: 0.0,
                        longitude: 0.0,
                    },
                }),
                id: 5,
            },
            Load {
                load_type: LoadType::Consumer(Consumer {
                    id: 6,
                    resistance: Resistance(17.0),
                    voltage: VoltageWrapper {
                        voltage: Voltage(0.0, 0.0, 0.0),
                        oscilloscope_detail: OscilloscopeDetail {
                            frequency: 0.0,
                            amplitude: 0.0,
                            phase: 0.0,
                        },
                    },
                    location: Location {
                        latitude: 0.0,
                        longitude: 0.0,
                    },
                }),
                id: 6,
            },
            Load {
                load_type: LoadType::Consumer(Consumer {
                    id: 7,
                    resistance: Resistance(25.0),
                    voltage: VoltageWrapper {
                        voltage: Voltage(0.0, 0.0, 0.0),
                        oscilloscope_detail: OscilloscopeDetail {
                            frequency: 0.0,
                            amplitude: 0.0,
                            phase: 0.0,
                        },
                    },
                    location: Location {
                        latitude: 0.0,
                        longitude: 0.0,
                    },
                }),
                id: 7,
            },
            Load {
                load_type: LoadType::Consumer(Consumer {
                    id: 8,
                    resistance: Resistance(10.0),
                    voltage: VoltageWrapper {
                        voltage: Voltage(0.0, 0.0, 0.0),
                        oscilloscope_detail: OscilloscopeDetail {
                            frequency: 0.0,
                            amplitude: 0.0,
                            phase: 0.0,
                        },
                    },
                    location: Location {
                        latitude: 0.0,
                        longitude: 0.0,
                    },
                }),
                id: 8,
            },
            Load {
                load_type: LoadType::Consumer(Consumer {
                    id: 9,
                    resistance: Resistance(11.0),
                    voltage: VoltageWrapper {
                        voltage: Voltage(0.0, 0.0, 0.0),
                        oscilloscope_detail: OscilloscopeDetail {
                            frequency: 0.0,
                            amplitude: 0.0,
                            phase: 0.0,
                        },
                    },
                    location: Location {
                        latitude: 0.0,
                        longitude: 0.0,
                    },
                }),
                id: 9,
            },
        ],
        connections: vec![
            Parallel(0, 1),
            Series(0, 2),
            Series(0, 3),
            Parallel(0, 4),
            Series(4, 5),
            Parallel(0, 7),
            Series(7, 8),
            Series(7, 9),
        ],
        generators: vec![Generator {
            id: 0,
            voltage: VoltageWrapper {
                voltage: Voltage(0.0, 0.0, 0.0),
                oscilloscope_detail: OscilloscopeDetail {
                    frequency: 0.0,
                    amplitude: 0.0,
                    phase: 0.0,
                },
            },
            max_voltage: 240.0,
            frequency: 50.0,
            transmission_line: 0,
            location: Location {
                latitude: 0.0,
                longitude: 0.0,
            },
        }],
        transformers: vec![],
    }
}

#[test]
//Result tested by theoretical result determined by hand(with assistance of Casio fx-991ZA Plus)
fn calculate_equivalent_impedance_when_inductance_and_capacitance_zero() {
    let mut circuit = create_sample_circuit();
    let result = circuit.calculate_equivalent_impedance(50.0, 0);
    assert_eq!(result, 44.572_29)
}

#[test]
//Tested against result determined by desmos online graphing calculator
fn calculate_ideal_generator_voltages_at_time_zero() {
    let mut circuit = create_sample_circuit();
    let result = circuit.calculate_ideal_generator_voltages(0.0);
    assert_eq!(result.voltage.0, 0.0);
    assert_eq!(result.voltage.1, -207.8461);
    assert_eq!(result.voltage.2, 207.8461);
}

#[test]
//Result vetted against a simulink simulation
fn set_voltages_when_inductance_and_capacitance_zero() {
    let mut circuit = create_sample_circuit();
    let voltage = circuit.calculate_ideal_generator_voltages(0.0);
    let impedance = Resistance(circuit.calculate_equivalent_impedance(50.0, 0));
    let current = CurrentWrapper::ohms_law(voltage, impedance);
    circuit.set_voltages(current, 50.0, 0);

    assert_eq!(circuit.loads[3].get_voltage().voltage.0, 0.0);
    assert_eq!(circuit.loads[3].get_voltage().voltage.1, -46.631237);
    assert_eq!(circuit.loads[3].get_voltage().voltage.2, 46.631237);
}

#[test]
fn set_transformers_secondary_voltages_when_inductance_and_capacitance_zero() {
    let mut circuit1 = create_sample_circuit();
    let mut circuit2 = create_sample_circuit();
    circuit2.id = 1;
    let transformer = Transformer {
        id: 0,
        ratio: 1.0,
        primary_circuit: 0,
        secondary_circuit: 1,
        primary_load: 0,
        secondary_voltage: VoltageWrapper {
            voltage: Voltage(0.0, 0.0, 0.0),
            oscilloscope_detail: OscilloscopeDetail {
                frequency: 0.0,
                amplitude: 0.0,
                phase: 0.0,
            },
        },
        location: Location {
            latitude: 0.0,
            longitude: 0.0,
        },
        target: None,
    };

    let trans_ref = Arc::new(Mutex::new(transformer));
    circuit1.transformers.push(trans_ref.clone());
    circuit2.transformers.push(trans_ref.clone());

    circuit1.loads[0].set_voltage(
        CurrentWrapper {
            current: Current(1.0, 1.0, 1.0),
            oscilloscope_detail: OscilloscopeDetail {
                frequency: 0.0,
                amplitude: 0.0,
                phase: 0.0,
            },
        },
        50.0,
    );
    circuit1.set_transformers_secondary_voltages(50.0);

    assert_eq!(
        circuit2.transformers[0]
            .lock()
            .unwrap()
            .secondary_voltage
            .voltage
            .0,
        10.0
    );
}
