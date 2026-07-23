mod badges;
mod manifest;
pub mod project;

pub use self::badges::{BadgeInfo, SUPPORTED_BADGES};
pub use self::manifest::get_manifest;
pub use self::manifest::Manifest;

/// The badges cargo-readme can render, in output order.
pub fn supported_badges() -> &'static [BadgeInfo] {
    SUPPORTED_BADGES
}
