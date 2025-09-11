use bincode::{Decode, Encode, config};
use log::info;
use rand::{
    Rng,
    distr::{Alphabetic, SampleString},
    rng,
    seq::IndexedRandom,
};

#[derive(Debug, Encode, Decode, Clone)]
pub enum Payload {
    Spawn { id: u64, name: String },
    Damage { id: u64, value: i64 },
    ZoneChange,
}

pub enum Phase {
    Initial,
    Fight,
    End,
}

pub struct Simulation {
    phase: Phase,
    raid_damage: i64,
    total_damage: i64,
    ids: Vec<u64>,
}

static CONFIG: config::Configuration = config::standard();

impl Simulation {
    pub fn new() -> Self {
        Self {
            raid_damage: 0,
            total_damage: 100_000,
            phase: Phase::Initial,
            ids: vec![],
        }
    }

    pub fn tick(&mut self) -> Vec<Vec<u8>> {
        let mut packets = vec![];
        let mut rng = rng();

        match &self.phase {
            Phase::Initial => {
                self.phase = Phase::Fight;

                for _ in 0..4 {
                    let id = rng.random();
                    self.ids.push(id);

                    let payload = Payload::Spawn {
                        id,
                        name: random_nickname(),
                    };

                    info!("{:?}", payload);
                    let data = bincode::encode_to_vec(payload, CONFIG).unwrap();
                    packets.push(data);
                }
            }
            Phase::Fight => {
                if self.raid_damage >= self.total_damage {
                    let payload = Payload::ZoneChange;
                    let data = bincode::encode_to_vec(payload, CONFIG).unwrap();
                    packets.push(data);

                    self.phase = Phase::End;
                    return packets;
                }

                let value = rng.random_range(1000..10000);
                self.raid_damage += value;

                let id = *self.ids.choose(&mut rng).unwrap();

                let payload = Payload::Damage { id, value };

                let data = bincode::encode_to_vec(payload, CONFIG).unwrap();
                packets.push(data);
            }
            _ => {}
        }

        packets
    }
}

pub fn random_nickname() -> String {
    let mut rng = rng();
    let mut string;

    string = Alphabetic.sample_string(&mut rng, 10);

    let char = string.get_mut(0..1).unwrap();
    char.make_ascii_uppercase();

    let str = string.get_mut(1..).unwrap();
    str.make_ascii_lowercase();

    string
}
