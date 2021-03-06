use std::time::Instant;

#[derive(Debug, PartialEq, Eq, PartialOrd, Hash, Clone, Copy)]
pub enum Resource {
    FirstAid,
    Soap,
    Candy,
    EnergyDrink,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Action {
    Idle,
    Sleep,
    Eat,
    Clean,
    Train,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Hash, Clone, Copy)]
pub enum State {
    Health,
    Hunger,
    Energy,
    Happiness,
    Cleanliness,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Hash, Clone, Copy)]
pub enum Asset {
    Resource(Resource),
    State(State),
    LifeTime,
    BirthCertificate(Instant),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Exchange {
    LifeTimeForState,
    LifeTimeForHealth,
}
