use anyhow::Result;
use serde_derive::{Deserialize, Serialize};
use std::{collections::HashMap, fs::read_to_string, path::Path};

// https://ddragon.leagueoflegends.com/cdn/14.24.1/data/en_US/item.json

pub fn parse_item_json(path: &Path) -> Result<Items> {
    let json = read_to_string(path)?;

    let result = serde_json::from_str(&json)?;

    Ok(result)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Items {
    pub version: String,
    pub basic: Item,
    pub data: HashMap<String, Item>,
    pub groups: Vec<Group>,
    pub tree: Vec<Tree>,
}

impl Items {
    pub fn find_group_for_item(&self, id: &str, _item: &Item) -> Option<&Group> {
        self.groups.iter().find(|group| group.id == id)
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Gold {
    pub base: i64,
    pub total: i64,
    pub sell: i64,
    pub purchasable: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Item {
    pub name: String,
    pub description: String,
    pub colloq: String,
    pub plaintext: String,
    #[serde(default)]
    pub into: Vec<String>,
    pub gold: Gold,
    pub tags: Vec<String>,
    pub maps: HashMap<String, bool>,
    pub stats: HashMap<String, f64>,
    #[serde(rename = "inStore")]
    pub in_store: Option<bool>,
    pub from: Option<Vec<String>>,
    pub effect: Option<Effect>,
    pub depth: Option<i64>,
    pub consumed: Option<bool>,
    pub stacks: Option<i64>,
    #[serde(rename = "hideFromAll")]
    pub hide_from_all: Option<bool>,
    #[serde(rename = "consumeOnFull")]
    pub consume_on_full: Option<bool>,
    #[serde(rename = "requiredChampion")]
    pub required_champion: Option<String>,
    #[serde(rename = "requiredAlly")]
    pub required_ally: Option<String>,
    #[serde(rename = "specialRecipe")]
    pub special_recipe: Option<i64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Effect {
    #[serde(rename = "Effect1Amount")]
    pub effect1_amount: String,
    #[serde(rename = "Effect2Amount")]
    pub effect2_amount: Option<String>,
    #[serde(rename = "Effect3Amount")]
    pub effect3_amount: Option<String>,
    #[serde(rename = "Effect4Amount")]
    pub effect4_amount: Option<String>,
    #[serde(rename = "Effect5Amount")]
    pub effect5_amount: Option<String>,
    #[serde(rename = "Effect6Amount")]
    pub effect6_amount: Option<String>,
    #[serde(rename = "Effect7Amount")]
    pub effect7_amount: Option<String>,
    #[serde(rename = "Effect8Amount")]
    pub effect8_amount: Option<String>,
    #[serde(rename = "Effect9Amount")]
    pub effect9_amount: Option<String>,
    #[serde(rename = "Effect10Amount")]
    pub effect10_amount: Option<String>,
    #[serde(rename = "Effect11Amount")]
    pub effect11_amount: Option<String>,
    #[serde(rename = "Effect12Amount")]
    pub effect12_amount: Option<String>,
    #[serde(rename = "Effect13Amount")]
    pub effect13_amount: Option<String>,
    #[serde(rename = "Effect14Amount")]
    pub effect14_amount: Option<String>,
    #[serde(rename = "Effect15Amount")]
    pub effect15_amount: Option<String>,
    #[serde(rename = "Effect16Amount")]
    pub effect16_amount: Option<String>,
    #[serde(rename = "Effect17Amount")]
    pub effect17_amount: Option<String>,
    #[serde(rename = "Effect18Amount")]
    pub effect18_amount: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Group {
    pub id: String,
    #[serde(rename = "MaxGroupOwnable")]
    pub max_group_ownable: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Tree {
    pub header: String,
    pub tags: Vec<String>,
}

// All stats
//  "FlatHPPoolMod": 0,
// "rFlatHPModPerLevel": 0,
// "FlatMPPoolMod": 0,
// "rFlatMPModPerLevel": 0,
// "PercentHPPoolMod": 0,
// "PercentMPPoolMod": 0,
// "FlatHPRegenMod": 0,
// "rFlatHPRegenModPerLevel": 0,
// "PercentHPRegenMod": 0,
// "FlatMPRegenMod": 0,
// "rFlatMPRegenModPerLevel": 0,
// "PercentMPRegenMod": 0,
// "FlatArmorMod": 0,
// "rFlatArmorModPerLevel": 0,
// "PercentArmorMod": 0,
// "rFlatArmorPenetrationMod": 0,
// "rFlatArmorPenetrationModPerLevel": 0,
// "rPercentArmorPenetrationMod": 0,
// "rPercentArmorPenetrationModPerLevel": 0,
// "FlatPhysicalDamageMod": 0,
// "rFlatPhysicalDamageModPerLevel": 0,
// "PercentPhysicalDamageMod": 0,
// "FlatMagicDamageMod": 0,
// "rFlatMagicDamageModPerLevel": 0,
// "PercentMagicDamageMod": 0,
// "FlatMovementSpeedMod": 0,
// "rFlatMovementSpeedModPerLevel": 0,
// "PercentMovementSpeedMod": 0,
// "rPercentMovementSpeedModPerLevel": 0,
// "FlatAttackSpeedMod": 0,
// "PercentAttackSpeedMod": 0,
// "rPercentAttackSpeedModPerLevel": 0,
// "rFlatDodgeMod": 0,
// "rFlatDodgeModPerLevel": 0,
// "PercentDodgeMod": 0,
// "FlatCritChanceMod": 0,
// "rFlatCritChanceModPerLevel": 0,
// "PercentCritChanceMod": 0,
// "FlatCritDamageMod": 0,
// "rFlatCritDamageModPerLevel": 0,
// "PercentCritDamageMod": 0,
// "FlatBlockMod": 0,
// "PercentBlockMod": 0,
// "FlatSpellBlockMod": 0,
// "rFlatSpellBlockModPerLevel": 0,
// "PercentSpellBlockMod": 0,
// "FlatEXPBonus": 0,
// "PercentEXPBonus": 0,
// "rPercentCooldownMod": 0,
// "rPercentCooldownModPerLevel": 0,
// "rFlatTimeDeadMod": 0,
// "rFlatTimeDeadModPerLevel": 0,
// "rPercentTimeDeadMod": 0,
// "rPercentTimeDeadModPerLevel": 0,
// "rFlatGoldPer10Mod": 0,
// "rFlatMagicPenetrationMod": 0,
// "rFlatMagicPenetrationModPerLevel": 0,
// "rPercentMagicPenetrationMod": 0,
// "rPercentMagicPenetrationModPerLevel": 0,
// "FlatEnergyRegenMod": 0,
// "rFlatEnergyRegenModPerLevel": 0,
// "FlatEnergyPoolMod": 0,
// "rFlatEnergyModPerLevel": 0,
// "PercentLifeStealMod": 0,
// "PercentSpellVampMod": 0
