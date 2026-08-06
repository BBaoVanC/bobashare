// Unit tests for root functions in [`bobashare_web`]

use crate::str_to_duration;

#[test]
fn no_number() {
    assert!(str_to_duration("s").is_err());
    assert!(str_to_duration("h").is_err());
    assert!(str_to_duration("d").is_err());
    assert!(str_to_duration("mon").is_err());
    assert!(str_to_duration("y").is_err());
}

#[test]
fn trailing_junk() {
    assert!(str_to_duration("15djkak").is_err());
    assert!(str_to_duration("15djkak").is_err());
    assert!(str_to_duration("15ykjjk").is_err());
    assert!(str_to_duration("15mon23u").is_err());
}

#[test]
fn preceeding_junk() {
    assert!(str_to_duration("sdf15d").is_err());
    assert!(str_to_duration("s23y").is_err());
    assert!(str_to_duration("$93h").is_err());
}

#[test]
fn large_counts() {
    use std::time::Duration;

    // See issue #27, where counts of 3 or more digits were rejected due to
    // buggy code not splitting the number part from the duration unit properly
    assert_eq!(
        str_to_duration("100m").unwrap(),
        Duration::from_secs(100 * 60)
    );
    assert_eq!(
        str_to_duration("999d").unwrap(),
        Duration::from_secs(999 * 60 * 60 * 24),
    );
    // Larger counts, e.g. using seconds or minutes for finer resolution.
    assert_eq!(
        str_to_duration("86400s").unwrap(),
        Duration::from_secs(86400)
    );
    assert_eq!(
        str_to_duration("1440m").unwrap(),
        Duration::from_secs(1440 * 60),
    );
}
