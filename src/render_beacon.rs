use anyhow::Result;

use prot2rust::{
    file::GenFile,
    generate::{bitfield, structure::Structure},
};

pub fn render_superframe() -> Result<()> {
    let mut genfile = GenFile::new();

    let bitfield = bitfield::BitField::new("Superframe", "Superframe specification field.")
        .add_bit_field(
            "Beacon_order",
            "This field contains information about the transmission interval of the beacon.",
            4,
            |v| v,
        )
        .add_bit_field(
            "Superframe_order",
            "This field contains information about the transmission duration of the beacon.",
            4,
            |v| v,
        )
        .add_bit_field(
            "Final_CAP_slot",
            "This fied specifies the final superframe slot utilized by the CAP.",
            4,
            |v| v,
        ).add_bit_field(
            "batt_life_ext",
            "Set if the frames transmitted are required to start before battery life extended periods.",
            1,
            |v|
            v.add_enum_value_desc("BLE_not_set", "Battery life extension is not required.", 0)
                .add_enum_value_desc("BLE_set", "Battery life extension is required and packets must be sent before macBattlifeExtPeriods full backoff periods afetr the IFS period following the beacon.", 1))
        .add_reserved(1)
        .add_bit_field(
            "PAN_Coordinator",
            "Specifies if the sender is a PAN coordinator.",
            1,
            |v|
            v.add_enum_value_desc("not_pan_coordinator", "The transmitting device is not a PAN coordinator.", 0)
                .add_enum_value_desc("pan_coordinator", "The transmitting device is a PAN coordinator", 1))
        .add_bit_field("Assocation_permit", "Specifices if devices are permitted to join the PAN.", 1, |v|
        v.add_enum_value_desc("not_permitted", "Devices are not permitted to associate with the PAN.", 0).add_enum_value_desc("permitted", "Devices are permitted to associate with the PAN.", 1));

    genfile.add_bitfield(&bitfield)?;

    genfile.write_file("out/beacon/superframe.rs")
}

pub fn render_gts_specification() -> Result<()> {
    let mut genfile = GenFile::new();

    let bitfield = bitfield::BitField::new(
        "GTS_specification",
        "Guarranteed timeslot specification field.",
    )
    .add_bit_field(
        "descriptor_count",
        "The number of guaranteed timeslot descriptors included.",
        3,
        |v| v,
    )
    .add_reserved(4)
    .add_bit_field(
        "permit",
        "Specifies if the coordinator is accepting guaranteed timeslot requests.",
        1,
        |v| {
            v.add_enum_value_desc(
                "not_permitted",
                "The coordinator is not accepting GTS requests.",
                0,
            )
            .add_enum_value_desc(
                "permitted",
                "The coordinator is accepting GTS requests.",
                1,
            )
        },
    );

    genfile.add_bitfield(&bitfield)?;

    genfile.write_file("out/beacon/gts_specification.rs")
}

pub fn render_gts_direction() -> Result<()> {
    let mut genfile = GenFile::new();

    let bitfield =
        bitfield::BitField::new("GTS_Direction", "Guarranteed timeslot directions field.")
            .add_bit_field(
                "directions_mask",
                "Mask identifying the directions of the GTSs in the superframe.",
                7,
                |v| v,
            )
            .add_reserved(1);

    genfile.add_bitfield(&bitfield)?;

    genfile.write_file("out/beacon/gts_directions.rs")
}

pub fn render_gts_descriptor_config() -> Result<()> {
    let mut genfile = GenFile::new();

    let bitfield =
        bitfield::BitField::new("GTS_Descriptor", "The starting slot and length of a guaranteed time slot. Note that this does not include the device short address.")
            .add_bit_field("starting_slot", "The starting slot of the guaranteed time slot.", 4, |v| v)
            .add_bit_field("length", "The number of contiguous superframe slots over which this guaranteed time slot is active.", 4, |v| v);

    genfile.add_bitfield(&bitfield)?;

    genfile.write_file("out/beacon/gts_descriptor_config.rs")
}

pub fn render_gts_descriptor() -> Result<()> {
    let mut genfile = GenFile::new();

    genfile.add_struct_imports()?;

    let structure = Structure::new("gts_descriptor")
        .add_u16_field("short_address")
        .add_bitfield("config", "gts_descriptor_config", 1);

    genfile.add_struct(&structure)?;

    genfile.write_file("out/beacon/gts_descriptor.rs")
}

pub fn render() -> Result<()> {
    render_superframe()?;
    render_gts_specification()?;
    render_gts_direction()?;
    render_gts_descriptor_config()?;
    render_gts_descriptor()
}
