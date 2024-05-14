pub(super) struct CPUConfig {
    pub logic_quirk: bool,
    pub shift_quirk: bool,
    pub jump_quirk: bool,
    pub load_store_quirk_offset: (bool, usize)
}

pub enum CPUVariant {
    Chip8,
    SChipv1_1,
    XOChip
}

impl From<CPUVariant> for CPUConfig {
    fn from(value: CPUVariant) -> Self {
        let logic_quirk = match value {
            CPUVariant::Chip8 => true,
            _ => false
        };

        let shift_quirk = match value {
            CPUVariant::SChipv1_1 => true,
            _ => false
        };

        let jump_quirk = match value {
            CPUVariant::SChipv1_1 => true,
            _ => false
        };

        let load_store_quirk_offset = match value {
            CPUVariant::SChipv1_1 => (true, 0),
            _ => (false, 1)
        };

        Self {
            logic_quirk,
            shift_quirk,
            jump_quirk,
            load_store_quirk_offset
        }
    }
}