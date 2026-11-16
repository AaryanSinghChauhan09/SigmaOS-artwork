// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-mint-artwork: mint artwork component model.
//!
//! Real implementation: artworks, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Artwork {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MintArtworkError {
    EmptyArtworkName,
    DuplicateArtwork(String),
    EmptyEventSubject,
}

impl fmt::Display for MintArtworkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyArtworkName => write!(f, "artwork name must not be empty"),
            Self::DuplicateArtwork(id) => write!(f, "duplicate artwork {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for MintArtworkError {}

pub struct MintArtwork {
    pub name: String,
    artworks: BTreeMap<String, Artwork>,
    events: BTreeMap<String, Event>,
}

impl MintArtwork {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            artworks: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_artwork(&mut self, item: Artwork) -> Result<(), MintArtworkError> {
        if item.name.trim().is_empty() {
            return Err(MintArtworkError::EmptyArtworkName);
        }
        let id = item.id.clone();
        if self.artworks.contains_key(&id) {
            return Err(MintArtworkError::DuplicateArtwork(id.clone()));
        }
        self.artworks.insert(id, item);
        Ok(())
    }

    pub fn artwork_count(&self) -> usize {
        self.artworks.len()
    }

    pub fn get_artwork(&self, id: &str) -> Option<&Artwork> {
        self.artworks.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), MintArtworkError> {
        if event.subject.trim().is_empty() {
            return Err(MintArtworkError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_artwork(id: &str, name: &str) -> Artwork {
        Artwork {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = MintArtwork::new("MintArtwork");
        assert_eq!(m.name, "MintArtwork");
        assert_eq!(m.artwork_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_artwork() {
        let mut m = MintArtwork::new("MintArtwork");
        m.add_artwork(sample_artwork("a1", "Alpha")).unwrap();
        assert_eq!(m.artwork_count(), 1);
        assert!(m.get_artwork("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = MintArtwork::new("MintArtwork");
        assert!(m.add_artwork(sample_artwork("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = MintArtwork::new("MintArtwork");
        m.add_artwork(sample_artwork("x", "One")).unwrap();
        assert!(m.add_artwork(sample_artwork("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = MintArtwork::new("MintArtwork");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
