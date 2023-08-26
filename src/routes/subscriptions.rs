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

impl TryFrom<FormData> for NewSubscriber {
    type Error = String;

    fn try_from(value: FormData) -> Result<Self, Self::Error> {
        let name = SubscriberName::parse(value.name)?;
        let email = SubscriberEmail::parse(value.email)?;
        Ok(Self { email, name })
    }
}

pub fn parse_subscriber(form: FormData) -> Result<NewSubscriber, String> {
    let name = SubscriberName::parse(form.name)?;
    let email = SubscriberEmail::parse(form.email)?;
    Ok(NewSubscriber { email, name })
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
    let new_subscriber = match form.0.try_into() {
        Ok(form) => form,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };

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
