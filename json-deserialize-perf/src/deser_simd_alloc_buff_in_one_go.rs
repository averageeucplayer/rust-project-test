#![allow(dead_code)]
#![allow(unsafe_op_in_unsafe_fn)]

use std::{fs::{self, File}, io::{BufReader, Read}, sync::OnceLock, thread::{self, JoinHandle}};
use anyhow::anyhow;
use serde::de::DeserializeOwned;
use hashbrown::{HashMap, HashSet};

use crate::models::*;

unsafe fn load_json<T: DeserializeOwned>(path: &str) -> T {
    let file = File::open(path).unwrap_unchecked();
    let reader = BufReader::with_capacity(1024 * 1024, file);
    serde_json::from_reader(reader).unwrap_unchecked()
}

// pub static COMBAT_EFFECT_DATA: OnceLock<HashMap<i32, CombatEffectData>> = OnceLock::new();
// pub static ENGRAVING_DATA: OnceLock<HashMap<u32, EngravingData>> = OnceLock::new();
// pub static SKILL_BUFF_DATA: OnceLock<HashMap<u32, SkillBuffData>> = OnceLock::new();
// pub static SKILL_DATA: OnceLock<HashMap<u32, SkillData>> = OnceLock::new();
// pub static SKILL_EFFECT_DATA: OnceLock<HashMap<u32, SkillEffectData>> = OnceLock::new();
// pub static SUPPORT_AP_GROUP: OnceLock<HashSet<u32>> = OnceLock::new();
// pub static SUPPORT_IDENTITY_GROUP: OnceLock<HashSet<u32>> = OnceLock::new();
// pub static STAT_TYPE_MAP: OnceLock<HashMap<String, u32>> = OnceLock::new();
// pub static ESTHER_DATA: OnceLock<Vec<Esther>> = OnceLock::new();
// pub static NPC_DATA: OnceLock<HashMap<u32, Npc>> = OnceLock::new();
// pub static GEM_SKILL_MAP: OnceLock<HashMap<u32, Vec<u32>>> = OnceLock::new();
// pub static RAID_MAP: OnceLock<HashMap<String, String>> = OnceLock::new();

pub struct AssetPreloader {
    pub combat_effect_data: HashMap<i32, CombatEffectData>,
    pub engraving_data: HashMap<u32, EngravingData>,
    pub skill_buff_data: HashMap<u32, SkillBuffData>,
    pub skill_data: HashMap<u32, SkillData>,
    pub skill_effect_data: HashMap<u32, SkillEffectData>,
    pub support_ap_group: HashSet<u32>,
    pub support_identity_group: HashSet<u32>,
    pub stat_type_map: HashMap<String, u32>,
    pub esther_data: Vec<Esther>,
    pub npc_data: HashMap<u32, Npc>,
    pub gem_skill_map: HashMap<u32, Vec<u32>>,
    pub raid_map: HashMap<String, String>,
}


unsafe fn load<T: DeserializeOwned>(path: &str, buffer: &mut Vec<u8>) -> T {
    buffer.clear();
    let mut file = File::open(path).unwrap_unchecked();
    file.read_to_end(buffer).unwrap_unchecked();
    simd_json::from_slice::<T>(buffer).unwrap_unchecked()
}

impl AssetPreloader {
    pub fn new() -> anyhow::Result<Self> {
        let mut buffer = Vec::with_capacity(1024 * 1024 * 30);

        Ok(Self {
            combat_effect_data: unsafe { load("meter-data/CombatEffect.json", &mut buffer) },
            engraving_data: unsafe { load("meter-data/Ability.json", &mut buffer) },
            skill_buff_data: unsafe { load("meter-data/SkillBuff.json", &mut buffer) },
            skill_data: unsafe { load("meter-data/Skill.json", &mut buffer) },
            skill_effect_data: unsafe { load("meter-data/SkillEffect.json", &mut buffer) },
            stat_type_map: unsafe { load("meter-data/StatType.json", &mut buffer) },
            esther_data: unsafe { load("meter-data/Esther.json", &mut buffer) },
            npc_data: unsafe { load("meter-data/Npc.json", &mut buffer) },
            gem_skill_map: unsafe {
                let raw: HashMap<String, (String, String, Vec<u32>)> =
                    load("meter-data/GemSkillGroup.json", &mut buffer);
                raw.into_iter()
                    .filter_map(|(key, entry)| key.parse::<u32>().ok().map(|id| (id, entry.2)))
                    .collect()
            },
            raid_map: unsafe {
                let encounters: HashMap<String, HashMap<String, Vec<String>>> =
                    load("meter-data/encounters.json", &mut buffer);
                encounters
                    .values()
                    .flat_map(|raid| raid.iter())
                    .flat_map(|(gate, bosses)| {
                        bosses.iter().map(move |boss| (boss.clone(), gate.clone()))
                    })
                    .collect()
            },
            support_ap_group: HashSet::from([101204, 101105, 314004, 480030]),
            support_identity_group: HashSet::from([211400, 368000, 310501, 480018]),
        })
    }

    // `wait` is no longer needed because loading is synchronous
}

// impl AssetPreloader {
//     pub fn new() -> Self {
//         let handle = thread::spawn(|| {
//             unsafe {
//                 let mut buffer = Vec::with_capacity(1024 * 1024 * 30);

//                 COMBAT_EFFECT_DATA
//                     .set(load("meter-data/CombatEffect.json", &mut buffer))
//                     .unwrap_unchecked();

//                 ENGRAVING_DATA
//                     .set(load("meter-data/Ability.json", &mut buffer))
//                     .unwrap_unchecked();

//                 SKILL_BUFF_DATA
//                     .set(load("meter-data/SkillBuff.json", &mut buffer))
//                     .unwrap_unchecked();

//                 SKILL_DATA
//                     .set(load("meter-data/Skill.json", &mut buffer))
//                     .unwrap_unchecked();

//                 SKILL_EFFECT_DATA
//                     .set(load("meter-data/SkillEffect.json", &mut buffer))
//                     .unwrap_unchecked();

//                 STAT_TYPE_MAP
//                     .set(load("meter-data/StatType.json", &mut buffer))
//                     .unwrap_unchecked();

//                 ESTHER_DATA
//                     .set(load("meter-data/Esther.json", &mut buffer))
//                     .unwrap_unchecked();

//                 NPC_DATA
//                     .set(load("meter-data/Npc.json", &mut buffer))
//                     .unwrap_unchecked();

//                 GEM_SKILL_MAP
//                     .set({
//                         let raw: HashMap<String, (String, String, Vec<u32>)> =
//                             load("meter-data/GemSkillGroup.json", &mut buffer);
//                         raw.into_iter()
//                             .filter_map(|(key, entry)| key.parse::<u32>().ok().map(|id| (id, entry.2)))
//                             .collect()
//                     })
//                     .unwrap_unchecked();

//                 RAID_MAP
//                     .set({
//                         let encounters: HashMap<String, HashMap<String, Vec<String>>> =
//                             load("meter-data/encounters.json", &mut buffer);
//                         encounters
//                             .values()
//                             .flat_map(|raid| raid.iter())
//                             .flat_map(|(gate, bosses)| {
//                                 bosses.iter().map(move |boss| (boss.clone(), gate.clone()))
//                             })
//                             .collect()
//                     })
//                     .unwrap_unchecked();

//                 SUPPORT_AP_GROUP
//                     .set(HashSet::from([
//                         101204, // bard
//                         101105, // paladin
//                         314004, // artist
//                         480030, // valkyrie
//                     ]))
//                     .unwrap_unchecked();

//                 SUPPORT_IDENTITY_GROUP
//                     .set(HashSet::from([
//                         211400, // bard serenade of courage
//                         368000, // paladin holy aura
//                         310501, // artist moonfall
//                         480018, // valkyrie release light
//                     ]))
//                     .unwrap_unchecked();
//             }
            

//         });

//         Self(handle)
//     }

//     pub fn wait(self) -> anyhow::Result<()> {
//         self.0.join().map_err(|err| anyhow!("Could not load assets {:?}", err))?;
//         anyhow::Ok(())
//     }
// }