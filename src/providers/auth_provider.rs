use oauth2::{ClientId, ClientSecret, basic::BasicClient, PkceCodeChallenge, CsrfToken, Scope, AuthUrl, TokenUrl, RedirectUrl, url::Url, PkceCodeVerifier, Client, StandardRevocableToken, EndpointSet, EndpointNotSet, AuthorizationCode};
use oauth2::basic::{BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse, BasicTokenResponse};
use crate::providers::common::errors::ProviderError;

type OAuthClient = Client<
    BasicErrorResponse,
    BasicTokenResponse,
    BasicTokenIntrospectionResponse,
    StandardRevocableToken,
    BasicRevocationErrorResponse,
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointSet,
>;


pub struct ChallengeDto
{
    pub auth_url: Url,
    pub csrf_token: CsrfToken,
    pub pkce_code_verifier: PkceCodeVerifier,
}

pub struct AuthProvider
{
    pub client: OAuthClient
}

impl AuthProvider {
    pub fn new(client_id: &str, client_secret: &str, base_url: &str, tenant_id: &str, redirect_url: &str) -> Result<Self, ProviderError> {
        let client = BasicClient::new(ClientId::new(client_id.to_string()))
            .set_client_secret(ClientSecret::new(client_secret.to_string()))
            .set_auth_uri(AuthUrl::new(format!("{base_url}/realms/{tenant_id}/protocol/openid-connect/auth").to_string()).map_err(|e| ProviderError(e.to_string()))?)
            .set_token_uri(TokenUrl::new(format!("{base_url}/realms/{tenant_id}/protocol/openid-connect/token").to_string()).map_err(|e| ProviderError(e.to_string()))?)
            .set_redirect_uri(RedirectUrl::new(redirect_url.to_string()).map_err(|e| ProviderError(e.to_string()))?);
        Ok(AuthProvider {
            client
        })
    }

    pub async fn get_tokens(&self, pkce_code_verifier: PkceCodeVerifier, code: &str) -> Result<BasicTokenResponse, ProviderError>
    {
        let http_client = reqwest::ClientBuilder::new()
            // Following redirects opens the client up to SSRF vulnerabilities.
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .expect("Client should build");

        let token_result = self.client
            .exchange_code(AuthorizationCode::new(code.to_string()))
            .set_pkce_verifier(pkce_code_verifier)
            .request_async(&http_client)
            .await.map_err(|e| ProviderError(e.to_string()))?;

        Ok(token_result)
    }

    pub fn get_auth_url(&self) -> Result<ChallengeDto, ProviderError> {

        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        let (auth_url, csrf_token) = self.client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("basic".to_string()))
            .add_scope(Scope::new("email".to_string()))
            .add_scope(Scope::new("profile".to_string()))
            .set_pkce_challenge(pkce_challenge)
            .url();

        println!("{}", auth_url);

        Ok(ChallengeDto {
            auth_url: auth_url,
            pkce_code_verifier: pkce_verifier,
            csrf_token: csrf_token,
        })
    }
}