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
    /// only for isc3.6
    Position = 0x05,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pos: u16,
}

impl Position {
    pub fn new_num(pos: u16) -> Result<Self, Error> {
        if pos < 3500 || pos > 11500 {
            return Err(Error("pos must be between 3500 and 11500"));
        }
        Ok(Self { pos: pos })
    }

    pub fn new_free() -> Self {
        Self { pos: 0 }
    }
}

impl From<u16> for Position {
    fn from(pos: u16) -> Self {
        Self::new_num(pos).unwrap()
    }
}

impl Into<u16> for Position {
    fn into(self) -> u16 {
        self.pos
    }
}

pub struct CommandGenerator {}

impl CommandGenerator {
    /// not tested
    ///
    /// id: 0 ~ 31
    ///
    /// pos: min 3500 ~ max 11500, center: 7500, free: 0
    pub fn unsafe_set_position(id: u8, pos: u16) -> [u8; 3] {
        [
            Command::Position as u8 + id,
            Self::h_byte(pos),
            Self::l_byte(pos),
        ]
    }

    /// id: 0 ~ 31
    ///
    /// pos: read from [Position] struct
    pub fn set_position(id: u8, pos: Position) -> Result<[u8; 3], Error> {
        Ok(Self::unsafe_set_position(Self::check_id(id)?, pos.into()))
    }

    /// not tested
    ///
    /// id: 0 ~ 31
    pub fn read_eeprom(id: u8) -> Result<[u8; 2], Error> {
        Ok([
            Command::ReadParam as u8 + Self::check_id(id)?,
            SC::EEPROM as u8,
        ])
    }

    /// not tested
    ///
    /// id: 0 ~ 31
    pub fn read_stretch(id: u8) -> Result<[u8; 2], Error> {
        Ok([
            Command::ReadParam as u8 + Self::check_id(id)?,
            SC::Stretch as u8,
        ])
    }

    /// not tested
    pub fn read_speed(id: u8) -> Result<[u8; 2], Error> {
        Ok([
            Command::ReadParam as u8 + Self::check_id(id)?,
            SC::Speed as u8,
        ])
    }

    /// not tested
    ///
    /// id: 0 ~ 31
    pub fn read_current(id: u8) -> Result<[u8; 2], Error> {
        Ok([
            Command::ReadParam as u8 + Self::check_id(id)?,
            SC::Current as u8,
        ])
    }

    /// not tested
    ///
    /// id: 0 ~ 31
    pub fn read_temp(id: u8) -> Result<[u8; 2], Error> {
        Ok([
            Command::ReadParam as u8 + Self::check_id(id)?,
            SC::Temp as u8,
        ])
    }

    /// not tested
    ///
    /// id: 0 ~ 31
    ///
    /// only for isc3.6
    pub fn read_position(id: u8) -> Result<[u8; 2], Error> {
        Ok([
            Command::ReadParam as u8 + Self::check_id(id)?,
            SC::Position as u8,
        ])
    }

    /// not tested
    fn check_id(id: u8) -> Result<u8, Error> {
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
