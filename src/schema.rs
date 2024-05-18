use async_graphql::{Enum, SimpleObject};
use chrono::{DateTime, Utc};

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum SportId {
    AcrobaticGymnastics,
    AlpineSkiing,
    Archery,
    ArtisticGymanstics,
    ArtisticSwimming,
    Athletics,
    Badminton,
    BaseballSoftball,
    Basketball,
    #[graphql(name = "BASKETBALL_3X3")]
    Basketball3x3,
    BeachHandball,
    BeachVolleyball,
    Biathlon,
    Bobsleigh,
    Boxing,
    Breaking,
    CanoeSlalom,
    CanoeSprint,
    Cricket,
    CrossCountrySkiing,
    Curling,
    CyclingBMXFreestyle,
    CyclingBMXRacing,
    CyclingMountainBike,
    CyclingRoad,
    CyclingTrack,
    Diving,
    Equestrian,
    Fencing,
    FigureSkating,
    FlagFootball,
    Football,
    FreestyleSkiing,
    Futsal,
    Golf,
    Handball,
    Hockey,
    IceHockey,
    Judo,
    Karate,
    Lacrosse,
    Luge,
    MarathonSwimming,
    ModernPentathlon,
    NordicCombined,
    RhthmicGymnastics,
    RollerSpeedSkating,
    Rowing,
    RugbySevens,
    Sailing,
    Shooting,
    ShortTrackSpeedSkating,
    Skateboarding,
    Skeleton,
    SkiJumping,
    SkiMountaineering,
    Snowboard,
    SpeedSkating,
    SportClimbing,
    Squash,
    Surfing,
    Swimming,
    TableTennis,
    Taekwondo,
    Tennis,
    Trampoline,
    Triathlon,
    Volleyball,
    WaterPolo,
    Weightlifting,
    Wrestling,
}

#[derive(SimpleObject)]
pub struct Location {
    pub id: i32,
    pub name: String,
    pub events: Vec<Event>,
}

#[derive(SimpleObject)]
pub struct Sport {
    pub id: SportId,
    pub name: String,
    pub events: Vec<Event>,
}

#[derive(SimpleObject)]
pub struct Event {
    pub sport: Sport,
    pub name: String,
    pub time: DateTime<Utc>,
    pub location: Location,
}

#[derive(SimpleObject)]
pub struct Day {
    pub events: Vec<Event>,
}
