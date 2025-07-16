pub mod files;
pub mod processes;
pub mod cpu;

pub fn option_to_str<T: Default>(option: Option<T>) -> T {
    option.unwrap_or_default()
}
