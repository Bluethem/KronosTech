use axum::{extract::State, http::StatusCode, Json};

use crate::{
    config::AppState,
    models::usuario::{AuthResponse, LoginRequest, RegisterRequest},
    repositories::UsuarioRepository,
    services::AuthService,
};

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let repo = UsuarioRepository::new(&state.db);
    let service = AuthService::new(repo, &state.jwt_secret);

    service
        .register(payload)
        .await
        .map(Json)
        .map_err(|err| (StatusCode::BAD_REQUEST, err.to_string()))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let repo = UsuarioRepository::new(&state.db);
    let service = AuthService::new(repo, &state.jwt_secret);

    service
        .login(payload)
        .await
        .map(Json)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Credenciales inválidas".into()))
}
