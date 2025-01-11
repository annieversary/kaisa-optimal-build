use crate::ddragon::Items;

#[derive(Clone, Debug)]
pub struct Item {
    pub name: String,
    pub id: String,

    pub cost: u16,

    pub ad: u16,
    pub ap: u16,
    pub asp: u16,
    pub ms: u16,

    pub limit: u16,
}

pub fn ddragon_to_items(items: &Items) -> Vec<Item> {
    // TODO get all items that are inStore

    // TODO limit comes from groups
    // so we need to find a group that contains that item and has a max ownable

    // ad -> FlatPhysicalDamageMod
    // ap -> FlatMagicDamageMod
    // asp -> FlatAttackSpeedMod
    // ms -> FlatMovementSpeedMod

    let mut result = vec![];
    for (id, item) in items.data.iter() {
        // filter out items
        let in_store = item.in_store.or(items.basic.in_store).unwrap_or(false);
        let purchasable = item.gold.purchasable;
        let in_rift = item.maps.get("11").cloned().unwrap_or(true);
        if !in_store || !purchasable || !in_rift {
            continue;
        }

        let stat = |name: &str| item.stats.get(name).cloned().unwrap_or(0.0);
        let percent_stat = |name: &str| (stat(name) * 100.0).floor() as u16;

        let ad = stat("FlatPhysicalDamageMod") as u16;
        let ap = stat("FlatMagicDamageMod") as u16;
        let asp = percent_stat("PercentAttackSpeedMod");
        let ms = percent_stat("PercentMovementSpeedMod");

        if ad == 0 && ap == 0 && asp == 0 {
            continue;
        }

        let cost = item.gold.total as u16;

        // max bc of item slots (ignoring group restrictions)
        let max = 5 + item.stacks.unwrap_or(1);
        // check the list of groups to see if it contains this item
        let group = items.find_group_for_item(id, item);
        let limit = group
            .and_then(|group| group.max_group_ownable.parse().ok())
            // -1 is nothing
            .and_then(|limit| if limit == -1 { None } else { Some(limit) })
            // if there is no group, or the limit is -1, use the default max
            .unwrap_or(max) as u16;

        result.push(Item {
            name: item.name.clone(),
            id: id.clone(),
            cost,
            ad,
            ap,
            asp,
            ms,
            limit,
        });
    }

    result.sort_unstable_by_key(|item| item.id.clone());

    result
}
