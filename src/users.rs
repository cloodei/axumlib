use axum::{extract::Path, http::StatusCode, Json};
use crate::prelude::{AppState, User, UserPayload};

pub async fn list_users(AppState(state): AppState) -> Json<Vec<User>> {
    let rows = state.client.query(&state.l_users, &[]).await.unwrap();
    let users = rows.iter().map(|row| User {
        id: row.get(0),
        name: row.get(1),
        email: row.get(2),
        password: row.get(3),
        trang_thai: row.get(4),
        created_at: row.get(5),
        updated_at: row.get(6)
    }).collect();
    
    Json(users)
}

pub async fn list_user(AppState(state): AppState, Path(id): Path<i32>) -> Json<User> {
    let row = state.client.query_one(&state.l_user, &[&id]).await.unwrap();
    let user = User {
        id: row.get(0),
        name: row.get(1),
        email: row.get(2),
        password: row.get(3),
        trang_thai: row.get(4),
        created_at: row.get(5),
        updated_at: row.get(6)
    };
    
    Json(user)
}

pub async fn insert_user(AppState(state): AppState, Json(payload): Json<UserPayload>) -> StatusCode {
    state.client.execute(&state.i_user, &[&payload.name, &payload.email, &payload.password]).await.unwrap();

    StatusCode::CREATED
}

pub async fn update_user(
    AppState(state): AppState, 
    Path(id): Path<i32>, 
    Json(payload): Json<UserPayload>
) -> StatusCode {
    state.client.execute(&state.u_user, &[&payload.name, &payload.email, &payload.password, &id]).await.unwrap();

    StatusCode::OK
}

pub async fn delete_user(AppState(state): AppState, Path(id): Path<i32>) -> StatusCode {
    state.client.execute(&state.d_user, &[&id]).await.unwrap();

    StatusCode::OK
}
