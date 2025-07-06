#![allow(unused_imports)]
#![allow(dead_code)]

use std::{convert::Infallible, sync::Arc};
use axum::{extract::{FromRequestParts, Path}, http::StatusCode, routing::get, Json, Router};
use serde::{Serialize, Deserialize};
use tokio::net::TcpListener;
use tokio_postgres::Statement;

#[derive(Debug, Serialize)]
struct User {
    id: i32,
    name: String,
    email: String,
    password: String,
    trang_thai: String,
    created_at: String,
    updated_at: String
}

#[derive(Debug, Deserialize)]
struct UserPayload {
    name: String,
    email: String,
    password: String
}

#[derive(Debug, Serialize)]
struct Book {
    id: i32,
    title: String,
    author: String,
    content: String,
    category: String,
    publish_date: String,
}

#[derive(Debug, Deserialize)]
struct BookPayload {
    title: String,
    author: String,
    content: String,
    category: String,
    publish_date: String,
}

#[derive(Debug, Serialize)]
struct Borrow {
    username: String,
    title: String,
    borrow_date: String
}

#[derive(Debug, Deserialize)]
struct BorrowPayload {
    user_id: i32,
    book_id: i32,
    borrow_date: String
}

struct PgConn {
    client : tokio_postgres::Client,
    l_users: Statement,
    l_user : Statement,
    i_user : Statement,
    u_user : Statement,
    d_user : Statement,

    l_books: Statement,
    l_book : Statement,
    i_book : Statement,
    u_book : Statement,
    d_book : Statement,

    l_borrows        : Statement,
    l_borrow         : Statement,
    l_borrows_by_user: Statement,
    l_borrows_by_book: Statement,
    i_borrow         : Statement
}

struct AppState(Arc<PgConn>);

impl FromRequestParts<Arc<PgConn>> for AppState {
    type Rejection = Infallible;

    async fn from_request_parts(_parts: &mut axum::http::request::Parts, state: &Arc<PgConn>) -> std::result::Result<Self, Self::Rejection> {
        Ok(Self(state.clone()))
    }
}

async fn list_users(AppState(state): AppState) -> Json<Vec<User>> {
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

async fn list_user(AppState(state): AppState, Path(id): Path<i32>) -> Json<User> {
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

async fn insert_user(AppState(state): AppState, Json(payload): Json<UserPayload>) -> StatusCode {
    state.client.execute(&state.i_user, &[&payload.name, &payload.email, &payload.password]).await.unwrap();

    StatusCode::CREATED
}

async fn update_user(
    AppState(state): AppState, 
    Path(id): Path<i32>, 
    Json(payload): Json<UserPayload>
) -> StatusCode {
    state.client.execute(&state.u_user, &[&payload.name, &payload.email, &payload.password, &id]).await.unwrap();

    StatusCode::OK
}

async fn delete_user(AppState(state): AppState, Path(id): Path<i32>) -> StatusCode {
    state.client.execute(&state.d_user, &[&id]).await.unwrap();

    StatusCode::OK
}

async fn list_books(AppState(state): AppState) -> Json<Vec<Book>> {
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

async fn list_book(AppState(state): AppState, Path(id): Path<i32>) -> Json<Book> {
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

async fn insert_book(AppState(state): AppState, Json(payload): Json<BookPayload>) -> StatusCode {
    state.client.execute(&state.i_book, &[&payload.title, &payload.author, &payload.content, &payload.category, &payload.publish_date]).await.unwrap();

    StatusCode::CREATED
}

async fn update_book(
    AppState(state): AppState, 
    Path(id): Path<i32>, 
    Json(payload): Json<BookPayload>
) -> StatusCode {
    state.client.execute(&state.u_book, &[&payload.title, &payload.author, &payload.content, &payload.category, &payload.publish_date, &id]).await.unwrap();

    StatusCode::OK
}

async fn delete_book(AppState(state): AppState, Path(id): Path<i32>) -> StatusCode {
    state.client.execute(&state.d_book, &[&id]).await.unwrap();

    StatusCode::OK
}

async fn list_borrows(AppState(state): AppState) -> Json<Vec<Borrow>> {
    let rows = state.client.query(&state.l_borrows, &[]).await.unwrap();
    let borrows = rows.iter().map(|row| Borrow {
        username: row.get(0),
        title: row.get(1),
        borrow_date: row.get(2)
    }).collect();
    
    Json(borrows)
}

async fn list_borrow(AppState(state): AppState, Path(user_id): Path<i32>, Path(book_id): Path<i32>) -> Json<Borrow> {
    let row = state.client.query_one(&state.l_borrow, &[&user_id, &book_id]).await.unwrap();
    let borrow = Borrow {
        username: row.get(0),
        title: row.get(1),
        borrow_date: row.get(2)
    };
    
    Json(borrow)
}

async fn list_borrows_by_user(AppState(state): AppState, Path(id): Path<i32>) -> Json<Vec<Borrow>> {
    let rows = state.client.query(&state.l_borrows_by_user, &[&id]).await.unwrap();
    let borrows = rows.iter().map(|row| Borrow {
        username: row.get(0),
        title: row.get(1),
        borrow_date: row.get(2)
    }).collect();
    
    Json(borrows)
}

async fn list_borrows_by_book(AppState(state): AppState, Path(id): Path<i32>) -> Json<Vec<Borrow>> {
    let rows = state.client.query(&state.l_borrows_by_book, &[&id]).await.unwrap();
    let borrows = rows.iter().map(|row| Borrow {
        username: row.get(0),
        title: row.get(1),
        borrow_date: row.get(2)
    }).collect();
    
    Json(borrows)
}

async fn insert_borrow(AppState(state): AppState, Json(payload): Json<BorrowPayload>) -> StatusCode {
    state.client.execute(&state.i_borrow, &[&payload.user_id, &payload.book_id, &payload.borrow_date]).await.unwrap();

    StatusCode::CREATED
}


#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Database URL: {}", database_url);
    let (client, conn) = tokio_postgres::connect(&database_url, tokio_postgres::NoTls).await.unwrap();

    tokio::spawn(async move {
        if let Err(e) = conn.await {
            eprintln!("Connection error: {}", e);
        }
    });
    
    let l_users = client.prepare("SELECT * FROM \"USERS\"").await.unwrap();
    let l_user = client.prepare("SELECT * FROM \"USERS\" WHERE id = $1").await.unwrap();
    let i_user = client.prepare("INSERT INTO \"USERS\" (name, email, password) VALUES ($1, $2, $3)").await.unwrap();
    let u_user = client.prepare("UPDATE \"USERS\" SET name = $1, email = $2, password = $3 WHERE id = $4").await.unwrap();
    let d_user = client.prepare("DELETE FROM \"USERS\" WHERE id = $1").await.unwrap();

    let l_books = client.prepare("SELECT * FROM \"BOOKS\"").await.unwrap();
    let l_book = client.prepare("SELECT * FROM \"BOOKS\" WHERE id = $1").await.unwrap();
    let i_book = client.prepare("INSERT INTO \"BOOKS\" (title, author, content, category, publish_date) VALUES ($1, $2, $3, $4, $5)").await.unwrap();
    let u_book = client.prepare("UPDATE \"BOOKS\" SET title = $1, author = $2, content = $3, category = $4, publish_date = $5 WHERE id = $6").await.unwrap();
    let d_book = client.prepare("DELETE FROM \"BOOKS\" WHERE id = $1").await.unwrap();

    let l_borrows = client.prepare("SELECT username, title, borrow_date FROM \"BORROW\" AS br JOIN \"USERS\" AS u ON br.user_id = u.id JOIN \"BOOKS\" AS bo ON br.book_id = bo.id").await.unwrap();
    let l_borrow = client.prepare("SELECT username, title, borrow_date FROM \"BORROW\" AS br JOIN \"USERS\" AS u ON br.user_id = u.id JOIN \"BOOKS\" AS bo ON br.book_id = bo.id WHERE br.user_id = $1 AND br.book_id = $2").await.unwrap();
    let l_borrows_by_user = client.prepare("SELECT username, title, borrow_date FROM \"BORROW\" AS br JOIN \"USERS\" AS u ON br.user_id = u.id JOIN \"BOOKS\" AS bo ON br.book_id = bo.id WHERE br.user_id = $1").await.unwrap();
    let l_borrows_by_book = client.prepare("SELECT username, title, borrow_date FROM \"BORROW\" AS br JOIN \"USERS\" AS u ON br.user_id = u.id JOIN \"BOOKS\" AS bo ON br.book_id = bo.id WHERE br.book_id = $1").await.unwrap();
    let i_borrow = client.prepare("INSERT INTO \"BORROW\" (user_id, book_id, borrow_date) VALUES ($1, $2, $3)").await.unwrap();

    let state = Arc::new(PgConn {
        client,
        l_users,
        l_user,
        i_user,
        u_user,
        d_user,

        l_books,
        l_book,
        i_book,
        u_book,
        d_book,

        l_borrows,
        l_borrow,
        l_borrows_by_user,
        l_borrows_by_book,
        i_borrow
    });

    let app = Router::new()
        .route(
            "/api/users",
            get(list_users).post(insert_user)
        )
        .route(
            "/api/users/:id",
            get(list_user).put(update_user).delete(delete_user)
        )
        .route(
            "/api/books",
            get(list_books).post(insert_book)
        )
        .route(
            "/api/books/:id",
            get(list_book).put(update_book).delete(delete_book)
        )
        .route(
            "/api/borrows",
            get(list_borrows).post(insert_borrow)
        )
        .route(
            "/api/borrows/:user_id/:book_id",
            get(list_borrow)
        )
        .route(
            "/api/books/:user_id",
            get(list_borrows_by_user)
        )
        .route(
            "/api/users/:book_id",
            get(list_borrows_by_book)
        )
        .with_state(state);
    
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    
    println!("Listening on http://localhost:8080/api/users");
    axum::serve(listener, app).await.unwrap();
}
