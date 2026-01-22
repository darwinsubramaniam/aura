// require update if last updated at is none or last updated at is more than duration ago
pub fn require_update(
    last_updated_at: Option<chrono::NaiveDateTime>,
    duration: chrono::Duration,
) -> bool {
    last_updated_at.is_none()
        || last_updated_at.unwrap() < chrono::Local::now().naive_local() - duration
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_require_update_none() {
        assert!(require_update(None, chrono::Duration::hours(1)));
    }

    #[test]
    fn test_require_update_expired() {
        let last_updated_at = chrono::Local::now().naive_local() - chrono::Duration::hours(2);
        assert!(require_update(
            Some(last_updated_at),
            chrono::Duration::hours(1)
        ));
    }

    #[test]
    fn test_require_update_not_expired() {
        let last_updated_at = chrono::Local::now().naive_local() - chrono::Duration::minutes(30);
        assert!(!require_update(
            Some(last_updated_at),
            chrono::Duration::hours(1)
        ));
    }
}
