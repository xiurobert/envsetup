pub mod rust;
pub mod python;
pub mod docker;

pub enum Language {
    Rust,
    Python
}

pub enum ContainerSystem {
    Docker
}