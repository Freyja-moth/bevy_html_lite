use std::{any::Any, fmt::Debug};

use bevy::{prelude::Observer, reflect::Reflect};
use std::collections::HashMap;

pub enum Attribute {
    Observer(Observer),
    String(String),
}

#[derive(Reflect, Default)]
pub struct Section {
    text: String,
    tags: Vec<String>,
    #[reflect(ignore)]
    attributes: HashMap<String, Box<dyn Any + Sync + Send>>,
}
impl Debug for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let attributes: HashMap<&str, &str> = self
            .attributes
            .keys()
            .map(|name| (name.as_str(), "..."))
            .collect();
        f.debug_struct("Section")
            .field("text", &self.text)
            .field("tags", &self.tags)
            .field("attributes", &attributes)
            .finish()
    }
}
impl Section {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            tags: vec![],
            attributes: HashMap::default(),
        }
    }
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }
    pub fn with_attribute(
        mut self,
        name: impl Into<String>,
        value: impl Any + Sync + Send,
    ) -> Self {
        self.attributes.insert(name.into(), Box::new(value));
        self
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.contains(&tag.into())
    }

    /// Gets an attribute from the section and casts it to [`V`]
    pub fn get_attribute<V: 'static>(&self, name: &str) -> Option<&V> {
        self.attributes
            .get(name)
            .and_then(|attr| attr.downcast_ref())
    }

    /// Removes an attribute from the section and casts it to an owned value of [`V`].
    ///
    /// Useful if you need to have owned access to an attribute that cannot implement clone.
    pub fn take_attribute<V: 'static>(&mut self, name: &str) -> Option<V> {
        self.attributes
            .remove(name)
            .and_then(|attr| attr.downcast().ok())
            .map(|attr| *attr)
    }
}

#[derive(Reflect, Debug)]
pub struct Sections(Vec<Section>);
impl FromIterator<Section> for Sections {
    fn from_iter<T: IntoIterator<Item = Section>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}
impl Sections {
    pub fn new(sections: impl IntoIterator<Item = Section>) -> Self {
        Self::from_iter(sections)
    }
    pub fn new_single(section: Section) -> Self {
        Self::from_iter([section])
    }

    pub fn iter(&self) -> impl Iterator<Item = &Section> {
        self.0.iter()
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Section> {
        self.0.iter_mut()
    }
}
