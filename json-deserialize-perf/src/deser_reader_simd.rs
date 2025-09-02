#![allow(dead_code)]
#![allow(unsafe_op_in_unsafe_fn)]

use std::{fs::File, io::BufReader};
use anyhow::anyhow;
use serde::de::DeserializeOwned;
use hashbrown::{HashMap, HashSet};

use crate::models::*;

unsafe fn load_json<T: DeserializeOwned>(path: &str) -> T {
    let file = File::open(path).unwrap_unchecked();
    let reader = BufReader::with_capacity(1024 * 1024, file);
    simd_json::from_reader(reader).unwrap_unchecked()
}

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

impl AssetPreloader {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            combat_effect_data: unsafe { load_json("meter-data/CombatEffect.json") },
            engraving_data: unsafe { load_json("meter-data/Ability.json") },
            skill_buff_data: unsafe { load_json("meter-data/SkillBuff.json") },
            skill_data: unsafe { load_json("meter-data/Skill.json") },
            skill_effect_data: unsafe { load_json("meter-data/SkillEffect.json") },
            stat_type_map: unsafe { load_json("meter-data/StatType.json") },
            esther_data: unsafe { load_json("meter-data/Esther.json") },
            npc_data: unsafe { load_json("meter-data/Npc.json") },
            gem_skill_map: unsafe {
                let raw_map: HashMap<String, (String, String, Vec<u32>)> =
                    load_json("meter-data/GemSkillGroup.json");
                raw_map
                    .into_iter()
                    .filter_map(|(key, entry)| key.parse::<u32>().ok().map(|id| (id, entry.2)))
                    .collect()
            },
            raid_map: unsafe {
                let encounters: HashMap<String, HashMap<String, Vec<String>>> =
                    load_json("meter-data/encounters.json");
                encounters
                    .values()
                    .flat_map(|raid| raid.iter())
                    .flat_map(|(gate, bosses)| bosses.iter().map(move |boss| (boss.clone(), gate.clone())))
                    .collect()
            },
            support_ap_group: HashSet::from([101204, 101105, 314004, 480030]),
            support_identity_group: HashSet::from([211400, 368000, 310501, 480018]),
        })
    }

    // No thread, so no wait() needed
}

// #![allow(dead_code)]
// #![allow(unsafe_op_in_unsafe_fn)]

// use std::{fs::File, io::BufReader, thread::{self, JoinHandle}};
// use anyhow::anyhow;
// use serde::de::DeserializeOwned;
// use std::sync::LazyLock;
// use hashbrown::{HashMap, HashSet};

// use crate::models::*;

// unsafe fn load_json<T: DeserializeOwned>(path: &str) -> T {
//     let file = File::open(path).unwrap_unchecked();
//     let reader = BufReader::with_capacity(1024 * 1024, file);
//     simd_json::from_reader(reader).unwrap_unchecked()
// }

// pub static COMBAT_EFFECT_DATA: LazyLock<HashMap<i32, CombatEffectData>> = LazyLock::new(|| {
//     unsafe { load_json("meter-data/CombatEffect.json") }
// });

// pub static ENGRAVING_DATA: LazyLock<HashMap<u32, EngravingData>> = LazyLock::new(|| {
//     unsafe { load_json("meter-data/Ability.json") }
// });

// pub static SKILL_BUFF_DATA: LazyLock<HashMap<u32, SkillBuffData>> = LazyLock::new(|| {
//     unsafe { load_json("meter-data/SkillBuff.json") }
// });

// pub static SKILL_DATA: LazyLock<HashMap<u32, SkillData>> = LazyLock::new(|| {
//     unsafe { load_json("meter-data/Skill.json") }
// });

// pub static SKILL_EFFECT_DATA: LazyLock<HashMap<u32, SkillEffectData>> = LazyLock::new(|| {
//     unsafe { load_json("meter-data/SkillEffect.json") }
// });

// pub static SUPPORT_AP_GROUP: LazyLock<HashSet<u32>> = LazyLock::new(|| {
//     let set = HashSet::from([
//         101204, // bard
//         101105, // paladin
//         314004, // artist
//         480030, // valkyrie
//     ]);

//     set
// });

// pub static SUPPORT_IDENTITY_GROUP: LazyLock<HashSet<u32>> = LazyLock::new(|| {
//     let set = HashSet::from([
//         211400, // bard serenade of courage
//         368000, // paladin holy aura
//         310501, // artist moonfall
//         480018, // valkyrie release light
//     ]);

//     set
// });

// pub static STAT_TYPE_MAP: LazyLock<HashMap<String, u32>> = LazyLock::new(|| {
//     unsafe { load_json("meter-data/StatType.json") }
// });

// pub static ESTHER_DATA: LazyLock<Vec<Esther>> = LazyLock::new(|| {
//     unsafe { load_json("meter-data/Esther.json") }
// });

// pub static NPC_DATA: LazyLock<HashMap<u32, Npc>> = LazyLock::new(|| {
//     unsafe { load_json("meter-data/Npc.json") }
// });

// pub static GEM_SKILL_MAP: LazyLock<HashMap<u32, Vec<u32>>> = LazyLock::new(|| {
//     let raw_map: HashMap<String, (String, String, Vec<u32>)> = unsafe { load_json("meter-data/GemSkillGroup.json") };
    
//     raw_map
//         .into_iter()
//         .filter_map(|(key, entry)| key.parse::<u32>().ok().map(|id| (id, entry.2)))
//         .collect()
// });

// pub static RAID_MAP: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
//     let encounters: HashMap<String, HashMap<String, Vec<String>>> = unsafe { load_json("meter-data/encounters.json") };

//     encounters
//         .values()
//         .flat_map(|raid| raid.iter())
//         .flat_map(|(gate, bosses)| bosses.iter().map(move |boss| (boss.clone(), gate.clone())))
//         .collect()
// });

// pub struct AssetPreloader(JoinHandle<()>);

// impl AssetPreloader {
//     pub fn new() -> Self {
//         let handle = thread::spawn(|| {
//             SKILL_BUFF_DATA.iter().next();
//             SKILL_BUFF_DATA.iter().next();
//             SKILL_DATA.iter().next();
//             STAT_TYPE_MAP.iter().next();
//             ESTHER_DATA.iter().next();
//             NPC_DATA.iter().next();
//             GEM_SKILL_MAP.iter().next();
//         });

//         Self(handle)
//     }

//     pub fn wait(self) -> anyhow::Result<()> {
//         self.0.join().map_err(|err| anyhow!("Could not load assets {:?}", err))?;
//         anyhow::Ok(())
//     }
// }