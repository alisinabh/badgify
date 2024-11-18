pub mod shields_io;

use crate::data_source::SourceResponseWithMetadata;

#[derive(Debug)]
pub enum Logo {
    Slug(String),
}

pub struct Badge {
    pub color: Option<String>,
    pub label: Option<String>,
    pub label_color: Option<String>,
    pub logo: Option<Logo>,
    pub message: String,
    pub suffix: Option<String>,
}

impl Badge {
    pub fn new(message: &str) -> Self {
        Badge {
            color: None,
            label: None,
            label_color: None,
            logo: None,
            message: message.to_string(),
            suffix: None,
        }
    }
}

impl From<&SourceResponseWithMetadata> for Badge {
    fn from(value: &SourceResponseWithMetadata) -> Self {
        let mut badge = Badge::new(&value.result.formatted_tiny());

        badge.label = value.metadata.label();
        badge.logo = value.metadata.logo().map(Logo::Slug);
        badge.suffix = Some(value.metadata.symbol());

        badge
    }
}
