pub(super) struct CPUConfig {
    pub variant: CPUVariant,

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
    // TODO: Add SCHIPv1_1_Modern
    XOChip,
}

impl From<CPUVariant> for CPUConfig {
    fn from(variant: CPUVariant) -> Self {
        // Enabled features
        // TODO: Might need a quirk/variant for modern vs legacy SCHIP?
        let hires_enabled = matches!(
            variant,
            CPUVariant::SChipv1_0 | CPUVariant::SChipv1_1 | CPUVariant::XOChip
        );
        let scrolling_enabled = matches!(variant, CPUVariant::SChipv1_1 | CPUVariant::XOChip);
        let flag_registers_enabled = hires_enabled;

        // Quirks
        let logic_quirk = matches!(variant, CPUVariant::Chip8);
        let shift_quirk = matches!(
            variant,
            CPUVariant::Chip48 | CPUVariant::SChipv1_0 | CPUVariant::SChipv1_1
        );
        let jump_quirk = matches!(
            variant,
            CPUVariant::Chip48 | CPUVariant::SChipv1_0 | CPUVariant::SChipv1_1
        );

        let vblank_quirk = true;
        let scroll_quirk = false;

        let load_store_offset = match variant {
            CPUVariant::Chip48 => Some(0),
            CPUVariant::SChipv1_0 => Some(0),
            CPUVariant::SChipv1_1 => None,
            _ => Some(1),
        };

        let dxy0_lores_width = match variant {
            CPUVariant::SChipv1_0 => Some(8),
            CPUVariant::SChipv1_1 => Some(8),
            CPUVariant::XOChip => Some(16),
            _ => None,
        };

        // Misc
        let pc_start = 0x200;

        // Base resolution
        let mut resolutions = vec![(64, 32)];

        // Hi-res modes
        match variant {
            CPUVariant::SChipv1_1 => resolutions.push((128, 64)),
            CPUVariant::XOChip => resolutions.push((128, 64)),
            _ => (),
        };

        Self {
            variant,
            hires_enabled,
            scrolling_enabled,
            flag_registers_enabled,
            logic_quirk,
            shift_quirk,
            jump_quirk,
            vblank_quirk,
            scroll_quirk,
            load_store_offset,
            dxy0_lores_width,
            pc_start,
            resolutions,
        }
    }
}
