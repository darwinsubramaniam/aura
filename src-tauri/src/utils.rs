// require update if last updated at is none or last updated at is more than duration ago
pub fn require_update(
    last_updated_at: Option<chrono::NaiveDateTime>,
    duration: chrono::Duration,
) -> bool {
    last_updated_at.is_none()
        || last_updated_at.unwrap() < chrono::Local::now().naive_local() - duration
}
