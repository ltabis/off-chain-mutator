/// Account storing available and held amounts.
#[derive(Debug, serde::Serialize)]
pub struct Account {
    #[serde(rename = "client")]
    pub client_id: u16,
    // TODO: up to four places past the decimal.
    pub available: f32,
    pub held: f32,
    pub total: f32,
    pub locked: bool,
}

impl Account {
    pub fn new(client_id: u16) -> Self {
        Self {
            client_id,
            available: 0.0,
            held: 0.0,
            total: 0.0,
            locked: false,
        }
    }
}
