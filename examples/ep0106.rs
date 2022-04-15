use sensor_hub::{ep0106::Ep0106, error::SensorHubError};

fn main() -> Result<(), SensorHubError> {
    let sensor_hub = Ep0106::new()?;

    match sensor_hub.ext_temp() {
        Ok(temp) => println!("Current external Sensor Temperature = {temp} Celsius"),
        Err(e) => eprintln!("{e}"),
    }

    match sensor_hub.brightness() {
        Ok(brightness) => println!("Current onboard sensor brightness = {brightness} Lux"),
        Err(e) => eprintln!("{e}"),
    }

    match sensor_hub.on_board_temp() {
        Ok(temp) => println!("Current onboard sensor temperature = {temp} Celsius",),
        Err(e) => eprintln!("{e}"),
    }

    match sensor_hub.on_board_humidity() {
        Ok(humidity) => println!("Current onboard sensor humidity = {humidity}%"),
        Err(e) => eprintln!("{e}"),
    }

    match sensor_hub.bmp280_temp() {
        Ok(temp) => println!("Current barometer temperature = {temp} Celsius"),
        Err(e) => eprintln!("{e}"),
    }

    match sensor_hub.bmp280_air_pressure() {
        Ok(pressure) => println!("Current barometer pressure = {pressure} Pascal"),
        Err(e) => eprintln!("{e}"),
    }

    match sensor_hub.human_detected() {
        Ok(true) => println!("Live body detected within 5 seconds!"),
        Ok(false) => println!("No humans detected!"),
        Err(e) => eprintln!("{e}"),
    }

    Ok(())
}
