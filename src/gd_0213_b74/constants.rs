/// LUT definition for setup of the SSD1680 controller
pub struct WaveformSetting {
    pub(crate) lut: [u8; 153],

    pub(crate) eopq: u8,
    pub(crate) vgh: u8,
    pub(crate) vsh1: u8,
    pub(crate) vsh2: u8,
    pub(crate) vsl: u8,
    pub(crate) vcom: u8,
}

impl From<[u8; 159]> for WaveformSetting {
    fn from(value: [u8; 159]) -> Self {
        let mut lut = [0; 153];
        lut.copy_from_slice(&value[..153]);

        WaveformSetting {
            lut,
            eopq: value[153],
            vgh: value[154],
            vsh1: value[155],
            vsh2: value[156],
            vsl: value[157],
            vcom: value[158],
        }
    }
}

#[rustfmt::skip]
pub(crate) const LUT_GRAY4: WaveformSetting = WaveformSetting {
    lut: [
        // VS groups
        // VS[nX-LUTm] represents Source and VCOM voltage level which is used in each phase.
        0x40, 0x48, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // LUT0
        0x08, 0x48, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // LUT1
        0x02, 0x48, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // LUT2
        0x20, 0x48, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // LUT3
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // LUT4
        
        // TP[nX] represents the phase length set by the number of frame
        // SR[nAB] and SR[nCD] represent the state repeat counting number for Phase A & B and Phase C & D respectively
        // RP[n] represents the repeat counting number for the Group

        // TPa, TPb, SRab, TPc, TPd, SRcd, RP
        0x0a, 0x19, 0x00, 0x03, 0x08, 0x00, 0x00, // Group 0
        0x14, 0x01, 0x00, 0x14, 0x01, 0x00, 0x03, // Group 1
        0x0a, 0x03, 0x00, 0x08, 0x19, 0x00, 0x00, // Group 2
        0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, // Group 3
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Group 4
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Group 5
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Group 6
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Group 7
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Group 8
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Group 9
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Group 10
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Group 11

        // FR Frame group 
        // Group per nibble
        0x22, 0x22, 0x22, 0x22, 0x22, 0x22,
        // 0x44, 0x44, 0x44, 0x44, 0x44, 0x44, 
        // 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 

        // XON Gate scan selection
        // Group per bit-pair
        0x00, 0x00, 0x00,
    ],

    eopq: 0x22,
    vgh: 0x17,
    vsh1: 0x41,
    vsh2: 0x00,
    vsl: 0x32,
    vcom: 0x1c,
};

#[rustfmt::skip]
pub(crate) const LUT_FAST_REFRESH: WaveformSetting = WaveformSetting {
    lut: [
        // VS groups
        // VS[nX-LUTm] represents Source and VCOM voltage level which is used in each phase.
        0x00, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // LUT0
        0x80, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // LUT1
        0x40, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // LUT2
        0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // LUT3
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // LUT4
        
        
        // TP[nX] represents the phase length set by the number of frame
        // SR[nAB] and SR[nCD] represent the state repeat counting number for Phase A & B and Phase C & D respectively
        // RP[n] represents the repeat counting number for the Group

        // TPa, TPb, SRab, TPc, TPd, SRcd, RP
        0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Group 0
        0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Group 1
        0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Group 2
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Group 3
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Group 4
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Group 5
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Group 6
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Group 7
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Group 8
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Group 9
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Group 10
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Group 11
        // FR Frame group
        // Group per nibble
        0x22, 0x22, 0x22, 0x22, 0x22, 0x22, // XON Gate scan selection
        // Group per bit-pair
        0x00, 0x00, 0x00,
    ],

    eopq: 0x22,
    vgh: 0x17,
    vsh1: 0x41,
    vsh2: 0x00,
    vsl: 0x32,
    // vcom: 0x36,
    vcom: 0x1c,
};

// #[rustfmt::skip]
pub(crate) const LUT_FAST_REFRESH2: WaveformSetting = WaveformSetting {
    lut: [
        // VS groups
        // VS[nX-LUTm] represents Source and VCOM voltage level which is used in each phase.
        0x80, 0x4A, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // LUT0
        0x40, 0x4A, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // LUT1
        0x80, 0x4A, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // LUT2
        0x40, 0x4A, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // LUT3
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // LUT4
        // TP[nX] represents the phase length set by the number of frame
        // SR[nAB] and SR[nCD] represent the state repeat counting number for Phase A & B and Phase C & D respectively
        // RP[n] represents the repeat counting number for the Group

        // TPa, TPb, SRab, TPc, TPd, SRcd, RP
        0x0F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Group 0
        0x0F, 0x00, 0x00, 0x0F, 0x00, 0x00, 0x02, // Group 1
        0x0F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Group 2
        0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Group 3
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Group 4
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Group 5
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Group 6
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Group 7
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Group 8
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Group 9
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Group 10
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // Group 11
        // FR Frame group
        // Group per nibble
        0x22, 0x22, 0x22, 0x22, 0x22, 0x22, // XON Gate scan selection
        // Group per bit-pair
        0x00, 0x00, 0x00,
    ],

    eopq: 0x22,
    vgh: 0x17,
    vsh1: 0x41,
    vsh2: 0x00,
    vsl: 0x32,
    // vcom: 0x36,
    vcom: 0x1c,
};
