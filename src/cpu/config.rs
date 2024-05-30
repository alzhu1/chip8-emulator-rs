pub(super) struct CPUConfig {
    pub variant: CPUVariant,
    pub logic_quirk: bool,
    pub shift_quirk: bool,
    pub jump_quirk: bool,
    pub load_store_quirk_offset: (bool, usize),

    // Resolutions
    // https://emulation.gametechwiki.com/index.php/Resolution#cite_note-CHIP-8_RES-1
    // pub base_resolution: (usize, usize),
    // pub hires_resolution: Option<(usize, usize)>,

    pub resolutions: Vec<(usize, usize)>
    // TODO: Megachip?
}

pub enum CPUVariant {
    Chip8,
    Chip48,
    SChipv1_1,
    XOChip,
}

impl From<CPUVariant> for CPUConfig {
    fn from(variant: CPUVariant) -> Self {
        let logic_quirk = match variant {
            CPUVariant::Chip8 => true,
            _ => false,
        };

        let shift_quirk = match variant {
            CPUVariant::Chip48 => true,
            CPUVariant::SChipv1_1 => true,
            _ => false,
        };

        let jump_quirk = match variant {
            CPUVariant::Chip48 => true,
            CPUVariant::SChipv1_1 => true,
            _ => false,
        };

        let load_store_quirk_offset = match variant {
            CPUVariant::Chip48 => (false, 0),
            CPUVariant::SChipv1_1 => (true, 0),
            _ => (false, 1),
        };

        // Base resolution
        let mut resolutions = vec!();
        resolutions.push((64, 32));

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
            resolutions
        }
    }
}
