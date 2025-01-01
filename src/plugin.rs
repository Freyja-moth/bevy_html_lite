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
pub struct DefaultTextColor(Color);

#[derive(Event, Reflect, Debug)]
pub struct PushSections(Sections);

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
    sections: Trigger<PushSections>,
    mut commands: Commands,
    fonts: Res<HtmlLiteFonts>,
    text_color: Res<DefaultTextColor>,
    dialogue: Query<Entity, With<DialogueArea>>,
) {
    let area = dialogue.single();

    let regular = fonts.regular.clone();
    let bold = fonts.bold.clone();
    let italic = fonts.italic.clone();
    let bold_italic = fonts.bold_italic.clone();

    let sections = sections
        .event()
        .0
        .iter()
        .cloned()
        .map(|section| {
            commands
                .spawn((
                    Node {
                        ..Default::default()
                    },
                    Text::new(section.value),
                    TextColor(section.color.unwrap_or(text_color.0)),
                    TextFont {
                        font: if section.italic && section.bold {
                            bold_italic.clone()
                        } else if section.italic {
                            italic.clone()
                        } else if section.bold {
                            bold.clone()
                        } else {
                            regular.clone()
                        },
                        ..Default::default()
                    },
                    DialogueSection,
                ))
                .id()
        })
        .collect_vec();

    commands.entity(area).add_children(sections.as_slice());
}

fn clear_sections(
    _: Trigger<ClearSections>,
    mut commands: Commands,
    dialogue: Query<Entity, With<DialogueArea>>,
) {
    let area = dialogue.single();

    commands.entity(area).despawn_descendants();
}
