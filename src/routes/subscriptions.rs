use actix_web::{web, HttpResponse};
use prisma_client_rust::QueryError;
use uuid::Uuid;

use crate::{
    domain::{NewSubscriber, SubscriberEmail, SubscriberName},
    prisma::{subscriptions::Data, PrismaClient},
};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, prisma_client),
    fields(
        request_id = %Uuid::new_v4(),
        subscriber_email = %form.email,
        subscriber_name= %form.name
    )
)]
pub async fn subscribe(
    form: web::Form<FormData>,
    prisma_client: web::Data<PrismaClient>,
) -> HttpResponse {
    let name = match SubscriberName::parse(form.0.name.clone()) {
        Ok(name) => name,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    let email = match SubscriberEmail::parse(form.0.email) {
        Ok(email) => email,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    let new_subscriber = NewSubscriber { email, name };

    match insert_subscriber(&new_subscriber, &prisma_client).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(new_subscriber, prisma_client)
)]
pub async fn insert_subscriber(
    new_subscriber: &NewSubscriber,
    prisma_client: &PrismaClient,
) -> Result<Data, QueryError> {
    prisma_client
        .subscriptions()
        .create(
            new_subscriber.email.as_ref().to_owned(),
            new_subscriber.name.as_ref().to_owned(),
            vec![],
        )
        .exec()
        .await
}
