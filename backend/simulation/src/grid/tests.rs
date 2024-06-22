use crate::grid::{Grid, Resistance, ToJson, Voltage};
use crate::grid::consumer::Consumer;
use crate::grid::generator::Generator;
use crate::grid::transformer::Transformer;
use crate::grid::transmission_line::TransmissionLine;


#[test]
fn grid_to_json_after_initialising(){
    let grid =  Grid {
        consumers: vec![],
        transmission_lines: vec![],
        generators: vec![],
        transformers: vec![],
        started: false,
    };

    assert_eq!(grid.to_json(),"{\"Consumers\":[],\"Generators\":[],\"Started\":false,\"Transformers\":[],\"Transmission Lines\":[]}")
}


#[test]
fn grid_to_json_after_setting_consumers(){
    let mut grid =  Grid {
        consumers: vec![],
        transmission_lines: vec![],
        generators: vec![],
        transformers: vec![],
        started: false,
    };

    grid.consumers.push(Consumer {
        id: 0,
        resistance: Resistance(0.0),
        transmission_line: 0,
        voltage: Voltage(0.0,0.0,0.0),
    });

    grid.consumers.push(Consumer {
        id: 0,
        resistance: Resistance(0.0),
        transmission_line: 0,
        voltage: Voltage(0.0,0.0,0.0),
    });

    assert_eq!(grid.to_json(),"{\"Consumers\":[\"{\\\"Connected Transmission Line\\\":0,\\\"ID\\\":0,\\\"Resistance\\\":0.0,\\\"Voltage\\\":{\\\"Phase 1\\\":0.0,\\\"Phase 2\\\":0.0,\\\"Phase 3\\\":0.0}}\",\"{\\\"Connected Transmission Line\\\":0,\\\"ID\\\":0,\\\"Resistance\\\":0.0,\\\"Voltage\\\":{\\\"Phase 1\\\":0.0,\\\"Phase 2\\\":0.0,\\\"Phase 3\\\":0.0}}\"],\"Generators\":[],\"Started\":false,\"Transformers\":[],\"Transmission Lines\":[]}")
}


#[test]
fn grid_to_json_after_setting_transmission_lines(){
    let mut grid =  Grid {
        consumers: vec![],
        transmission_lines: vec![],
        generators: vec![],
        transformers: vec![],
        started: false,
    };

    grid.transmission_lines.push(TransmissionLine {
        id: 0,
        resistance: Resistance(0.0),
        impedance: Resistance(0.0),
        voltage: Voltage(0.0, 0.0, 0.0),
    });

    grid.transmission_lines.push(TransmissionLine {
        id: 0,
        resistance: Resistance(0.0),
        impedance: Resistance(0.0),
        voltage: Voltage(0.0, 0.0, 0.0),
    });

    assert_eq!(grid.to_json(),"{\"Consumers\":[],\"Generators\":[],\"Started\":false,\"Transformers\":[],\"Transmission Lines\":[\"{\\\"ID\\\":0,\\\"Impedance\\\":0.0,\\\"Resistance\\\":0.0,\\\"Voltage\\\":{\\\"Phase 1\\\":0.0,\\\"Phase 2\\\":0.0,\\\"Phase 3\\\":0.0}}\",\"{\\\"ID\\\":0,\\\"Impedance\\\":0.0,\\\"Resistance\\\":0.0,\\\"Voltage\\\":{\\\"Phase 1\\\":0.0,\\\"Phase 2\\\":0.0,\\\"Phase 3\\\":0.0}}\"]}")
}

#[test]
fn grid_to_json_after_setting_generators() {
    let mut grid = Grid {
        consumers: vec![],
        transmission_lines: vec![],
        generators: vec![],
        transformers: vec![],
        started: false,
    };

    grid.generators.push(Generator {
        id: 0,
        voltage: Voltage(0.0,0.0,0.0),
        max_voltage: 0.0,
        frequency: 0.0,
        transmission_line: 0,
    });

    grid.generators.push(Generator {
        id: 0,
        voltage: Voltage(0.0,0.0,0.0),
        max_voltage: 0.0,
        frequency: 0.0,
        transmission_line: 0,
    });


    assert_eq!(grid.to_json(),"{\"Consumers\":[],\"Generators\":[\"{\\\"Connected Transmission Line\\\":0,\\\"Frequency\\\":0.0,\\\"ID\\\":0,\\\"Max Voltage\\\":0.0,\\\"Voltage\\\":{\\\"Phase 1\\\":0.0,\\\"Phase 2\\\":0.0,\\\"Phase 3\\\":0.0}}\",\"{\\\"Connected Transmission Line\\\":0,\\\"Frequency\\\":0.0,\\\"ID\\\":0,\\\"Max Voltage\\\":0.0,\\\"Voltage\\\":{\\\"Phase 1\\\":0.0,\\\"Phase 2\\\":0.0,\\\"Phase 3\\\":0.0}}\"],\"Started\":false,\"Transformers\":[],\"Transmission Lines\":[]}")
}

#[test]
fn grid_to_json_after_setting_transformers() {
    let mut grid = Grid {
        consumers: vec![],
        transmission_lines: vec![],
        generators: vec![],
        transformers: vec![],
        started: false,
    };

    grid.transformers.push(Transformer {
        id: 0,
        ratio: 0.0,
        primary: 0,
        secondary: 0,
    });

    grid.transformers.push(Transformer {
        id: 0,
        ratio: 0.0,
        primary: 0,
        secondary: 0,
    });

    assert_eq!(grid.to_json(),"{\"Consumers\":[],\"Generators\":[],\"Started\":false,\"Transformers\":[\"{\\\"ID\\\":0,\\\"Primary Transmission Line\\\":0,\\\"Ratio\\\":0.0,\\\"Secondary Transmission Line\\\":0}\",\"{\\\"ID\\\":0,\\\"Primary Transmission Line\\\":0,\\\"Ratio\\\":0.0,\\\"Secondary Transmission Line\\\":0}\"],\"Transmission Lines\":[]}")
}

#[test]
fn grid_to_json_after_setting_start(){
    let mut grid =  Grid {
        consumers: vec![],
        transmission_lines: vec![],
        generators: vec![],
        transformers: vec![],
        started: false,
    };
    grid.started = true;
    assert_eq!(grid.to_json(),"{\"Consumers\":[],\"Generators\":[],\"Started\":true,\"Transformers\":[],\"Transmission Lines\":[]}")
}

#[test]
fn grid_get_average_line_voltage() {
    let grid =  Grid {
        consumers: vec![],
        transmission_lines: vec![],
        generators: vec![],
        transformers: vec![],
        started: false,
    };

}
#[test]
fn grid_add_generator() {
    let grid =  Grid {
        consumers: vec![],
        transmission_lines: vec![],
        generators: vec![],
        transformers: vec![],
        started: false,
    };

}
#[test]
fn grid_add_consumer() {
    let grid =  Grid {
        consumers: vec![],
        transmission_lines: vec![],
        generators: vec![],
        transformers: vec![],
        started: false,
    };

}
#[test]
fn grid_update_generator(){
    let grid =  Grid {
        consumers: vec![],
        transmission_lines: vec![],
        generators: vec![],
        transformers: vec![],
        started: false,
    };

}
#[test]
fn grid_update_consumer(){
    let grid =  Grid {
        consumers: vec![],
        transmission_lines: vec![],
        generators: vec![],
        transformers: vec![],
        started: false,
    };

}

#[test]
fn grid_update_impedance() {
    let grid =  Grid {
        consumers: vec![],
        transmission_lines: vec![],
        generators: vec![],
        transformers: vec![],
        started: false,
    };

}
#[test]
fn grid_update_generator_voltages() {
    let grid =  Grid {
        consumers: vec![],
        transmission_lines: vec![],
        generators: vec![],
        transformers: vec![],
        started: false,
    };

}
#[test]
fn grid_sync_voltages() {
    let grid =  Grid {
        consumers: vec![],
        transmission_lines: vec![],
        generators: vec![],
        transformers: vec![],
        started: false,
    };

}

