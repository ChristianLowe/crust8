
pub struct Quirks {
    pub is_lazy_shift: bool,
    pub is_static_dump_index: bool,
}

impl Quirks {
    pub fn from_flag(is_active: bool) -> Self {
        if is_active {
            Quirks::active()
        } else {
            Quirks::inactive()
        }
    }

    pub fn active() -> Self {
        Quirks {
            is_lazy_shift: true,
            is_static_dump_index: true,
        }
    }

    pub fn inactive() -> Self {
        Quirks {
            is_lazy_shift: false,
            is_static_dump_index: false,
        }
    }
}
