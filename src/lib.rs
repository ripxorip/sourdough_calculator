/// Struct used for the ingredients
#[derive(Debug)]
pub struct DoughData {
    pub inoculation: f32,
    pub starter_hydration: f32,
    pub salt: f32,
    pub flour: f32,
    pub hydration: f32
}

/// Resulting struct for the dough (in grams)
#[derive(Debug)]
pub struct DoughResult {
    pub flour: f32,
    pub water: f32,
    pub salt: f32,
    pub starter: f32,
    pub dough_weight: f32,
}

pub fn calculate_dough(dd: DoughData) -> DoughResult {
    let tot_flour = dd.flour + dd.inoculation * dd.flour * (1.00 - dd.starter_hydration);
    let tot_starter = dd.flour * dd.inoculation;
    let tot_water = tot_flour * dd.hydration;
    let missing_water = tot_water - (dd.starter_hydration * tot_starter);
    let amount_salt = dd.salt * dd.flour;
    let tot_weight = amount_salt + tot_flour + tot_water;

    DoughResult{flour: dd.flour, water: missing_water, salt: dd.salt*dd.flour, starter: tot_starter, dough_weight: tot_weight}
}
