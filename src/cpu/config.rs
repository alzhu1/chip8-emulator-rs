pub(super) struct CPUConfig {
    pub variant: CPUVariant,
    pub logic_quirk: bool,
    pub shift_quirk: bool,
    pub jump_quirk: bool,
    pub load_store_quirk_offset: (bool, usize),
    pub vblank_quirk: bool,
    pub scroll_quirk: bool,

    // Resolutions
    // https://emulation.gametechwiki.com/index.php/Resolution#cite_note-CHIP-8_RES-1
    // pub base_resolution: (usize, usize),
    // pub hires_resolution: Option<(usize, usize)>,
    pub resolutions: Vec<(usize, usize)>, // TODO: Megachip?
}

pub enum CPUVariant {
    Chip8,
    Chip48,
    SChipv1_1,
    // TODO: Add SCHIPv1_1_Modern
    XOChip,
}

impl From<CPUVariant> for CPUConfig {
    fn from(variant: CPUVariant) -> Self {
        let logic_quirk = matches!(variant, CPUVariant::Chip8);
        let shift_quirk = matches!(variant, CPUVariant::Chip48 | CPUVariant::SChipv1_1);
        let jump_quirk = matches!(variant, CPUVariant::Chip48 | CPUVariant::SChipv1_1);

        // TODO: Might need a quirk/variant for modern vs legacy SCHIP?

        let load_store_quirk_offset = match variant {
            CPUVariant::Chip48 => (false, 0),
            CPUVariant::SChipv1_1 => (true, 0),
            _ => (false, 1),
        };

        let vblank_quirk = true;
        let scroll_quirk = false;

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
            logic_quirk,
            shift_quirk,
            jump_quirk,
            load_store_quirk_offset,
            vblank_quirk,
            scroll_quirk,
            resolutions,
        }
    }
}
