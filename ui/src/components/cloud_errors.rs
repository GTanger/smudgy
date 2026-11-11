//! Shared mapping from [`CloudError`] to user-facing strings for the cloud /
//! account / social UI surfaces.

use smudgy_cloud::CloudError;

/// Renders a [`CloudError`] as a short, user-facing message.
pub fn display_error(err: &CloudError) -> String {
    match err {
        CloudError::AreaNotFound(id) => {
            smudgy_i18n::t!("cloud-error-area-not-found", "id" => id)
        }
        CloudError::RoomNotFound(room) => smudgy_i18n::t!(
            "cloud-error-room-not-found",
            "room" => room.room_number,
            "area" => room.area_id
        ),
        CloudError::ExitNotFound(id) => {
            smudgy_i18n::t!("cloud-error-exit-not-found", "id" => id)
        }
        CloudError::LabelNotFound(id) => {
            smudgy_i18n::t!("cloud-error-label-not-found", "id" => id)
        }
        CloudError::ShapeNotFound(id) => {
            smudgy_i18n::t!("cloud-error-shape-not-found", "id" => id)
        }
        CloudError::PropertyNotFound {
            entity_type,
            entity_id,
            property_name,
        } => smudgy_i18n::t!(
            "cloud-error-property-not-found",
            "property" => property_name,
            "entity_type" => entity_type,
            "entity_id" => entity_id
        ),
        CloudError::InvalidInput(detail) => {
            smudgy_i18n::t!("cloud-error-invalid-input", "detail" => detail)
        }
        CloudError::DatabaseError(detail) => {
            smudgy_i18n::t!("cloud-error-database", "detail" => detail)
        }
        CloudError::NetworkError(detail) => {
            smudgy_i18n::t!("cloud-error-network", "detail" => detail)
        }
        CloudError::SerializationError(detail) => {
            smudgy_i18n::t!("cloud-error-serialization", "detail" => detail)
        }
        CloudError::AuthenticationError(detail) => {
            smudgy_i18n::t!("cloud-error-authentication", "detail" => detail)
        }
        CloudError::PermissionDenied(detail) => {
            smudgy_i18n::t!("cloud-error-permission", "detail" => detail)
        }
        CloudError::InternalError(detail) => {
            smudgy_i18n::t!("cloud-error-internal", "detail" => detail)
        }
        CloudError::PendingOperations(detail) => {
            smudgy_i18n::t!("cloud-error-pending", "detail" => detail)
        }
        // Authentication failures often carry an internal English diagnostic
        // such as `no credential configured`.  That detail is useful in logs,
        // but it is neither actionable nor suitable for a localized UI.
        CloudError::Unauthorized(_) => smudgy_i18n::t!("cloud-error-unauthorized"),
        CloudError::EmailNotVerified => smudgy_i18n::t!("cloud-error-email-unverified"),
        CloudError::NotFoundOrNoAccess => smudgy_i18n::t!("cloud-error-not-found"),
        CloudError::NameUnavailable(detail) => {
            smudgy_i18n::t!("cloud-error-name-unavailable", "detail" => detail)
        }
        CloudError::UpgradeRequired => smudgy_i18n::t!("cloud-error-upgrade-required"),
        CloudError::VersionUnavailable(version) if version.is_empty() => {
            smudgy_i18n::t!("cloud-error-version-unavailable")
        }
        CloudError::VersionUnavailable(version) => smudgy_i18n::t!(
            "cloud-error-version-unavailable-number",
            "version" => version
        ),
        CloudError::VersionNotYanked => smudgy_i18n::t!("cloud-error-version-not-yanked"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unauthorized_errors_do_not_leak_internal_diagnostics() {
        let rendered = display_error(&CloudError::Unauthorized(
            "no credential configured".to_string(),
        ));

        assert!(!rendered.contains("credential"));
        assert!(!rendered.contains("configured"));
    }
}
