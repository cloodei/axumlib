use std::{convert::Infallible, sync::Arc};
use axum::extract::FromRequestParts;
use chrono::NaiveDateTime;
use tokio_postgres::Statement;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub trang_thai: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct UserPayload {
    pub name: String,
    pub email: String,
    pub password: String
}

#[derive(Debug, Serialize)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub content: String,
    pub category: String,
    pub publish_date: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct BookPayload {
    pub title: String,
    pub author: String,
    pub content: String,
    pub category: String,
    pub publish_date: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct Borrow {
    pub username: String,
    pub title: String,
    pub borrow_date: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct BorrowPayload {
    pub user_id: i32,
    pub book_id: i32,
    pub borrow_date: NaiveDateTime,
}

pub struct PgConn {
    pub client : tokio_postgres::Client,
    pub redis  : redis::Client,

    pub l_users: Statement,
    pub l_user : Statement,
    pub i_user : Statement,
    pub u_user : Statement,
    pub d_user : Statement,

    pub l_books: Statement,
    pub l_book : Statement,
    pub i_book : Statement,
    pub u_book : Statement,
    pub d_book : Statement,

    pub l_borrows        : Statement,
    pub l_borrow         : Statement,
    pub l_borrows_by_user: Statement,
    pub l_borrows_by_book: Statement,
    pub i_borrow         : Statement
}

pub struct AppState(pub Arc<PgConn>);

impl FromRequestParts<Arc<PgConn>> for AppState {
    type Rejection = Infallible;

    async fn from_request_parts(_parts: &mut axum::http::request::Parts, state: &Arc<PgConn>) -> std::result::Result<Self, Self::Rejection> {
        Ok(Self(state.clone()))
    }
}
