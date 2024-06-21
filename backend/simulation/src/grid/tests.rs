use crate::grid::{Grid, ToJson};


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

