use alloc::boxed::Box;
use crate::times::TimeMode;

pub struct ModeFactoryEntry  {
    pub name: &'static str,
    pub make: fn() -> Box<dyn TimeMode + Send>,
}

#[cfg(feature = "mode-default")]
fn make_default() -> Box<dyn TimeMode + Send> {
    Box::new(crate::config::default::DefaultMode {})
}

#[cfg(feature = "mode-am")]
fn make_am() -> Box<dyn TimeMode + Send> {
    Box::new(create::config::am::AdrianMorgan {})
}

pub static REGISTRY: &[ModeFactoryEntry] = &[
    #[cfg(feature = "mode-default")]
    ModeFactoryEntry { name: "default", make: make_default },

    #[cfg(feature = "mode-am")]
    ModeFactoryEntry { name: "am", make: make_am },
];

pub fn make_by_name(name: &str) -> Option<Box<dyn TimeMode + Send>> {
    for e in REGISTRY {
        if e.name == name { return Some((e.make)());}
    }
    None
}