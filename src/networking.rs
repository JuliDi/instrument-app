//! Provides a [`DeviceConnection`] and utility functions for networking and communication

use std::{
    net::{TcpStream, SocketAddr},
    str::FromStr,
    io::{Write, BufReader, BufRead},
    time::Duration};

/// Wrapper for an [`Option<TcpStream>`] that is shared across the application
#[derive(Default)]
pub struct DeviceConnection {
    stream: Option<TcpStream>,
}

impl DeviceConnection {
    /// Connect to a device listening on `address`
    pub fn connect(&mut self, address: &SocketAddr) -> Result<(), std::io::Error> {
        // Attempt connection
        match TcpStream::connect_timeout(address, Duration::from_secs(1)) {
            Ok(stream) => {
                stream.set_read_timeout(Some(Duration::from_secs(1))).expect("Could not set a read timeout");
                self.stream = Some(stream);
                Ok(())
            }
            Err(e) => {
                self.stream = None;
                Err(e)
            }
        }
    }

    /// Return if the [`DeviceConnection`] is connected to a device
    #[allow(dead_code)]
    pub fn connected(&self) -> bool {
        self.stream.is_some()
    }

    /// Send a `&[u8]` slice to the connected device
    pub fn send(&self, data: &[u8]) -> std::io::Result<usize> {
        self.stream.as_ref().ok_or(std::io::ErrorKind::NotConnected)?.write(data)
    }

    /// Receives a `u8` buffer from the connected device and reads it to a string
    pub fn receive(&mut self) -> std::io::Result<String> {
        if let Some(stream) = self.stream.as_mut() {
            let mut data = String::new();
            let mut reader = BufReader::new(stream);
            reader.read_line(&mut data).unwrap_or(0);
            Ok(data)
        } else {
            Err(std::io::Error::from(std::io::ErrorKind::NotConnected))
        }
    }

    /// Get the connected peer's address or a `NotConnected` error, if there is no connection established
    #[allow(dead_code)]
    pub fn get_peer_address(&self) -> std::io::Result<SocketAddr> {
        self.stream.as_ref().ok_or(std::io::ErrorKind::NotConnected)?.peer_addr()
    }
}

/// Attempt to parse an `IP:port` string into a [`SocketAddr`]
pub fn parse_ip(address: &String) -> Result<SocketAddr, String> {
    SocketAddr::from_str(&address).map_err(|_x| { "Invalid IP Address".to_string() })
}