use actix_web::{get, web, HttpResponse, Responder};
use actix_session::Session;
use crate::providers::auth_provider::AuthProvider;

#[utoipa::path(
    get,
    path = "/api/authentication/login",
    responses(
        (
            status = FOUND,
            description = "Redirects user to external authentication provider",
            headers(
                (
                    "Location" = String,
                    description = "Authentication provider redirect URL"
                )
            )
        ),
        (
            status = INTERNAL_SERVER_ERROR,
            description = "Failed to generate authentication challenge URL"
        )
    ),
    tag = "Authentication"
)]
#[get("api/authentication/login")]
pub async fn login(auth_provider: web::Data<AuthProvider>, session: Session) -> impl Responder {

    let challenge_dto = match auth_provider.get_auth_url(){
        Ok(dto) => dto,
        Err(e) => return HttpResponse::InternalServerError().body(e.0),
    };
    session.insert("pkce_code_verifier", challenge_dto.pkce_code_verifier).expect("session couldn't be added to pkce_code_verifier");
    session.insert("csrf_token", challenge_dto.csrf_token).expect("session couldn't be added to csrf_token");

    HttpResponse::Found()
        .append_header(("Location", challenge_dto.auth_url.to_string())).finish()
}