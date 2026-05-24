use actix_web::{get, web, HttpResponse, Responder};
use actix_session::Session;
use oauth2::{CsrfToken, PkceCodeVerifier, TokenResponse};
use crate::providers::auth_provider::AuthProvider;

#[derive(serde::Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
pub struct CallbackRequest {
    pub code: String,
    pub state: String,
}

#[utoipa::path(
    get,
    path = "/api/authentication/callback",
    params(CallbackRequest),
    responses(
        (
            status = OK,
            description = "Authentication completed successfully"
        ),
        (
            status = FORBIDDEN,
            description = "CSRF secret mismatch"
        ),
        (
            status = INTERNAL_SERVER_ERROR,
            description = "Failed to complete authentication flow"
        )
    ),
    tag = "Authentication"
)]
#[get("api/authentication/callback")]
pub async fn callback(auth_provider: web::Data<AuthProvider>, session: Session, query: web::Query<CallbackRequest>) -> impl Responder
{
    let pkce_code_verifier = session.get::<PkceCodeVerifier>("pkce_code_verifier").expect("Failed to get pkce_code_verifier").unwrap();
    let csrf_token = session.get::<CsrfToken>("csrf_token").expect("Failed to get csrf_token").unwrap();
    
    if csrf_token.into_secret() != query.state
    {
        return HttpResponse::Forbidden().body("CSRF secret mismatch")
    }

    let token_result = match auth_provider.get_tokens(pkce_code_verifier, &query.code).await {
        Ok(token) => token,
        Err(e) => return HttpResponse::InternalServerError().body(e.0)
    };
    
    let access_token = token_result.access_token().clone().into_secret();

    session.insert("access_token", access_token).expect("Failed to set access token");

    HttpResponse::Ok().body(token_result.access_token().clone().into_secret())
}