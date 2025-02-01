use super::{Badge, Logo};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ShildsIoBadgeData {
    #[serde(rename = "schemaVersion")]
    schema_version: u8,
    label: String,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,
    #[serde(rename = "labelColor", skip_serializing_if = "Option::is_none")]
    label_color: Option<String>,
    #[serde(rename = "isError")]
    is_error: bool,
    #[serde(rename = "namedLogo", skip_serializing_if = "Option::is_none")]
    named_logo: Option<String>,
    #[serde(rename = "logoSvg", skip_serializing_if = "Option::is_none")]
    logo_svg: Option<String>,
    #[serde(rename = "logoColor", skip_serializing_if = "Option::is_none")]
    logo_color: Option<String>,
    #[serde(rename = "logoSize", skip_serializing_if = "Option::is_none")]
    logo_size: Option<String>,
    #[serde(rename = "logoWidth", skip_serializing_if = "Option::is_none")]
    logo_width: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<String>,
}

impl From<Badge> for ShildsIoBadgeData {
    fn from(value: Badge) -> Self {
        let label = value.label.clone().unwrap_or_default();

        ShildsIoBadgeData {
            schema_version: 1,
            label,
            message: value.message,
            color: Some(value.color.unwrap_or("blue".to_string())),
            label_color: value.label_color,
            is_error: false,
            named_logo: value.icon.map(from_badge_logo),
            logo_svg: None,
            logo_color: None,
            logo_size: None,
            logo_width: None,
            style: None,
        }
    }
}

fn from_badge_logo(logo: Logo) -> String {
    match logo {
        Logo::Slug(slug) => slug.to_string(),
    }
}
