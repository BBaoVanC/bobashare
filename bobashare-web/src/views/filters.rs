use askama::Result;
use chrono::Duration;
use humansize::format_size;

pub fn humanduration(duration: &Duration) -> Result<String> {
    let duration = *duration;
    if duration < Duration::minutes(1) {
        return Ok(format!("{} seconds", duration.num_seconds()).into());
    }
    if duration < Duration::hours(1) {
        return Ok(format!("{} minutes", duration.num_minutes()).into());
    }
    if duration < Duration::days(1) {
        return Ok(format!("{} hours", duration.num_hours()).into());
    }
    // using weeks is dumb, let's use days up until a month
    if duration < Duration::days(30) {
        return Ok(format!("{} days", duration.num_days()).into());
    }
    // I think most people would assume a month is about 30 days
    return Ok(format!("{} months", duration.num_days() / 30).into());
    // probably no point going past months
}

pub fn humansize(size: &u64) -> Result<String> {
    Ok(format_size(*size, humansize::BINARY))
}
