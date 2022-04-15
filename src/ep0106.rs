use rppal::i2c::I2c;

use crate::error::{Result, SensorHubError};

// some registers listed here are not use
const DEVICE_BUS: u8 = 1;
const DEVICE_ADDR: u16 = 0x17;
const TEMP_REG: u8 = 0x01;
const LIGHT_REG_L: u8 = 0x02;
// const LIGHT_REG_H: u8 = 0x03;
const STATUS_REG: u8 = 0x04;
const ON_BOARD_TEMP_REG: u8 = 0x05;
const ON_BOARD_HUMIDITY_REG: u8 = 0x06;
// const ON_BOARD_SENSOR_ERROR: u8 = 0x07;
const BMP280_TEMP_REG: u8 = 0x08;
const BMP280_PRESSURE_REG_L: u8 = 0x09;
// const BMP280_PRESSURE_REG_M: u8 = 0x0A;
// const BMP280_PRESSURE_REG_H: u8 = 0x0B;
const BMP280_STATUS: u8 = 0x0C;
const HUMAN_DETECT: u8 = 0x0D;

pub struct Ep0106 {
    i2c: I2c,
}

impl Ep0106 {
    /// creates a new Connection to i2c and sets the device address
    pub fn new() -> Result<Self> {
        let mut i2c = I2c::with_bus(DEVICE_BUS)?;
        let _ = i2c.set_slave_address(DEVICE_ADDR)?;
        Ok(Self { i2c })
    }

    /// trys to read from the external temperature sensor
    /// if the sensors is not connected the returns an `SensorHubError`
    ///
    /// Thermistor Detection Temperature Range -30℃~127℃
    pub fn ext_temp(&self) -> Result<i8> {
        // read status
        let mut status_buffer = [0u8; 1];
        let _ = self.i2c.block_read(STATUS_REG, &mut status_buffer)?;
        let status_reg = status_buffer[0];

        if (status_reg & 0x01) == 1 {
            Err(SensorHubError::ExternalTemperatureOverflow)
        } else if (status_reg & 0x02) == 2 {
            Err(SensorHubError::ExternalTemperatureNotFound)
        } else {
            let mut buffer = [0u8; 1];
            let _ = self.i2c.block_read(TEMP_REG, &mut buffer)?;
            Ok(buffer[0] as i8)
        }
    }

    /// reads the light intensity in lux
    ///
    /// detection range 0Lux~1800Lux
    pub fn brightness(&self) -> Result<u16> {
        // read status
        let mut status_buffer = [0u8; 1];
        let _ = self.i2c.block_read(STATUS_REG, &mut status_buffer)?;
        let status_reg = status_buffer[0];

        if (status_reg & 0x04) == 4 {
            Err(SensorHubError::BrightnessOverflow)
        } else if (status_reg & 0x08) == 8 {
            Err(SensorHubError::BrightnessNotFound)
        } else {
            let mut buffer = [0u8; 2];
            let _ = self.i2c.block_read(LIGHT_REG_L, &mut buffer)?;

            let mut light: u16 = (buffer[1] as u16) << 8;
            light |= buffer[0] as u16;

            if light >= 1800 {
                return Err(SensorHubError::BrightnessOverflow);
            }

            Ok(light)
        }
    }

    /// reads the temperature from the on board sensor
    ///
    /// detection range: DHT11 -20℃~60℃
    pub fn on_board_temp(&self) -> Result<i8> {
        let mut buffer = [0u8; 1];
        let _ = self.i2c.block_read(ON_BOARD_TEMP_REG, &mut buffer)?;
        let temp = buffer[0] as i8;

        if temp >= 60 {
            return Err(SensorHubError::ExternalTemperatureOverflow);
        }

        Ok(temp)
    }

    /// reads the humidity
    ///
    /// sensors supports humidity between 20% Rh ~ 95% Rh
    pub fn on_board_humidity(&self) -> Result<u8> {
        let mut buffer = [0u8; 1];
        let _ = self.i2c.block_read(ON_BOARD_HUMIDITY_REG, &mut buffer)?;
        Ok(buffer[0])
    }

    /// reads the temperature on the bmp280 sensor
    ///
    /// detection range: -40℃~80℃.
    pub fn bmp280_temp(&self) -> Result<i8> {
        let mut bmp_status_buffer = [0u8; 1];
        let _ = self.i2c.block_read(BMP280_STATUS, &mut bmp_status_buffer)?;
        if bmp_status_buffer[0] != 0 {
            return Err(SensorHubError::BarometerValueNotValid);
        }

        let mut buffer = [0u8; 1];
        let _ = self.i2c.block_read(BMP280_TEMP_REG, &mut buffer)?;
        let temp = buffer[0] as i8;

        if temp >= 80 {
            return Err(SensorHubError::ExternalTemperatureOverflow);
        }

        Ok(temp)
    }

    /// reads the air pressure in pascal on the bmp280 sensor
    ///
    /// detection range: 300 Pa ~ 1100 hPa (110000 Pa)
    pub fn bmp280_air_pressure(&self) -> Result<u32> {
        let mut bmp_status_buffer = [0u8; 1];
        let _ = self.i2c.block_read(BMP280_STATUS, &mut bmp_status_buffer)?;
        if bmp_status_buffer[0] != 0 {
            return Err(SensorHubError::BarometerValueNotValid);
        }

        let mut buffer = [0u8; 4];
        let _ = self.i2c.block_read(BMP280_PRESSURE_REG_L, &mut buffer)?;
        let mut bmp_pressure: u32 = (buffer[2] as u32) << 16;
        bmp_pressure |= (buffer[1] as u32) << 8;
        bmp_pressure |= buffer[0] as u32;

        if bmp_pressure >= 110000 {
            return Err(SensorHubError::BarometerValueNotValid);
        }

        Ok(bmp_pressure)
    }

    /// reads the thermal infrared sensor
    pub fn human_detected(&self) -> Result<bool> {
        let mut buffer = [0u8; 1];
        let _ = self.i2c.block_read(HUMAN_DETECT, &mut buffer)?;
        Ok(buffer[0] == 1)
    }
}
