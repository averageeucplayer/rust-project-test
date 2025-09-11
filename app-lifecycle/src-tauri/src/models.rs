#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum Kind {
    Unknown,
    Spawn,
    Damage,
    ZoneChange
}
