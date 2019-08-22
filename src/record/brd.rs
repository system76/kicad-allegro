use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct BrdRecord {
    pub record_kind: String,
    pub layer_sort: String,
    pub layer_subclass: String,
    pub layer_artwork: String,
    pub layer_use: String,
    pub layer_conductor: String,
    pub layer_dielectric_constant: String,
    pub layer_electrical_conductivity: String,
    pub layer_material: String,
    pub layer_shield_layer: String,
    pub layer_thermal_conductivity: String,
    pub layer_thickness: String,
}
