use crate::{authenticated::UserExtension, SharedState};
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    Extension,
};
use bson::{doc, oid::ObjectId, serde_helpers::hex_string_as_object_id};
use serde::{Deserialize, Serialize};
use tera::Context;

#[derive(Deserialize)]
pub struct Goal {
    #[serde(with = "hex_string_as_object_id")]
    _id: String,
    name: String,
    target: f64,
}

#[derive(Serialize)]
pub struct GoalRecord {
    name: String,
    target: f64,
    id: String,
}

pub async fn page(
    shared_state: State<SharedState>,
    user: Extension<UserExtension>,
) -> Result<Response, StatusCode> {
    log::debug!("{:?}", user);
    let Ok(user_id) = ObjectId::parse_str(&user.id) else {
        return Err(StatusCode::FORBIDDEN);
    };

    let collection = shared_state
        .mongo
        .database("simple_budget")
        .collection::<Goal>("goals");

    let mut context = Context::new();
    let mut goals: Vec<GoalRecord> = Vec::new();

    match collection.find(doc! {"user_id": &user_id}, None).await {
        Ok(mut cursor) => {
            while cursor.advance().await.unwrap() {
                match cursor.deserialize_current() {
                    Ok(goal) => {
                        goals.push(GoalRecord {
                            name: goal.name,
                            target: goal.target,
                            id: goal._id,
                        });
                    }
                    Err(e) => {
                        log::error!("{}", e);
                    }
                }
            }
            context.insert("goals", &goals);
        }
        Err(e) => {
            log::error!("{}", e);
            context.insert("goals", &goals);
        }
    }

    let Ok(content) = shared_state.tera.render("goals/index.html", &context) else {
        log::error!("could not render template");
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    Ok(Html::from(content).into_response())
}
