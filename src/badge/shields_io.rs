use super::{Badge, Logo};

const BASE_URL: &str = "https://img.shields.io/badge";

pub struct ShildsIoBadge {
    pub image_url: String,
}

impl ShildsIoBadge {
    fn new(image_url: String) -> Self {
        Self { image_url }
    }
}

impl From<Badge> for ShildsIoBadge {
    fn from(value: Badge) -> Self {
        let label = value.label.clone().unwrap_or_default();

        let mut image_url: String = format!(
            "{BASE_URL}/{}-{}{}-{}",
            label,
            value.message,
            if value.suffix.is_some() {
                format!(" {}", value.suffix.unwrap())
            } else {
                "".to_string()
            },
            &value.color.unwrap_or("blue".to_string())
        );

        let mut params: Vec<(&str, String)> = Vec::new();

        if let Some(label_color) = &value.label_color {
            params.push(("labelColor", label_color.to_string()));
        }

        if let Some(logo) = &value.logo {
            params.push(("logo", from_badge_logo(logo)));
        }

        if !params.is_empty() {
            let joined = params.iter().fold(String::new(), |mut acc, param| {
                acc.push_str(param.0);
                acc.push('=');
                acc.push_str(&param.1);
                acc
            });

            image_url.push('?');
            image_url.push_str(&joined);
        }

        ShildsIoBadge::new(image_url)
    }
}

fn from_badge_logo(logo: &Logo) -> String {
    match logo {
        Logo::Slug(slug) => slug.to_string(),
    }
}
