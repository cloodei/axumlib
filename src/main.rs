use std::sync::Arc;
use axum::routing::get;
use axum::Router;
use tokio::net::TcpListener;

use crate::prelude::PgConn;
use crate::users::*;
use crate::books::*;
use crate::borrow::*;

pub mod prelude;
mod users;
mod books;
mod borrow;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");

    let (client, conn) = tokio_postgres::connect(&database_url, tokio_postgres::NoTls).await.unwrap();
    let redis = redis::Client::open(redis_url).unwrap();
    
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

    let l_borrows = client.prepare(
        "SELECT u.name AS username, bo.title, br.borrow_date FROM \"BORROW\" AS br JOIN \"USERS\" AS u ON br.user_id = u.id JOIN \"BOOKS\" AS bo ON br.book_id = bo.id"
    ).await.unwrap();
    let l_borrow = client.prepare(
        "SELECT u.name AS username, bo.title, br.borrow_date FROM \"BORROW\" AS br JOIN \"USERS\" AS u ON br.user_id = u.id JOIN \"BOOKS\" AS bo ON br.book_id = bo.id WHERE br.user_id = $1 AND br.book_id = $2"
    ).await.unwrap();
    let l_borrows_by_user = client.prepare(
        "SELECT u.name AS username, bo.title, br.borrow_date FROM \"BORROW\" AS br JOIN \"USERS\" AS u ON br.user_id = u.id JOIN \"BOOKS\" AS bo ON br.book_id = bo.id WHERE br.user_id = $1"
    ).await.unwrap();
    let l_borrows_by_book = client.prepare(
        "SELECT u.name AS username, bo.title, br.borrow_date FROM \"BORROW\" AS br JOIN \"USERS\" AS u ON br.user_id = u.id JOIN \"BOOKS\" AS bo ON br.book_id = bo.id WHERE br.book_id = $1"
    ).await.unwrap();
    let i_borrow = client.prepare("INSERT INTO \"BORROW\" (user_id, book_id, borrow_date) VALUES ($1, $2, $3)").await.unwrap();

    let state = Arc::new(PgConn {
        client,
        redis,
        
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
            "/api/users/{id}",
            get(list_user).put(update_user).delete(delete_user)
        )

        .route(
            "/api/books",
            get(list_books).post(insert_book)
        )
        .route(
            "/api/books/{id}",
            get(list_book).put(update_book).delete(delete_book)
        )

        .route(
            "/api/borrows",
            get(list_borrows).post(insert_borrow)
        )
        .route(
            "/api/borrows/{user_id}/{book_id}",
            get(list_borrow)
        )
        .route(
            "/api/borrows/books/{user_id}",
            get(list_borrows_by_user)
        )
        .route(
            "/api/borrows/users/{book_id}",
            get(list_borrows_by_book)
        )
        .with_state(state);
    
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    
    println!("Listening on http://localhost:8080/api/users");
    axum::serve(listener, app).await.unwrap();
}
