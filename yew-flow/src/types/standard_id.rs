use uuid::Uuid;


/// # Standard Id type
///
/// Central/Standard ID type
///
/// - Currently set to String, but can be changed centrally from here if needed.
pub type StandardId = String;

/// # Identifier trait
///
/// Allows for generating unique random ids.
pub trait IdentifierExt {
    /// Generates a new unique identifier.
    fn generate() -> Self;
}

impl IdentifierExt for StandardId {
    fn generate() -> Self {
        Uuid::new_v4().to_string()
    }
}
