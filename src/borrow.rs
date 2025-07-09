use axum::{extract::Path, Json};
use axum::http::StatusCode;
use crate::prelude::{AppState, Borrow, BorrowPayload};

pub async fn list_borrows(AppState(state): AppState) -> Json<Vec<Borrow>> {
    let rows = state.client.query(&state.l_borrows, &[]).await.unwrap();
    let borrows = rows.iter().map(|row| Borrow {
        username: row.get(0),
        title: row.get(1),
        borrow_date: row.get(2)
    }).collect();
    
    Json(borrows)
}

pub async fn list_borrow(AppState(state): AppState, Path(user_id): Path<i32>, Path(book_id): Path<i32>) -> Json<Borrow> {
    let row = state.client.query_one(&state.l_borrow, &[&user_id, &book_id]).await.unwrap();
    let borrow = Borrow {
        username: row.get(0),
        title: row.get(1),
        borrow_date: row.get(2)
    };
    
    Json(borrow)
}

pub async fn list_borrows_by_user(AppState(state): AppState, Path(user_id): Path<i32>) -> Json<Vec<Borrow>> {
    let rows = state.client.query(&state.l_borrows_by_user, &[&user_id]).await.unwrap();
    let borrows = rows.iter().map(|row| Borrow {
        username: row.get(0),
        title: row.get(1),
        borrow_date: row.get(2)
    }).collect();
    
    Json(borrows)
}

pub async fn list_borrows_by_book(AppState(state): AppState, Path(book_id): Path<i32>) -> Json<Vec<Borrow>> {
    let rows = state.client.query(&state.l_borrows_by_book, &[&book_id]).await.unwrap();
    let borrows = rows.iter().map(|row| Borrow {
        username: row.get(0),
        title: row.get(1),
        borrow_date: row.get(2)
    }).collect();
    
    Json(borrows)
}

pub async fn insert_borrow(AppState(state): AppState, Json(payload): Json<BorrowPayload>) -> StatusCode {
    state.client.execute(&state.i_borrow, &[&payload.user_id, &payload.book_id, &payload.borrow_date]).await.unwrap();

    StatusCode::CREATED
}
