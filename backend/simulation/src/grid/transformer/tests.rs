use crate::grid::transformer::Transformer;
use crate::grid::ToJson;

#[test]
fn transformer_to_json_after_initialising() {
    let transformer = Transformer {
        id: 0,
        ratio: 0.0,
        primary: 0,
        secondary: 0,
    };
    assert_eq!(transformer.to_json(),"{\"ID\":0,\"Primary Transmission Line\":0,\"Ratio\":0.0,\"Secondary Transmission Line\":0}")
}

#[test]
fn transformer_to_json_after_setting_ratio() {
    let mut transformer = Transformer {
        id: 0,
        ratio: 0.0,
        primary: 0,
        secondary: 0,
    };
    transformer.ratio = 15.0;
    assert_eq!(transformer.to_json(),"{\"ID\":0,\"Primary Transmission Line\":0,\"Ratio\":15.0,\"Secondary Transmission Line\":0}")
}

#[test]
fn transformer_to_json_after_setting_primary() {
    let mut transformer = Transformer {
        id: 0,
        ratio: 0.0,
        primary: 0,
        secondary: 0,
    };
    transformer.primary = 5;
    assert_eq!(transformer.to_json(),"{\"ID\":0,\"Primary Transmission Line\":5,\"Ratio\":0.0,\"Secondary Transmission Line\":0}")
}

#[test]
fn transformer_to_json_after_setting_secondary() {
    let mut transformer = Transformer {
        id: 0,
        ratio: 0.0,
        primary: 0,
        secondary: 0,
    };
    transformer.secondary = 5;
    assert_eq!(transformer.to_json(),"{\"ID\":0,\"Primary Transmission Line\":0,\"Ratio\":0.0,\"Secondary Transmission Line\":5}")
}
