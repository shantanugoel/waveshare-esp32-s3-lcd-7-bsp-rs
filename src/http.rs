use esp_idf_svc::http::client::{Configuration, EspHttpConnection};
use embedded_svc::http::client::{Client, Method};

pub struct HttpClient {
    client: Client<EspHttpConnection>,
}

#[derive(thiserror::Error, Debug)]
pub enum HttpClientError {
    #[error("Failed to initialize HTTP client")]
    Init,
    #[error("Failed to create request")]
    Request,
    #[error("Failed to submit request")]
    RequestSubmit,
}

impl HttpClient {
    pub fn new() -> Result<Self, HttpClientError> {
        let config = Configuration {
            use_global_ca_store: true,
            crt_bundle_attach: Some(esp_idf_svc::sys::esp_crt_bundle_attach),
            ..Default::default()
        };
        let client = EspHttpConnection::new(&config).map_err(|_| HttpClientError::Init)?;
        Ok(HttpClient { client: Client::wrap(client) })
    }

    pub fn get(&mut self, url: &str, headers: &[(&str, &str)]) -> Result<String, HttpClientError> {
        let request = self.client.request(Method::Get, url.as_ref(), headers).map_err(|_| HttpClientError::Request)?;
        let mut response = request.submit().map_err(|_| HttpClientError::RequestSubmit)?;
        let mut body = String::new();
        let mut buffer = [0; 1024];
        while let Ok(bytes_read) = response.read(&mut buffer) {
            if bytes_read == 0 {
                break;
            }
            body.push_str(&String::from_utf8_lossy(&buffer[..bytes_read]));
        }
        Ok(body)
    }
}