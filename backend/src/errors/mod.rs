pub mod codes;
pub mod conversion;
pub mod format;
pub mod serialization;
pub mod types;

#[cfg(test)]
mod tests;

pub use types::{
    AnalyticsError, AppError, AuthError, ContractError, EmailError, ErrorCategory, ExportError,
    FieldError, NotificationError, SerializationError, StorageError, ValidationError, WebhookError,
};
pub use conversion::{
    from_legacy_email, from_legacy_export, from_legacy_storage, from_validation_errors,
    from_validation_field, LegacyEmailKind, LegacyExportKind, LegacyStorageKind,
};
pub use serialization::ErrorResponse;
