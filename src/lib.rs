// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-virsh: virsh component model.
//!
//! Real implementation: virsh_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VirshItem {
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
pub enum VirshError {
    EmptyVirshItemName,
    DuplicateVirshItem(String),
    EmptyEventSubject,
}

impl fmt::Display for VirshError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyVirshItemName => write!(f, "virsh_item name must not be empty"),
            Self::DuplicateVirshItem(id) => write!(f, "duplicate virsh_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for VirshError {}

pub struct Virsh {
    pub name: String,
    virsh_items: BTreeMap<String, VirshItem>,
    events: BTreeMap<String, Event>,
}

impl Virsh {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            virsh_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_virsh_item(&mut self, item: VirshItem) -> Result<(), VirshError> {
        if item.name.trim().is_empty() {
            return Err(VirshError::EmptyVirshItemName);
        }
        let id = item.id.clone();
        if self.virsh_items.contains_key(&id) {
            return Err(VirshError::DuplicateVirshItem(id.clone()));
        }
        self.virsh_items.insert(id, item);
        Ok(())
    }

    pub fn virsh_item_count(&self) -> usize {
        self.virsh_items.len()
    }

    pub fn get_virsh_item(&self, id: &str) -> Option<&VirshItem> {
        self.virsh_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), VirshError> {
        if event.subject.trim().is_empty() {
            return Err(VirshError::EmptyEventSubject);
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

    fn sample_virsh_item(id: &str, name: &str) -> VirshItem {
        VirshItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Virsh::new("Virsh");
        assert_eq!(m.name, "Virsh");
        assert_eq!(m.virsh_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_virsh_item() {
        let mut m = Virsh::new("Virsh");
        m.add_virsh_item(sample_virsh_item("a1", "Alpha")).unwrap();
        assert_eq!(m.virsh_item_count(), 1);
        assert!(m.get_virsh_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Virsh::new("Virsh");
        assert!(m.add_virsh_item(sample_virsh_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Virsh::new("Virsh");
        m.add_virsh_item(sample_virsh_item("x", "One")).unwrap();
        assert!(m.add_virsh_item(sample_virsh_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Virsh::new("Virsh");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
