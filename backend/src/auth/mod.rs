use std::collections::HashMap;
use std::sync::Mutex;

use axum::extract::FromRequestParts;
use axum::extract::{FromRef, Query, State};
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect, Response};
use axum::routing::get;
use axum::{Json, Router};
use axum_extra::extract::cookie::{Cookie, CookieJar};
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EndpointNotSet, EndpointSet,
    RedirectUrl, Scope, StandardErrorResponse, TokenResponse, TokenUrl,
};
use serde::Deserialize;
use sqlx::SqlitePool;
use std::sync::Arc;

use crate::error::ApiError;
use crate::models::{AuthState, AuthUser};

type OAuthClient = oauth2::Client<
    StandardErrorResponse<oauth2::basic::BasicErrorResponseType>,
    oauth2::StandardTokenResponse<oauth2::EmptyExtraTokenFields, oauth2::basic::BasicTokenType>,
    oauth2::StandardTokenIntrospectionResponse<
        oauth2::EmptyExtraTokenFields,
        oauth2::basic::BasicTokenType,
    >,
    oauth2::StandardRevocableToken,
    StandardErrorResponse<oauth2::RevocationErrorResponseType>,
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointSet,
>;

pub struct AppState {
    pub oauth_client: OAuthClient,
    pub editor_emails: Vec<String>,
    pub pool: SqlitePool,
    pub sessions: Mutex<HashMap<String, AuthState>>,
    /// URL to redirect to after login/logout (frontend app URL).
    pub frontend_url: String,
}

#[derive(Deserialize)]
pub struct AuthCallback {
    pub code: String,
    pub state: String,
}

#[derive(Deserialize)]
struct GoogleUserInfo {
    email: String,
    name: String,
    picture: Option<String>,
}

pub fn build_oauth_client() -> OAuthClient {
    let client_id = ClientId::new(std::env::var("GOOGLE_CLIENT_ID").unwrap_or_default());
    let client_secret =
        ClientSecret::new(std::env::var("GOOGLE_CLIENT_SECRET").unwrap_or_default());
    let auth_url =
        AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string()).unwrap();
    let token_url = TokenUrl::new("https://oauth2.googleapis.com/token".to_string()).unwrap();
    let redirect_url = RedirectUrl::new(
        std::env::var("OAUTH_REDIRECT_URL")
            .unwrap_or("http://localhost:8080/api/auth/callback".to_string()),
    )
    .unwrap();

    oauth2::Client::new(client_id)
        .set_client_secret(client_secret)
        .set_auth_uri(auth_url)
        .set_token_uri(token_url)
        .set_redirect_uri(redirect_url)
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/api/auth/login", get(login))
        .route("/api/auth/callback", get(callback))
        .route("/api/auth/logout", get(logout))
        .route("/api/auth/me", get(me))
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) -> impl IntoResponse {
    let (auth_url, csrf_token) = state
        .oauth_client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .url();

    let cookie = Cookie::build(("oauth_state", csrf_token.secret().to_string()))
        .path("/api/auth")
        .http_only(true)
        .same_site(axum_extra::extract::cookie::SameSite::Lax)
        .max_age(cookie::time::Duration::minutes(10))
        .build();

    (jar.add(cookie), Redirect::to(auth_url.as_str()))
}

pub async fn callback(
    State(state): State<Arc<AppState>>,
    mut jar: CookieJar,
    Query(query): Query<AuthCallback>,
) -> Response {
    let expected_state = jar
        .get("oauth_state")
        .and_then(|c| Some(c.value().to_string()));
    jar = jar.remove(
        Cookie::build(("oauth_state", ""))
            .path("/api/auth")
            .max_age(cookie::time::Duration::ZERO)
            .build(),
    );
    if expected_state.as_deref() != Some(query.state.as_str()) {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiError::new("Invalid state parameter")),
        )
            .into_response();
    }

    let http_client = oauth2::reqwest::Client::builder()
        .redirect(oauth2::reqwest::redirect::Policy::none())
        .build()
        .expect("Failed to build HTTP client");

    let token_result = state
        .oauth_client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(&http_client)
        .await;

    let token = match token_result {
        Ok(t) => t,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiError::new(format!("Token exchange failed: {}", e))),
            )
                .into_response();
        }
    };

    let access_token = token.access_token().secret();
    let user_info: GoogleUserInfo = match http_client
        .get("https://www.googleapis.com/oauth2/v3/userinfo")
        .bearer_auth(access_token)
        .send()
        .await
    {
        Ok(resp) => match resp.json().await {
            Ok(info) => info,
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiError::new(format!("Failed to parse user info: {}", e))),
                )
                    .into_response();
            }
        },
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiError::new(format!("Failed to fetch user info: {}", e))),
            )
                .into_response();
        }
    };

    let session_id = uuid::Uuid::new_v4().to_string();
    let auth_state = AuthState {
        email: user_info.email,
        name: user_info.name,
        picture: user_info.picture,
    };

    state
        .sessions
        .lock()
        .unwrap()
        .insert(session_id.clone(), auth_state);

    let cookie = Cookie::build(("session", session_id))
        .path("/")
        .http_only(true)
        .same_site(axum_extra::extract::cookie::SameSite::Lax)
        .build();

    let redirect_url = state.frontend_url.clone();
    (jar.add(cookie), Redirect::to(&redirect_url)).into_response()
}

pub async fn logout(State(state): State<Arc<AppState>>, jar: CookieJar) -> impl IntoResponse {
    let cookie = Cookie::build(("session", ""))
        .path("/")
        .http_only(true)
        .max_age(cookie::time::Duration::ZERO)
        .build();

    (jar.add(cookie), Redirect::to(&state.frontend_url))
}

pub async fn me(
    State(state): State<Arc<AppState>>,
    jar: CookieJar,
) -> impl IntoResponse {
    if let Some(user) = get_current_user(&jar, &state) {
        Json(user).into_response()
    } else {
        Json(Option::<AuthUser>::None).into_response()
    }
}

pub fn get_current_user(jar: &CookieJar, state: &Arc<AppState>) -> Option<AuthUser> {
    let session_id = jar.get("session")?.value().to_string();
    let sessions = state.sessions.lock().ok()?;
    let auth_state = sessions.get(&session_id)?;
    let is_editor = state
        .editor_emails
        .iter()
        .any(|e| e == &auth_state.email);
    Some(AuthUser {
        email: auth_state.email.clone(),
        name: auth_state.name.clone(),
        picture: auth_state.picture.clone(),
        is_editor,
    })
}

pub fn is_editor(jar: &CookieJar, state: &Arc<AppState>) -> bool {
    get_current_user(jar, state)
        .map(|u| u.is_editor)
        .unwrap_or(false)
}

/// Extractor that requires an authenticated user (any logged-in user).
pub struct RequireAuth(pub AuthUser);

impl<S> FromRequestParts<S> for RequireAuth
where
    S: Send + Sync,
    Arc<AppState>: FromRef<S>,
{
    type Rejection = (StatusCode, Json<ApiError>);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let app_state = Arc::from_ref(state);
        let jar = CookieJar::from_request_parts(parts, &())
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiError::new("Failed to read cookies")),
                )
            })?;
        let user = get_current_user(&jar, &app_state).ok_or((
            StatusCode::UNAUTHORIZED,
            Json(ApiError::new("Login required")),
        ))?;
        Ok(RequireAuth(user))
    }
}

/// Extractor that requires an authenticated editor.
pub struct RequireEditor(pub AuthUser);

impl<S> FromRequestParts<S> for RequireEditor
where
    S: Send + Sync,
    Arc<AppState>: FromRef<S>,
{
    type Rejection = (StatusCode, Json<ApiError>);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let app_state = Arc::from_ref(state);
        let jar = CookieJar::from_request_parts(parts, &())
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiError::new("Failed to read cookies")),
                )
            })?;
        let user = get_current_user(&jar, &app_state).ok_or((
            StatusCode::UNAUTHORIZED,
            Json(ApiError::new("Login required")),
        ))?;
        if !user.is_editor {
            return Err((
                StatusCode::FORBIDDEN,
                Json(ApiError::new("Editor access required")),
            ));
        }
        Ok(RequireEditor(user))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::extract::FromRequestParts;
    use axum::http::Request;
    use axum_extra::extract::cookie::CookieJar;
    use std::collections::HashMap;
    use std::sync::Mutex;

    async fn make_state(
        sessions: HashMap<String, AuthState>,
        editor_emails: Vec<String>,
    ) -> Arc<AppState> {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();
        Arc::new(AppState {
            oauth_client: build_oauth_client(),
            editor_emails,
            pool,
            sessions: Mutex::new(sessions),
            frontend_url: "http://localhost:5173".to_string(),
        })
    }

    async fn jar_with_session(session_id: &str) -> CookieJar {
        let req = Request::builder()
            .header("cookie", format!("session={}", session_id))
            .body(())
            .unwrap();
        let (mut parts, _) = req.into_parts();
        CookieJar::from_request_parts(&mut parts, &()).await.unwrap()
    }

    async fn empty_jar() -> CookieJar {
        let req = Request::builder().body(()).unwrap();
        let (mut parts, _) = req.into_parts();
        CookieJar::from_request_parts(&mut parts, &()).await.unwrap()
    }

    #[tokio::test]
    async fn get_current_user_none_without_cookie() {
        let state = make_state(HashMap::new(), vec![]).await;
        let jar = empty_jar().await;
        assert!(get_current_user(&jar, &state).is_none());
    }

    #[tokio::test]
    async fn get_current_user_none_for_invalid_session_id() {
        let state = make_state(HashMap::new(), vec![]).await;
        let jar = jar_with_session("nonexistent-session").await;
        assert!(get_current_user(&jar, &state).is_none());
    }

    #[tokio::test]
    async fn get_current_user_returns_editor_when_email_matches() {
        let mut sessions = HashMap::new();
        sessions.insert(
            "editor-session".to_string(),
            AuthState {
                email: "editor@test.com".to_string(),
                name: "Editor".to_string(),
                picture: None,
            },
        );
        let state = make_state(sessions, vec!["editor@test.com".to_string()]).await;

        let jar = jar_with_session("editor-session").await;
        let user = get_current_user(&jar, &state).unwrap();
        assert_eq!(user.email, "editor@test.com");
        assert_eq!(user.name, "Editor");
        assert!(user.is_editor);
    }

    #[tokio::test]
    async fn get_current_user_returns_viewer_when_email_not_in_editor_list() {
        let mut sessions = HashMap::new();
        sessions.insert(
            "viewer-session".to_string(),
            AuthState {
                email: "viewer@test.com".to_string(),
                name: "Viewer".to_string(),
                picture: None,
            },
        );
        let state = make_state(sessions, vec!["editor@test.com".to_string()]).await;

        let jar = jar_with_session("viewer-session").await;
        let user = get_current_user(&jar, &state).unwrap();
        assert_eq!(user.email, "viewer@test.com");
        assert!(!user.is_editor);
    }

    #[tokio::test]
    async fn is_editor_true_for_editor_session() {
        let mut sessions = HashMap::new();
        sessions.insert(
            "editor-session".to_string(),
            AuthState {
                email: "editor@test.com".to_string(),
                name: "Editor".to_string(),
                picture: None,
            },
        );
        let state = make_state(sessions, vec!["editor@test.com".to_string()]).await;

        let jar = jar_with_session("editor-session").await;
        assert!(is_editor(&jar, &state));
    }

    #[tokio::test]
    async fn is_editor_false_without_session() {
        let state = make_state(HashMap::new(), vec!["editor@test.com".to_string()]).await;
        let jar = empty_jar().await;
        assert!(!is_editor(&jar, &state));
    }

    #[tokio::test]
    async fn is_editor_false_for_viewer_session() {
        let mut sessions = HashMap::new();
        sessions.insert(
            "viewer-session".to_string(),
            AuthState {
                email: "viewer@test.com".to_string(),
                name: "Viewer".to_string(),
                picture: None,
            },
        );
        let state = make_state(sessions, vec!["editor@test.com".to_string()]).await;

        let jar = jar_with_session("viewer-session").await;
        assert!(!is_editor(&jar, &state));
    }
}
