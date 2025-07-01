use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_html_lite::prelude::*;
use std::fmt::Debug;

const BACKGROUND_COLOR: Color = Color::srgb_u8(45, 35, 46);
const NORMAL_COLOR: Color = Color::srgb_u8(106, 142, 174);
const HOVERED_COLOR: Color = Color::srgb_u8(155, 209, 229);

fn tada(_: Trigger<Pointer<Click>>, mut commands: Commands) {
    commands.trigger(PushSections::new(sections!({ "Tada" })));
}

fn set_text_color_on<E: Reflect + Clone + Debug>(
    color: Color,
) -> impl Fn(Trigger<Pointer<E>>, Query<&mut TextColor>) -> Result<(), BevyError> {
    move |trigger, mut query| {
        let mut text_color = query.get_mut(trigger.target())?;

        text_color.0 = color;

        Ok(())
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, DefaultHtmlLiteDisplayPlugin))
        .init_resource::<DefaultFontSize>()
        .insert_resource(DefaultTextColor(NORMAL_COLOR))
        .add_systems(Startup, (init_fonts, spawn_ui))
        .add_systems(
            Update,
            (
                say_hi.run_if(input_just_pressed(KeyCode::Space)),
                clear.run_if(input_just_pressed(KeyCode::Backspace)),
            ),
        )
        .run();
}

fn init_fonts(mut commands: Commands, asset_server: Res<AssetServer>) {
    let regular = asset_server.load("regular.otf");
    let italic = asset_server.load("italic.otf");
    let bold = asset_server.load("bold.otf");
    let bold_italic = asset_server.load("bold_italic.otf");

    commands.insert_resource(HtmlLiteFonts {
        regular,
        italic,
        bold,
        bold_italic,
    });
}

fn spawn_ui(mut commands: Commands) {
    let camera = commands.spawn(Camera2d).id();

    commands.spawn((
        UiTargetCamera(camera),
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Percent(2.)),
            row_gap: Val::Percent(2.),
            ..Default::default()
        },
        BackgroundColor(BACKGROUND_COLOR),
        children![
            (
                Text::new("Press Space to say hi, and Backspace to clear the text!"),
                TextColor(NORMAL_COLOR),
                TextLayout::new_with_justify(Justify::Center),
                Node {
                    width: Val::Percent(100.),
                    ..Default::default()
                }
            ),
            (
                Node {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Percent(2.),
                    ..Default::default()
                },
                DialogueArea
            )
        ],
    ));
}

fn say_hi(mut commands: Commands) {
    commands.trigger(ClearSections);
    commands.trigger(PushSections::new(sections!(
        { "Hello there. " }
        <i> { "I'm italic now! " } </i>
        <b
            click = { Observer::new(tada) }
            over = { Observer::new(set_text_color_on::<Over>(HOVERED_COLOR)) }
            out = { Observer::new(set_text_color_on::<Out>(NORMAL_COLOR)) }
        > { "You should click on me" } </b>
    )));
}

fn clear(mut commands: Commands) {
    commands.trigger(ClearSections);
}
