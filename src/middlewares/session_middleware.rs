use actix_session::SessionMiddleware;
use actix_session::storage::CookieSessionStore;
use actix_web::cookie::Key;

pub fn build_cookie_session_middleware(secret_key: Key) -> SessionMiddleware<CookieSessionStore>
{
    SessionMiddleware::builder(
        CookieSessionStore::default(),
        secret_key,
    ).cookie_name("hotel-booking".to_string()).build()
}