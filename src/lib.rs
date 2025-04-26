#[cfg(feature = "plugin")]
pub mod plugin;

pub mod prelude {
    #[cfg(feature = "plugin")]
    pub use crate::plugin::{
        ClearSections, DefaultFontSize, DefaultHtmlLiteDisplayPlugin, DefaultTextColor,
        DialogueArea, DialogueSection, HtmlLiteFonts, PushSections,
    };
    pub use html_lite_macros::sections;
    pub use html_lite_sections::prelude::{Section, Sections};
}
