use std::str::FromStr;

use actix_web::{get, http::header::LOCATION, web, HttpResponse, Responder};
use alloy::primitives::U256;
use bigdecimal::{BigDecimal, ParseBigDecimalError};
use num::BigInt;
use serde::{Deserialize, Serialize};

use crate::{
    badge::{shields_io_data::ShildsIoBadgeData, Badge, Logo},
    data_source::{SourceResponse, SourceResponseWithMetadata},
    Executor,
};

const DEFAULT_BELOW_THRESHOLD_COLOR: &str = "yellow";
const DEFAULT_ABOVE_THRESHOLD_COLOR: &str = "blue";

#[derive(Serialize, Deserialize)]
struct BadgeQuery {
    color: Option<String>,
    label: Option<String>,
    icon: Option<String>,
    warning_threshold: Option<String>,
}

#[get("/badge/{badge_query:.*}")]
pub async fn badge_image(
    badge_query: web::Path<String>,
    query: web::Query<BadgeQuery>,
) -> impl Responder {
    let query_string = serde_urlencoded::to_string(&query.into_inner()).unwrap();
    let badge_data_url = format!(
        "https://badgify.io/badge_data/{}?{}",
        badge_query, query_string
    )
    .trim_end_matches(|c| c == '?')
    .to_string();

    HttpResponse::TemporaryRedirect()
        .insert_header((
            LOCATION,
            format!("https://img.shields.io/endpoint?url={}", badge_data_url),
        ))
        .finish()
}

#[get("/badge_data/{badge_query:.*}")]
pub async fn badge_data_api(
    badge_query: web::Path<String>,
    executor: web::Data<Executor>,
    query: web::Query<BadgeQuery>,
) -> impl Responder {
    let Ok(result) = executor.query_data(&badge_query.to_string()).await else {
        return render_failed_badge();
    };

    let mut badge: Badge = Badge::from(&result);

    badge.color = if let Some(color) = &query.color {
        Some(color.to_string())
    } else {
        let Ok(color) = extract_threshold_color(&query, &result) else {
            return render_failed_badge();
        };

        color
    };

    if let Some(label) = &query.label {
        badge.label = Some(label.to_string());
    }

    if let Some(icon) = &query.icon {
        badge.icon = Some(Logo::Slug(icon.to_string()));
    }

    render_badge(badge)
}

pub fn render_failed_badge() -> HttpResponse {
    let mut failure_badge = Badge::new("Failed");
    failure_badge.color = Some("red".to_string());
    failure_badge.label = Some("Badge".to_string());
    failure_badge.is_error = true;
    render_badge(failure_badge)
}

pub fn render_badge(badge_data: Badge) -> HttpResponse {
    let shields_io_badge_data: ShildsIoBadgeData = badge_data.into();
    HttpResponse::Ok().json(shields_io_badge_data)
}

fn parse_decimal(value: U256, decimals: u8) -> BigDecimal {
    let bigint = BigInt::from_bytes_le(num::bigint::Sign::Plus, &value.as_le_bytes());
    BigDecimal::from_bigint(bigint, decimals as i64)
}

fn extract_threshold_color(
    query: &web::Query<BadgeQuery>,
    result: &SourceResponseWithMetadata,
) -> Result<Option<String>, ParseBigDecimalError> {
    match result.result {
        SourceResponse::Decimal { value, decimals } => {
            let decimal_value = parse_decimal(value, decimals);
            let warning_threshold =
                BigDecimal::from_str(query.warning_threshold.as_deref().unwrap_or("0"))?;

            let color = match decimal_value.cmp(&warning_threshold) {
                std::cmp::Ordering::Less => DEFAULT_BELOW_THRESHOLD_COLOR,
                std::cmp::Ordering::Equal => DEFAULT_BELOW_THRESHOLD_COLOR,
                std::cmp::Ordering::Greater => DEFAULT_ABOVE_THRESHOLD_COLOR,
            };

            Ok(Some(color.into()))
        }
        _ => Ok(None),
    }
}
