use crate::{authenticated::UserExtension, SharedState};
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    Extension,
};
use log::debug;
use log::error;
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use tera::Context;

#[derive(Serialize, Deserialize)]
struct Account {
    name: String,
    amount: f64,
    _id: ObjectId,
}

#[derive(Serialize)]
struct AccountRecord {
    name: String,
    amount: f64,
    id: String,
}

pub async fn page(
    shared_state: State<SharedState>,
    user: Extension<UserExtension>,
) -> Result<Response, StatusCode> {
    debug!("{:?}", user);

    let Ok(user_id) = ObjectId::from_str(&user.id) else {
        return Err(StatusCode::FORBIDDEN);
    };

    let collection = shared_state
        .mongo
        .database("simple_budget")
        .collection::<Account>("accounts");

    let mut context = Context::new();
    let mut accounts: Vec<AccountRecord> = Vec::new();

    match collection.find(doc! {"user_id": &user_id}, None).await {
        Ok(mut cursor) => {
            while cursor.advance().await.unwrap() {
                match cursor.deserialize_current() {
                    Ok(account) => {
                        accounts.push(AccountRecord {
                            name: account.name,
                            amount: account.amount,
                            id: account._id.to_string(),
                        });
                    }
                    Err(e) => {
                        log::error!("{}", e);
                    }
                }
            }
            context.insert("accounts", &accounts);
        }
        Err(e) => {
            error!("{}", e);
            context.insert("accounts", &accounts);
        }
    }

    let Ok(content) = shared_state.tera.render("accounts/index.html", &context) else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    Ok(Html::from(content).into_response())
}
