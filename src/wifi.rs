use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::modem::Modem,
    nvs::EspDefaultNvsPartition,
    wifi::{ClientConfiguration, Configuration::Client, EspWifi},
};
use esp_idf_svc::hal::sys::EspError;
use std::{thread::sleep, time::Duration};

#[derive(Debug, thiserror::Error)]
pub enum WifiError {
    #[error("Failed to create wifi driver")]
    CreateWifiDriver(#[source] EspError),
    #[error("Failed to set wifi configuration")]
    SetWifiConfiguration(#[source] EspError),
    #[error("Failed to start wifi")]
    StartWifi(#[source] EspError),
    #[error("Invalid SSID string")]
    InvalidSsidString,
    #[error("Invalid password string")]
    InvalidPasswordString,
    #[error("Failed to connect wifi")]
    ConnectWifi(#[source] EspError),
    #[error("Failed to wait for wifi connection")]
    WaitNetifUp(#[source] EspError),
}

pub fn init<'a>(
    modem: Modem,
    event_loop: EspSystemEventLoop,
    nvs_partition: Option<EspDefaultNvsPartition>,
) -> Result<EspWifi<'a>, WifiError> {
    let wifi_driver = EspWifi::new(modem, event_loop, nvs_partition)
        .map_err(|e| WifiError::CreateWifiDriver(e))?;
    Ok(wifi_driver)
}

pub fn connect(wifi_driver: &mut EspWifi, ssid: &str, password: &str) -> Result<(), WifiError> {
    let client_config = ClientConfiguration {
        ssid: ssid.try_into().map_err(|_| WifiError::InvalidSsidString)?,
        password: password.try_into().map_err(|_| WifiError::InvalidPasswordString)?, 
        ..Default::default()
    };
    wifi_driver.set_configuration(&Client(client_config)).map_err(|e| WifiError::SetWifiConfiguration(e))?;
    wifi_driver.start().map_err(|e| WifiError::StartWifi(e))?;
    wifi_driver.connect().map_err(|e| WifiError::ConnectWifi(e))?;
    Ok(())
}

pub fn wait_for_connection(wifi_driver: &EspWifi) -> Result<(), WifiError> {
    while !wifi_driver.is_connected().map_err(|e| WifiError::WaitNetifUp(e))? {
        log::info!("Waiting for wifi connection...");
        sleep(Duration::from_millis(100));
    }
    Ok(())
}
