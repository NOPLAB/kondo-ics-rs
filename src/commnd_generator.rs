#[derive(Debug, Clone, Copy)]
pub struct Error(&'static str);

#[derive(Debug, Clone, Copy)]
pub enum Command {
    Position = 0x80,
    ReadParam = 0xa0,
    WriteParam = 0xc0,
    Id = 0xe0,
}

#[derive(Debug, Clone, Copy)]
pub enum SC {
    EEPROM = 0x00,
    Stretch = 0x01,
    Speed = 0x02,
    Current = 0x03,
    Temp = 0x04,
    Position = 0x05, // only for isc3.6
}

pub struct CommandGenerator {}

impl CommandGenerator {
    /// not tested
    /// id: 0 ~ 31
    /// pos: min 3500 ~ max 11500, center: 7500, free: 0
    pub fn set_position(id: u8, pos: u16) -> Result<[u8; 3], Error> {
        Ok([
            Command::Position as u8 + Self::id(id)?,
            Self::h_byte(pos),
            Self::l_byte(pos),
        ])
    }

    /// not tested
    /// id: 0 ~ 31
    pub fn read_eeprom(id: u8) -> Result<[u8; 2], Error> {
        Ok([Command::ReadParam as u8 + Self::id(id)?, SC::EEPROM as u8])
    }

    /// not tested
    /// id: 0 ~ 31
    pub fn read_stretch(id: u8) -> Result<[u8; 2], Error> {
        Ok([Command::ReadParam as u8 + Self::id(id)?, SC::Stretch as u8])
    }

    /// not tested
    pub fn read_speed(id: u8) -> Result<[u8; 2], Error> {
        Ok([Command::ReadParam as u8 + Self::id(id)?, SC::Speed as u8])
    }

    /// not tested
    /// id: 0 ~ 31
    pub fn read_current(id: u8) -> Result<[u8; 2], Error> {
        Ok([Command::ReadParam as u8 + Self::id(id)?, SC::Current as u8])
    }

    /// not tested
    /// id: 0 ~ 31
    pub fn read_temp(id: u8) -> Result<[u8; 2], Error> {
        Ok([Command::ReadParam as u8 + Self::id(id)?, SC::Temp as u8])
    }

    /// not tested
    /// id: 0 ~ 31
    /// only for isc3.6
    pub fn read_position(id: u8) -> Result<[u8; 2], Error> {
        Ok([Command::ReadParam as u8 + Self::id(id)?, SC::Position as u8])
    }

    /// not tested
    fn id(id: u8) -> Result<u8, Error> {
        if id > 0x1f {
            return Err(Error("id must be less than 0x1f"));
        }
        Ok(id & 0x1f)
    }

    /// not tested
    fn h_byte(byte: u16) -> u8 {
        (byte >> 7 & 0x7f) as u8
    }

    /// not tested
    fn l_byte(byte: u16) -> u8 {
        (byte & 0x7f) as u8
    }
}
