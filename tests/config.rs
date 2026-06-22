use rust_hub75_matrix::{Error, MatrixConfig};

#[test]
fn default_configuration_is_valid() {
    assert!(MatrixConfig::default().validate().is_ok());
}

#[test]
fn zero_panel_rows_are_rejected() {
    let mut config = MatrixConfig::default();
    config.rows = 0;
    assert!(matches!(config.validate(), Err(Error::InvalidConfig(_))));
}

#[test]
fn brightness_above_100_is_rejected() {
    let mut config = MatrixConfig::default();
    config.brightness = 101;
    assert!(matches!(
        config.validate(),
        Err(Error::InvalidBrightness(101))
    ));
}
