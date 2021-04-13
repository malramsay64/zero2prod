use crate::helpers::spawn_app;

#[actix_rt::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app = spawn_app().await;

    let test_cases = vec![
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both the name and email"),
        ("name=le%20guin", "missing the email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = app.post_subscriptions(invalid_body.into()).await;

        dbg!(&response);
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 bad request when the payload was {}.",
            error_message
        )
    }
}

#[actix_rt::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app = spawn_app().await;

    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    // Act
    let response = app.post_subscriptions(body.into()).await;

    dbg!(&response);

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");
}

#[actix_rt::test]
async fn subscribe_returns_a_400_when_fields_are_present_but_invalid() {
    let app = spawn_app().await;

    let test_cases = vec![
        ("name=&email=ursula_le_guin%40gmail.com", "empty name"),
        ("name=Ursula&email=", "empty email"),
        ("name=Ursula&email=definitely-not-an-email", "invalid email"),
    ];

    for (body, description) in test_cases {
        let response = app.post_subscriptions(body.into()).await;

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 bad request when the payload was {}.",
            description
        );
    }
}