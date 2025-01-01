pub mod plugin;

pub mod prelude {
    pub use crate::plugin::{
        ClearSections, DefaultHtmlLiteDisplayPlugin, DialogueArea, DialogueSection, HtmlLiteFonts,
        PushSections,
    };
    pub use html_lite_macros::sections;
    pub use html_lite_sections::prelude::{Section, Sections};
}
