#[derive(Debug)]
pub struct CmdAuthLogonChallenge {
    _opcode: u8,
    _protocol_version: u8,
    _size: u16,
    _game_name: String,
    _version: [u8; 3],
    _build: u16,
    _platform: String,
    _os: String,
    _locale: String,
    _worldregion_bias: u32,
    _ip: [u8; 4], // u32 on wowdev.wiki
    _account_name_length: u8,
    _account_name: String,
}

impl CmdAuthLogonChallenge {
    pub fn new(buf: &Vec<u8>) -> CmdAuthLogonChallenge {
        let account_name_length: usize = buf[33].into();

        CmdAuthLogonChallenge {
            _opcode: buf[0],
            _protocol_version: buf[1],
            _size: u16::from_le_bytes([buf[2], buf[3]]),
            _game_name: std::str::from_utf8(&[buf[4], buf[5], buf[6], buf[7]])
                .unwrap()
                .to_string(),
            _version: [buf[8], buf[9], buf[10]],
            _build: u16::from_le_bytes([buf[11], buf[12]]),
            _platform: std::str::from_utf8(&[buf[13], buf[14], buf[15], buf[16]])
                .unwrap()
                .to_string(),
            _os: std::str::from_utf8(&[buf[17], buf[18], buf[19], buf[20]])
                .unwrap()
                .to_string(),
            _locale: std::str::from_utf8(&[buf[21], buf[22], buf[23], buf[24]])
                .unwrap()
                .to_string(),
            _worldregion_bias: u32::from_le_bytes([buf[25], buf[26], buf[27], buf[28]]),
            _ip: [buf[29], buf[30], buf[31], buf[32]],
            _account_name_length: buf[33],
            _account_name: std::str::from_utf8(&buf[34..(34 + account_name_length)])
                .unwrap()
                .to_string(),
        }
    }
}