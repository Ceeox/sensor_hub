use rppal::i2c::Error as I2cError;
use thiserror::Error;

pub type Result<T> = ::std::result::Result<T, SensorHubError>;

#[derive(Error, Debug)]
pub enum SensorHubError {
    #[error("i2c error: {0}")]
    I2c(#[from] I2cError),

    #[error("external temperature overflow")]
    ExternalTemperatureOverflow,

    #[error("external temperature sensor is not connected")]
    ExternalTemperatureNotFound,

    #[error("Brightness Overflow")]
    BrightnessOverflow,

    #[error("Brightness Not Found")]
    BrightnessNotFound,

    #[error("onboard temperature and humidity sensor data may not be up to date")]
    NotUpToDate,

    #[error("bmp280 barometer retured a not valid value")]
    BarometerValueNotValid,
}
