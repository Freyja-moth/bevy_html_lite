use std::vec::IntoIter;
use std::{fmt::Debug, slice};

use bevy::{ecs::system::IntoObserverSystem, prelude::*};

/// A section describing a snippet of text
#[derive(Reflect, Default)]
pub struct Section {
    /// The value being displayed
    pub value: String,
    /// Whether or not the text should be bold
    pub bold: bool,
    /// Whether or not the text should be italic
    pub italic: bool,
    /// The color of the text (if specified)
    pub color: Option<Color>,
    pub font_size: Option<f32>,
    #[reflect(ignore)]
    pub over: Option<Observer>,
    #[reflect(ignore)]
    pub out: Option<Observer>,
    #[reflect(ignore)]
    pub click: Option<Observer>,
}
impl Debug for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Section")
            .field("value", &self.value)
            .field("bold", &self.bold)
            .field("italic", &self.italic)
            .field("color", &self.color)
            .field("over", &"...")
            .field("out", &"...")
            .field("click", &"...")
            .finish()
    }
}
impl Section {
    pub fn new(value: impl Into<String>, bold: bool, italic: bool) -> Self {
        Self {
            value: value.into(),
            bold,
            italic,
            ..Default::default()
        }
    }

    // pub fn with_marker(mut self, marker: impl Component) -> Self {
    // self.marker = Box::new(marker);
    // self
    // }
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
    pub fn with_font_size(mut self, font_size: f32) -> Self {
        self.font_size = Some(font_size);
        self
    }
    pub fn with_over<M>(mut self, over: impl IntoObserverSystem<Pointer<Over>, (), M>) -> Self {
        self.over = Some(Observer::new(over));
        self
    }
    pub fn with_out<M>(mut self, out: impl IntoObserverSystem<Pointer<Out>, (), M>) -> Self {
        self.out = Some(Observer::new(out));
        self
    }
    pub fn with_click<M>(mut self, click: impl IntoObserverSystem<Pointer<Click>, (), M>) -> Self {
        self.click = Some(Observer::new(click));
        self
    }

    // pub fn set_marker(&mut self, marker: impl Component) -> &mut Self {
    // self.marker = Box::new(marker);
    // self
    // }
    pub fn set_color(&mut self, color: Color) -> &mut Self {
        self.color = Some(color);
        self
    }
    pub fn set_font_size(&mut self, font_size: f32) -> &mut Self {
        self.font_size = Some(font_size);
        self
    }
    pub fn set_over<M>(
        &mut self,
        over: impl IntoObserverSystem<Pointer<Over>, (), M>,
    ) -> &mut Self {
        self.over = Some(Observer::new(over));
        self
    }
    pub fn set_out<M>(&mut self, out: impl IntoObserverSystem<Pointer<Out>, (), M>) -> &mut Self {
        self.out = Some(Observer::new(out));
        self
    }
    pub fn set_click<M>(
        &mut self,
        click: impl IntoObserverSystem<Pointer<Click>, (), M>,
    ) -> &mut Self {
        self.click = Some(Observer::new(click));
        self
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
    pub fn font_size(&self) -> Option<f32> {
        self.font_size
    }
    pub fn click(&self) -> Option<&Observer> {
        self.click.as_ref()
    }
    pub fn over(&self) -> Option<&Observer> {
        self.over.as_ref()
    }
    pub fn out(&self) -> Option<&Observer> {
        self.out.as_ref()
    }
}

#[derive(Reflect, Deref, Default, Debug)]
pub struct Sections(pub Vec<Section>);
impl FromIterator<Section> for Sections {
    fn from_iter<T: IntoIterator<Item = Section>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}
impl<'a> IntoIterator for Sections {
    type Item = Section;
    type IntoIter = IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
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
