use crate::*;


// Model MVC       Данные и параметры   
#[derive(Clone, Data, Lens)]
pub struct Composite {
    routes_chain: Vector<RouteChain>,
    routes: Vector<Route>,
    ships: Vector<Ship>,
    containers: Vector<Container>,
    clients: Vector<Client>,
    cargo: Vector<Cargo>,
    agents: Vector<Agent>,
    documents: Vector<Document>,
}





#[derive(Clone, Data)]
pub struct RouteChain {
    id: i32,
    chain: Vector<i32>,       // Vector<Route>
}

#[derive(Clone, Data)]
pub struct Route {
    id: i32,
    from: String,
    to: String,
    ship_id: i32,         // Ship
    ship_type: String,
    containers_list: Vector<i32>,     // Vector<Container>
    total_weight: f64,
    depart_date: String,
    arrive_date: String,
    route_time: i32,
    agents_list: Vector<i32>,      // Vector<Agent>
    income: f64,
    documents: Vector<i32>,      // Vector<Document>
}

// Вспомогательное
#[derive(Clone, Data)]
enum Shiptype {
    Container,
    Tanker,
    Bulker,
    Refriger,
    Other(String)
}

#[derive(Clone, Data)]
pub struct Ship {
    id: i32,
    ttype: Shiptype,
    capacity: f64,
    payload: f64,
}

#[derive(Clone, Data)]
pub struct Container {
    id: i32,
    capacity: f64,
    payload: f64,
    cargo_list: Vector<i32>,     // Vector<Cargo>
    ship_id: Option<i32>,    // Option<Ship>
}

#[derive(Clone, Data)]
pub struct Client {
    id: i32,
    cargo_list: Vector<i32>,     // Vector<Cargo>
    pay: f64,
    route_chain: Option<RouteChain>    // RouteChain
}

#[derive(Clone, Data)]
pub struct Cargo {
    id: i32,
    name: String,
    ttype: String,
    weight: f64,
    count: i32,
    client_id: i32,      // Client
}

#[derive(Clone, Data)]
pub struct Agent {
    id: i32,
    name: String,
}

// Вспомогательное
#[derive(Clone, Data)]
pub enum Doctype {
    DogovorOPerevozke(String, String),   // (from, to)
    Other(String)
}

#[derive(Clone, Data)]
pub struct Document {
    id: i32,
    title: String,
    ttype: Doctype,
    content: String,
}