use crate::prelude::*;
use bevy::prelude::*;
use itertools::Itertools;

/// The dialogue area, all sections are added as children to this struct
#[derive(Reflect, Component, Default, Debug)]
pub struct DialogueArea;

/// A spawned dialogue section
#[derive(Reflect, Component, Default, Debug)]
pub struct DialogueSection;

/// A font map used by [DefaultHtmlLiteDisplayPlugin] to decide which fonts to use for sections
#[derive(Resource)]
pub struct HtmlLiteFonts {
    pub regular: Handle<Font>,
    pub italic: Handle<Font>,
    pub bold: Handle<Font>,
    pub bold_italic: Handle<Font>,
}

/// The text color to be used when it isn't specified
#[derive(Resource, Default)]
pub struct DefaultTextColor(pub Color);

#[derive(Resource)]
pub struct DefaultFontSize(pub f32);
impl Default for DefaultFontSize {
    fn default() -> Self {
        Self(20.)
    }
}

#[derive(Event, Reflect, Debug)]
pub struct PushSections(Vec<Section>);
impl PushSections {
    pub fn new(value: impl IntoIterator<Item = Section>) -> Self {
        Self(Vec::from_iter(value))
    }
}

#[derive(Event, Reflect, Debug)]
pub struct ClearSections;

/// A default implementation for using the sections, it's not required for you to use this. It's
/// mostly here to give you an idea of how to use this crate.
pub struct DefaultHtmlLiteDisplayPlugin;

impl Plugin for DefaultHtmlLiteDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Section>()
            .register_type::<Sections>()
            .add_observer(push_sections)
            .add_observer(clear_sections);
    }
}

// Am I aware that this is a mess, yes, yes I am. Why haven't I fixed it? Tired.
fn push_sections(
    mut sections: On<PushSections>,
    mut commands: Commands,
    fonts: Res<HtmlLiteFonts>,
    text_color: Res<DefaultTextColor>,
    font_size: Res<DefaultFontSize>,
    dialogue: Single<Entity, With<DialogueArea>>,
) {
    let regular = fonts.regular.clone();
    let bold = fonts.bold.clone();
    let italic = fonts.italic.clone();
    let bold_italic = fonts.bold_italic.clone();

    let sections = sections
        .event_mut()
        .0
        .iter_mut()
        .map(|section| {
            let is_bold = section.has_tag("b");
            let is_italic = section.has_tag("i");
            let color = section.get_attribute::<Color>("color").cloned();

            let snippet = commands
                .spawn((
                    Node {
                        ..Default::default()
                    },
                    Text::new(section.text()),
                    TextColor(color.unwrap_or(text_color.0)),
                    TextFont {
                        font: if is_italic && is_bold {
                            bold_italic.clone()
                        } else if is_italic {
                            italic.clone()
                        } else if is_bold {
                            bold.clone()
                        } else {
                            regular.clone()
                        },
                        font_size: font_size.0,
                        ..Default::default()
                    },
                    DialogueSection,
                ))
                .id();

            // I'm not overly pleased with the but since observers don't implement Clone this is
            // really the only way to do it as far as I know
            if let Some(mut over) = section.take_attribute::<Observer>("over") {
                over.watch_entity(snippet);
                commands.spawn(over);
            }
            if let Some(mut out) = section.take_attribute::<Observer>("out") {
                out.watch_entity(snippet);
                commands.spawn(out);
            }
            if let Some(mut click) = section.take_attribute::<Observer>("click") {
                click.watch_entity(snippet);
                commands.spawn(click);
            }

            snippet
        })
        .collect_vec();

    commands.entity(*dialogue).add_children(sections.as_slice());
}

fn clear_sections(
    _: On<ClearSections>,
    mut commands: Commands,
    dialogue: Single<Entity, With<DialogueArea>>,
) {
    commands.entity(*dialogue).despawn_related::<Children>();
}
