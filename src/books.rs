use axum::{extract::Path, Json};
use axum::http::StatusCode;
use crate::prelude::{AppState, Book, BookPayload};

pub async fn list_books(AppState(state): AppState) -> Json<Vec<Book>> {
    let rows = state.client.query(&state.l_books, &[]).await.unwrap();
    let books = rows.iter().map(|row| Book {
        id: row.get(0),
        title: row.get(1),
        author: row.get(2),
        content: row.get(3),
        category: row.get(4),
        publish_date: row.get(5)
    }).collect();
    
    Json(books)
}

pub async fn list_book(AppState(state): AppState, Path(id): Path<i32>) -> Json<Book> {
    let row = state.client.query_one(&state.l_book, &[&id]).await.unwrap();
    let book = Book {
        id: row.get(0),
        title: row.get(1),
        author: row.get(2),
        content: row.get(3),
        category: row.get(4),
        publish_date: row.get(5)
    };
    
    Json(book)
}

pub async fn insert_book(AppState(state): AppState, Json(payload): Json<BookPayload>) -> StatusCode {
    state.client.execute(&state.i_book, &[&payload.title, &payload.author, &payload.content, &payload.category, &payload.publish_date]).await.unwrap();

    StatusCode::CREATED
}

pub async fn update_book(
    AppState(state): AppState, 
    Path(id): Path<i32>, 
    Json(payload): Json<BookPayload>
) -> StatusCode {
    state.client.execute(&state.u_book, &[&payload.title, &payload.author, &payload.content, &payload.category, &payload.publish_date, &id]).await.unwrap();

    StatusCode::OK
}

pub async fn delete_book(AppState(state): AppState, Path(id): Path<i32>) -> StatusCode {
    state.client.execute(&state.d_book, &[&id]).await.unwrap();

    StatusCode::OK
}
