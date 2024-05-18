use std::error::Error;

use async_graphql::{
    http::GraphiQLSource, Context, EmptyMutation, EmptySubscription, Enum, FieldResult, Object,
    Schema, SimpleObject,
};
use async_graphql_poem::*;
use chrono::{DateTime, Utc};
use poem::{listener::TcpListener, web::Html, *};

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
enum SportId {
    #[graphql(name = "ARCHERY")]
    Archery,
    #[graphql(name = "SPORT_CLIMBING")]
    SportClimbing,
}

#[derive(SimpleObject)]
struct Location {
    id: i32,
    name: String,
    events: Vec<Event>,
}

#[derive(SimpleObject)]
struct Sport {
    id: SportId,
    name: String,
    events: Vec<Event>,
}

#[derive(SimpleObject)]
struct Event {
    sport: Sport,
    name: String,
    time: DateTime<Utc>,
    location: Location,
}

#[derive(SimpleObject)]
struct Day {
    events: Vec<Event>,
}

struct Query;

#[Object]
impl Query {
    async fn events(&self, _ctx: &Context<'_>) -> FieldResult<Vec<Event>> {
        Ok(vec![])
    }

    async fn event(&self, _ctx: &Context<'_>) -> FieldResult<Event> {
        Ok(Event {
            sport: Sport {
                id: SportId::Archery,
                name: "Archery".to_string(),
                events: vec![],
            },
            name: "Event Name".to_string(),
            time: Utc::now(),
            location: Location {
                id: 1,
                name: "Location Name".to_string(),
                events: vec![],
            },
        })
    }

    async fn sports(&self, _ctx: &Context<'_>) -> FieldResult<Vec<Sport>> {
        Ok(vec![])
    }

    async fn sport(&self, _ctx: &Context<'_>, id: SportId) -> FieldResult<Sport> {
        Ok(Sport {
            id,
            name: "Sport Name".to_string(),
            events: vec![],
        })
    }

    async fn schedule(&self, _ctx: &Context<'_>) -> FieldResult<Day> {
        Ok(Day { events: vec![] })
    }
}

#[handler]
async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().finish())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // create the schema
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();

    // start the http server
    let app = Route::new().at("/", get(graphiql).post(GraphQL::new(schema)));
    println!("GraphiQL: http://localhost:8000");
    Server::new(TcpListener::bind("0.0.0.0:8000"))
        .run(app)
        .await?;
    Ok(())
}
