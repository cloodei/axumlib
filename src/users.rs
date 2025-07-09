use axum::{extract::Path, http::{header, StatusCode}, response::IntoResponse, Json};
use redis::AsyncCommands;
use crate::prelude::{AppState, User, UserPayload};

/// postgres version
// pub async fn list_users(AppState(state): AppState) -> impl IntoResponse {
//     let rows = state.client.query(&state.l_users, &[]).await.unwrap();
//     let users = rows.iter().map(|row| User {
//         id: row.get(0),
//         name: row.get(1),
//         email: row.get(2),
//         password: row.get(3),
//         trang_thai: row.get(4),
//         created_at: row.get(5),
//         updated_at: row.get(6)
//     }).collect::<Vec<User>>();
    
//     Json(users)
// }

pub async fn list_users(AppState(state): AppState) -> impl IntoResponse {
    let mut conn = state.redis.get_multiplexed_async_connection().await.unwrap();
    let cached = conn.get::<&'static str, String>("users:all").await;

    match cached {
        Ok(r) => {
            return ([(header::CONTENT_TYPE, "application/json")], r);
        },
        Err(err) => {
            println!("Error: {}", err.to_string());
        }
    }

    let rows = state.client.query(&state.l_users, &[]).await.unwrap();
    let users = rows.iter().map(|row| User {
        id: row.get(0),
        name: row.get(1),
        email: row.get(2),
        password: row.get(3),
        trang_thai: row.get(4),
        created_at: row.get(5),
        updated_at: row.get(6)
    }).collect::<Vec<User>>();

    let users = serde_json::to_string(&users).unwrap();
    conn.set_ex::<&str, &String, ()>("users:all", &users, 60 * 60).await.unwrap();
    
    ([(header::CONTENT_TYPE, "application/json")], users)
}

/// postgres version
// pub async fn list_user(AppState(state): AppState, Path(id): Path<i32>) -> Json<User> {
//     let row = state.client.query_one(&state.l_user, &[&id]).await.unwrap();
//     let user = User {
//         id: row.get(0),
//         name: row.get(1),
//         email: row.get(2),
//         password: row.get(3),
//         trang_thai: row.get(4),
//         created_at: row.get(5),
//         updated_at: row.get(6)
//     };
    
//     Json(user)
// }

pub async fn list_user(AppState(state): AppState, Path(id): Path<i32>) -> impl IntoResponse {
    let mut conn = state.redis.get_multiplexed_async_connection().await.unwrap();
    let cached = conn.get::<String, String>(format!("user:{}", id)).await;

    match cached {
        Ok(r) => {
            return ([(header::CONTENT_TYPE, "application/json")], r);
        },
        Err(err) => {
            println!("Error: {}", err.to_string());
        }
    }

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
    
    let user = serde_json::to_string(&user).unwrap();
    conn.set_ex::<String, &String, ()>(format!("user:{}", id), &user, 60 * 60).await.unwrap();
    
    ([(header::CONTENT_TYPE, "application/json")], user)
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
