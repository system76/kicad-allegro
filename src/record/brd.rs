use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct BrdRecord {
    record_kind: String,
    layer_sort: String,
    layer_subclass: String,
    layer_artwork: String,
    layer_use: String,
    layer_conductor: String,
    layer_dielectric_constant: String,
    layer_electrical_conductivity: String,
    layer_material: String,
    layer_shield_layer: String,
    layer_thermal_conductivity: String,
    layer_thickness: String,
}
