use bevy::prelude::*;

/// A section describing a snippet of text
#[derive(Reflect, Clone, Debug)]
pub struct Section {
    /// The value being displayed
    pub value: String,
    /// Whether or not the text should be bold
    pub bold: bool,
    /// Whether or not the text should be italic
    pub italic: bool,
    /// The color of the text (if specified)
    pub color: Option<Color>,
}
impl Section {
    pub fn new(value: impl Into<String>, bold: bool, italic: bool, color: Option<Color>) -> Self {
        Self {
            value: value.into(),
            bold,
            italic,
            color,
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn is_bold(&self) -> bool {
        self.bold
    }
    pub fn is_italic(&self) -> bool {
        self.italic
    }
    pub fn color(&self) -> Option<Color> {
        self.color
    }
}

#[derive(Reflect, Deref, Default, Debug)]
pub struct Sections(Vec<Section>);
impl FromIterator<Section> for Sections {
    fn from_iter<T: IntoIterator<Item = Section>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}
impl Sections {
    pub fn new(value: impl IntoIterator<Item = Section>) -> Self {
        Self::from_iter(value)
    }
    pub fn new_single(value: Section) -> Self {
        Self::from_iter([value])
    }
}
