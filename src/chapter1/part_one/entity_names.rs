#[derive(Copy, Clone)]
pub enum Names {
    MinerBob,
    // Elsa,
}

impl Names {
    pub fn to_string(&self) -> String {
        match self {
           // Names::Elsa => "Elsa",
            Names::MinerBob => "Miner Bob",
        }.to_string()
    }
}