use std::time::Instant;

use super::*;
use notify::{
    event::{CreateKind, EventKind, ModifyKind, RemoveKind},
    Event,
};

#[test]
fn test_empty_events_returns_false() {
    let events = vec![];
    assert!(!extract_should_build_from_event(events) == true);
}

#[test]
fn test_event_with_create_returns_true() {
    let events = vec![DebouncedEvent::new(
        Event::new(EventKind::Create(CreateKind::Any)),
        Instant::now(),
    )];

    assert!(extract_should_build_from_event(events) == true);
}

#[test]
fn test_event_with_modify_returns_true() {
    let events = vec![DebouncedEvent::new(
        Event::new(EventKind::Modify(ModifyKind::Any)),
        Instant::now(),
    )];

    assert!(extract_should_build_from_event(events) == true);
}

#[test]
fn test_event_with_remove_returns_true() {
    let events = vec![DebouncedEvent::new(
        Event::new(EventKind::Remove(RemoveKind::Any)),
        Instant::now(),
    )];

    assert!(extract_should_build_from_event(events) == true);
}

#[test]
fn test_event_with_other_kind_returns_false() {
    let events = vec![DebouncedEvent::new(
        Event::new(EventKind::Other),
        Instant::now(),
    )];

    assert!(extract_should_build_from_event(events) == false);
}

#[test]
fn test_mixed_events_returns_true_if_any_triggering_event_present() {
    let events = vec![
        DebouncedEvent::new(Event::new(EventKind::Other), Instant::now()),
        DebouncedEvent::new(
            Event::new(EventKind::Remove(RemoveKind::Any)),
            Instant::now(),
        ),
    ];

    assert!(extract_should_build_from_event(events) == true);
}

#[test]
fn test_create_watcher_instance() {}
