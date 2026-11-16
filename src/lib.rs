// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-langchain: langchain component model.
//!
//! Real implementation: langchain_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LangchainItem {
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
pub enum LangchainError {
    EmptyLangchainItemName,
    DuplicateLangchainItem(String),
    EmptyEventSubject,
}

impl fmt::Display for LangchainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyLangchainItemName => write!(f, "langchain_item name must not be empty"),
            Self::DuplicateLangchainItem(id) => write!(f, "duplicate langchain_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for LangchainError {}

pub struct Langchain {
    pub name: String,
    langchain_items: BTreeMap<String, LangchainItem>,
    events: BTreeMap<String, Event>,
}

impl Langchain {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            langchain_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_langchain_item(&mut self, item: LangchainItem) -> Result<(), LangchainError> {
        if item.name.trim().is_empty() {
            return Err(LangchainError::EmptyLangchainItemName);
        }
        let id = item.id.clone();
        if self.langchain_items.contains_key(&id) {
            return Err(LangchainError::DuplicateLangchainItem(id.clone()));
        }
        self.langchain_items.insert(id, item);
        Ok(())
    }

    pub fn langchain_item_count(&self) -> usize {
        self.langchain_items.len()
    }

    pub fn get_langchain_item(&self, id: &str) -> Option<&LangchainItem> {
        self.langchain_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), LangchainError> {
        if event.subject.trim().is_empty() {
            return Err(LangchainError::EmptyEventSubject);
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

    fn sample_langchain_item(id: &str, name: &str) -> LangchainItem {
        LangchainItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Langchain::new("Langchain");
        assert_eq!(m.name, "Langchain");
        assert_eq!(m.langchain_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_langchain_item() {
        let mut m = Langchain::new("Langchain");
        m.add_langchain_item(sample_langchain_item("a1", "Alpha")).unwrap();
        assert_eq!(m.langchain_item_count(), 1);
        assert!(m.get_langchain_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Langchain::new("Langchain");
        assert!(m.add_langchain_item(sample_langchain_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Langchain::new("Langchain");
        m.add_langchain_item(sample_langchain_item("x", "One")).unwrap();
        assert!(m.add_langchain_item(sample_langchain_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Langchain::new("Langchain");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
