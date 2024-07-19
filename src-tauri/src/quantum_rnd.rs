use std::{io::{Read, Write}, time::Duration};
use anyhow::anyhow;
use serial::{open, windows::COMPort, BaudRate, SerialPort};
use std::sync::{Arc, Mutex};

pub enum RndMode {
  Real,
  Fake
}

pub struct QuantumRnd {
  port: Arc<Mutex<COMPort>>
}

impl QuantumRnd {
  pub fn new(port: &str) -> Result<Self, anyhow::Error> {
    let mut port = open(port)?;

    port.reconfigure(&|settings| {
      settings.set_baud_rate(BaudRate::from_speed(460800))?;
      Ok(())
    })?;

    port.set_timeout(Duration::from_millis(1000))?;


    Ok(Self { port: Arc::new(Mutex::new(port)) })
  }

  // pub fn set_port(&self, port: &str) -> Result<&Self, anyhow::Error> {
  //   let mut port = open(port)?;

  //   port.reconfigure(&|settings| {
  //     settings.set_baud_rate(BaudRate::from_speed(460800))?;
  //     Ok(())
  //   })?;

  //   port.set_timeout(Duration::from_millis(1000))?;

  //   *self.port.lock().unwrap() = port;

  //   Ok(self)
  // }

  pub fn set_mode(&self, mode: RndMode) -> Result<(), anyhow::Error> {
    let ref mut port = self.port.try_lock().map_err(|_| anyhow!("Lock Error"))?;
    let mut buf = vec![0u8];

    port.write(&[0x42, 0x52, {
      match mode {
        RndMode::Fake => 0x59,
        RndMode::Real => 0x01
      }
    }])?;

    port.read(&mut buf)?;

    if buf[0] != 0x01 && buf[0] != 0x59 {
      return Err(anyhow!("unexpected return data: {:?}", buf));
    }

    Ok(())
  }

  pub fn random(&self) -> Result<[u8; 1024], anyhow::Error> {
    let ref mut port = self.port.try_lock().map_err(|_| anyhow!("Lock Error"))?;
    let mut buf = [0u8; 1024];

    port.write(&[0x4f, 0x52])?;
    port.read(&mut buf)?;

    Ok(buf)
  }
}

