use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;
// An extension trait to use `graphmemes` method on `String` and `&str`
use unicode_segmentation::UnicodeSegmentation;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

/// This creates a span at the beginning of the function invocation,
/// attaching all args passed to the function to the span's context.
#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pool),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    if !is_valid_name(&form.name) {
        return HttpResponse::BadRequest().finish();
    }
    match insert_subscriber(&pool, &form).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// Returns `true` if the input satisfies all validation constrains
// on subscriber names & `false` otherwise.
pub fn is_valid_name(s: &str) -> bool {
    // `.trim()` returns a view over input `s` without
    // trailing whitespace-like characters.
    // `.is_empty()` checks if the view contains any character.
    let is_empty_or_whitespace = s.trim().is_empty();

    // A graphmeme is defined by the Unicode standard as a "user-perceived" char.
    // `å` is a single grapheme, but it is composed of two characters (`a` and `̊`).
    //
    // `graphmemes` returns an iterator over the graphememes in input `s`.
    // `true` specifies that we want to use the extended graphmeme definition set. (recommended)
    let is_too_long = s.graphemes(true).count() > 256;

    // Iterate over all chars in the input `s` to check if any
    // match a char in the forbidden array.
    let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
    let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));

    // Return `false` if any of our conditions have been violated.
    !(is_empty_or_whitespace || is_too_long || contains_forbidden_characters)
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(form, pool)
)]

pub async fn insert_subscriber(pool: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
        // Using the `?` operator to return early if the function
        // failed, returning a sqlx::Error.
    })?;
    Ok(())
}
