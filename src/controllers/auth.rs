use crate::models::user;
use loco_rs::{auth::jwt, hash, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PasswordLoginParams {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub pid: String,
    pub name: String,
    pub is_verified: bool,
}

impl LoginResponse {
    pub fn new(user: &user::Model, token: &String) -> Self {
        Self {
            token: token.to_string(),
            pid: user.pid.to_string(),
            name: user.name.clone(),
            is_verified: true,
        }
    }
}

async fn login(
    State(ctx): State<AppContext>,
    Json(params): Json<PasswordLoginParams>,
) -> Result<Response> {
    // Find user by email
    let user = user::Entity::find()
        .filter(user::Column::Email.eq(&params.email))
        .one(&ctx.db)
        .await?;
    let Some(user) = user else {
        return unauthorized("unauthorized!");
    };

    // Verify password
    if !hash::verify_password(&params.password, &user.password) {
        return unauthorized("unauthorized!");
    }

    // Generate the JWT
    let jwt_secret = ctx.config.get_jwt_config()?;
    let token = jwt::JWT::new(&jwt_secret.secret)
        .generate_token(
            jwt_secret.expiration,
            params.email.to_string(),
            Default::default(),
        )
        .unwrap();

    // Login success
    format::json(LoginResponse::new(&user, &token))
}

pub fn routes() -> Routes {
    Routes::new()
        // Authentication route prefix
        .prefix("auth")
        // Handling login with password
        .add("/login", post(login))
}
