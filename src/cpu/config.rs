pub(super) struct CPUConfig {
    // Enabled features
    pub hires_enabled: bool,
    pub scrolling_enabled: bool,
    pub flag_registers_enabled: bool,

    // Quirks
    pub logic_quirk: bool,  // Should set VF = 0 after AND/OR/XOR operation
    pub shift_quirk: bool,  // Should set VX to the shifted value of VX, not VY
    pub jump_quirk: bool,   // Should jump to XNN + VX, instead of NNN + V0
    pub vblank_quirk: bool, // Should set vblank interrupt (no processing until drawing finishes)
    pub scroll_quirk: bool, // Should scroll by lores pixel size (e.g. 2x2)
    pub load_store_offset: Option<usize>, // If set, mem load/store does I += (X + offset)
    pub dxy0_lores_width: Option<usize>, // If set, DXY0 draws an (width x 16) sprite

    // Misc
    pub pc_start: usize,

    // Resolutions
    // https://emulation.gametechwiki.com/index.php/Resolution#cite_note-CHIP-8_RES-1
    pub resolutions: Vec<(usize, usize)>, // TODO: Megachip?
}

pub enum CPUVariant {
    Chip8,
    Chip48,
    SChipv1_0,
    SChipv1_1,
    SChipModern,
    SChipC,
    XOChip,
}

impl CPUVariant {
    fn into_config(self) -> CPUConfig {
        match self {
            CPUVariant::Chip8 => CPUVariant::into_chip8_config(),
            CPUVariant::Chip48 => CPUVariant::into_chip48_config(),
            CPUVariant::SChipv1_0 => CPUVariant::into_schipv_1_0_config(),
            CPUVariant::SChipv1_1 => CPUVariant::into_schipv_1_1_config(),
            CPUVariant::SChipC => CPUVariant::into_schipc_config(),
            CPUVariant::SChipModern => CPUVariant::into_schip_modern_config(),
            CPUVariant::XOChip => CPUVariant::into_xo_chip_config(),
        }
    }

    fn into_chip8_config() -> CPUConfig {
        CPUConfig::default()
    }

    fn into_chip48_config() -> CPUConfig {
        CPUConfig {
            shift_quirk: true,
            logic_quirk: false,
            jump_quirk: true,
            load_store_offset: Some(0),
            ..Default::default()
        }
    }

    fn into_schipv_1_0_config() -> CPUConfig {
        CPUConfig {
            hires_enabled: true,
            flag_registers_enabled: true,
            logic_quirk: false,
            shift_quirk: true,
            jump_quirk: true,
            load_store_offset: Some(0),
            dxy0_lores_width: Some(8),
            resolutions: vec![(64, 32), (128, 64)],
            ..Default::default()
        }
    }

    fn into_schipv_1_1_config() -> CPUConfig {
        CPUConfig {
            hires_enabled: true,
            scrolling_enabled: true,
            flag_registers_enabled: true,
            logic_quirk: false,
            shift_quirk: true,
            jump_quirk: true,
            load_store_offset: None,
            dxy0_lores_width: Some(8),
            resolutions: vec![(64, 32), (128, 64)],
            ..Default::default()
        }
    }

    // Configurations were sourced from Cadmium
    fn into_schipc_config() -> CPUConfig {
        CPUConfig {
            hires_enabled: true,
            scrolling_enabled: true,
            flag_registers_enabled: true,
            logic_quirk: false,
            vblank_quirk: false,
            dxy0_lores_width: Some(16),
            resolutions: vec![(64, 32), (128, 64)],
            ..Default::default()
        }
    }

    // Configurations were sourced from Cadmium
    fn into_schip_modern_config() -> CPUConfig {
        CPUConfig {
            hires_enabled: true,
            scrolling_enabled: true,
            flag_registers_enabled: true,
            logic_quirk: false,
            shift_quirk: true,
            jump_quirk: true,
            vblank_quirk: false,
            load_store_offset: None,
            dxy0_lores_width: Some(16),
            resolutions: vec![(64, 32), (128, 64)],
            ..Default::default()
        }
    }

    fn into_xo_chip_config() -> CPUConfig {
        CPUConfig {
            hires_enabled: true,
            scrolling_enabled: true,
            flag_registers_enabled: true,
            logic_quirk: false,
            dxy0_lores_width: Some(16),
            resolutions: vec![(64, 32), (128, 64)],
            ..Default::default()
        }
    }
}

// Define config default as CHIP-8 params
impl Default for CPUConfig {
    fn default() -> Self {
        Self {
            hires_enabled: false,
            scrolling_enabled: false,
            flag_registers_enabled: false,
            logic_quirk: true,
            shift_quirk: false,
            jump_quirk: false,
            vblank_quirk: true,
            scroll_quirk: false,
            load_store_offset: Some(1),
            dxy0_lores_width: None,
            pc_start: 0x200,
            resolutions: vec![(64, 32)],
        }
    }
}

impl From<CPUVariant> for CPUConfig {
    fn from(variant: CPUVariant) -> Self {
        variant.into_config()
    }
}
