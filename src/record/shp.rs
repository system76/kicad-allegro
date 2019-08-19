use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct ShpRecord {
    record_kind: String,
    subclass: String,
    pad_shape_name: String,
    graphic_data_name: String,
    graphic_data_number: String,
    record_tag: String,
    graphic_data_1: String,
    graphic_data_2: String,
    graphic_data_3: String,
    graphic_data_4: String,
    graphic_data_5: String,
    graphic_data_6: String,
    graphic_data_7: String,
    graphic_data_8: String,
    graphic_data_9: String,
    pad_stack_name: String,
    refdes: String,
    pin_number: String,
}
