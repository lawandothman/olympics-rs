use async_graphql::{Context, FieldResult, Object};
use chrono::Utc;

use crate::schema::{Day, Event, Location, Sport, SportId};

pub struct Query;

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
