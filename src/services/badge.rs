use actix_web::{get, http::header::LOCATION, web, HttpResponse, Responder};
use alloy::primitives::U256;
use serde::Deserialize;

use crate::{
    badge::{shields_io::ShildsIoBadge, Badge, Logo},
    data_source::SourceResponse,
    Executor,
};

#[derive(Deserialize)]
struct BadgeQuery {
    color: Option<String>,
    label: Option<String>,
    logo: Option<String>,
    warning_threshold: Option<U256>,
}

#[get("/badge/{badge_query:.*}")]
pub async fn badge(
    badge_query: web::Path<String>,
    executor: web::Data<Executor>,
    query: web::Query<BadgeQuery>,
) -> impl Responder {
    let Ok(result) = executor.query_data(&badge_query.to_string()).await else {
        let mut badge = Badge::new("Failed");
        badge.color = Some("red".to_string());
        badge.label = Some("Badge".to_string());
        return render_badge(badge);
    };

    let mut badge: Badge = Badge::from(&result);

    badge.color = if let Some(color) = &query.color {
        Some(color.to_string())
    } else {
        println!("No color");
        match result.result {
            SourceResponse::Decimal { value, decimals: _ } => {
                println!("Decimal");
                if value.le(&query.warning_threshold.unwrap_or(U256::ZERO)) {
                    Some("yellow".to_string())
                } else {
                    None
                }
            }
            _ => None,
        }
    };

    if let Some(label) = &query.label {
        badge.label = Some(label.to_string());
    }

    if let Some(logo) = &query.logo {
        badge.logo = Some(Logo::Slug(logo.to_string()));
    }

    render_badge(badge)
}

pub fn render_badge(badge_data: Badge) -> impl Responder {
    let shields_io_badge: ShildsIoBadge = badge_data.into();

    HttpResponse::TemporaryRedirect()
        .insert_header((LOCATION, shields_io_badge.image_url))
        .finish()
}
