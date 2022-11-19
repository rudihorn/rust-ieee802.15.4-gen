use std::process;

use anyhow::Result;

use prot2rust::generate::structure::Structure;
use prot2rust::{file::GenFile, generate::bitfield};

pub fn render_mac_command_id() -> Result<()> {
    let mut genfile = GenFile::new();

    let bitfield = bitfield::BitField::new("MAC command", "The MAC command identifier")
        .add_bit_field("id", "The MAC command identifier.", 8, |v| {
            v.add_enum_value_desc("assoc_request", "Association request command", 1)
                .add_enum_value_desc("assoc_response", "Association response command", 2)
                .add_enum_value_desc("dissasoc_notify", "Dissassociation Notification command", 3)
                .add_enum_value_desc("data_request", "Data request command", 4)
                .add_enum_value_desc("pan_id_conflict", "PAN ID Conflict Notification command", 5)
                .add_enum_value_desc("orphan_notify", "Orphan notification command", 6)
                .add_enum_value_desc("beacon_request", "Beacon request command", 7)
                .add_enum_value_desc("coordinator_realign", "Coordinator Realignment command", 8)
                .add_enum_value_desc("gts_request", "GTS request command", 9)
                .add_enum_value_desc("trle_mgmt_request", "TRLE Management Request command", 0x0a)
                .add_enum_value_desc(
                    "trle_mgmt_response",
                    "TRLE Management Response command",
                    0x0b,
                )
                // reserved
                .add_enum_value_desc(
                    "dsme_association_request",
                    "DSME Association Request command",
                    0x13,
                )
                .add_enum_value_desc(
                    "dsme_association_response",
                    "DSME Association Response command",
                    0x14,
                )
                .add_enum_value_desc("dsme_gts_request", "DSME Association Request command", 0x15)
                .add_enum_value_desc(
                    "dsme_gts_response",
                    "DSME Association Response command",
                    0x16,
                )
                .add_enum_value_desc("dsme_gts_notify", "DSME Association Notify command", 0x17)
                .add_enum_value_desc(
                    "dsme_info_request",
                    "DSME Information Request command",
                    0x18,
                )
                .add_enum_value_desc(
                    "dsme_info_response",
                    "DSME Information Response command",
                    0x19,
                )
                .add_enum_value_desc(
                    "dsme_beacon_alloc_notify",
                    "DSME Beacon Allocation Notification command",
                    0x1a,
                )
                .add_enum_value_desc(
                    "dsme_beacon_collision_notify",
                    "DSME Beacon Collision Notification command",
                    0x1b,
                )
                .add_enum_value_desc("dsme_link_report", "DSME Link Report command", 0x1c)
                // reserved
                .add_enum_value_desc("rit_data_request", "RIT Data Request command", 0x20)
                .add_enum_value_desc("dbs_request", "DBS Request command", 0x21)
                .add_enum_value_desc("dbs_response", "DBS Response command", 0x22)
                .add_enum_value_desc("rit_data_response", "RIT Data Response command", 0x23)
                .add_enum_value_desc("vendor_specific", "Vendor Specific command", 0x24)
                .add_enum_value_desc("srm_request", "SRM Request command", 0x25)
                .add_enum_value_desc("srm_response", "SRM Response command", 0x26)
                .add_enum_value_desc("srm_report", "SRM Report command", 0x27)
                .add_enum_value_desc("srm_info", "SRM Information command", 0x28)
            // reserved
        });

    genfile.add_bitfield(&bitfield)?;

    genfile.write_file("out/mac_command/command_id.rs")
}

pub fn render_association_request_capability() -> Result<()> {
    let mut genfile = GenFile::new();

    let bitfield = bitfield::BitField::new("Capability", "Association request capabilities")
        .add_reserved(1)
        .add_bit_field("device_type", "Set to one if the device is an FFD, otherwise it is an RFD.", 1, |v| v.add_enum_value("ffd_device", 1).add_enum_value("rfd_device", 0))
        .add_bit_field("power_source", "Set to one if the device is connected to Alternating Current, otherwise it is a battery device.", 1, |v| v.add_enum_value_desc("mains_powered", "The device is connected to alternative current mains.", 1).add_enum_value_desc("battery_powered", "The device is powered by a battery pack.", 0))
        .add_bit_field("receiver_on_when_idle", "The device does not disable its receiver to conserve power during idle periods.", 1, |v|
                       v.add_enum_value_desc("receives_on_idle", "The device does not disable its receiver during idle periods.", 1)
                       .add_enum_value_desc("disables_on_idle", "The device disables its receiver to conserve power during idle periods.", 0))
        .add_bit_field("association_type", "Set to one if the device requests fast association.", 1, |v| v.add_enum_value("fast_association", 1).add_enum_value("slow_association", 0))
        .add_reserved(1)
        .add_bit_field("security_capability", "Determines if the device is capable of sending and receiving cryptographically protected MAC frames.", 1, |v| v.add_enum_value_desc("secure", "The device is capable of sending and receiving cryptographically protected MAC frames.", 1).add_enum_value_desc("unsecure", "The device is incapable of sending and receiving cryptographically protected MAC frames.", 0))
        .add_bit_field("allocate_address", "Determines if the coordinator should allocate a short address as a result of the allocation procedure.", 1, |v| v.add_enum_value_desc("request_address", "The device wishes the coordinator to allocate a short address.", 1).add_enum_value_desc("no_request", "The device does not request the coordinator to allocate a short address.", 0));

    genfile.add_bitfield(&bitfield)?;

    genfile.write_file("out/mac_command/assoc_request_capability.rs")
}

pub fn render_association_status() -> Result<()> {
    let mut genfile = GenFile::new();

    let bitfield = bitfield::BitField::new("Assoc_status", "Association status").add_bit_field(
        "association_status",
        "The association status after a request.",
        8,
        |v| {
            v.add_enum_value_desc("assoc_success", "Association successful.", 0)
                .add_enum_value_desc("pan_at_capacity", "The PAN is at capacity.", 1)
                .add_enum_value_desc("pan_access_denied", "PAN access denied.", 2)
                .add_enum_value_desc(
                    "hopping_duplication",
                    "Hopping sequence offset duplication.",
                    3,
                )
                .add_enum_value_desc("fast_assoc_success", "Fast association successful.", 0x80)
        },
    );

    genfile.add_bitfield(&bitfield)?;

    genfile.write_file("out/mac_command/assoc_status.rs")
}

pub fn render_commands() -> Result<()> {
    let mut genfile = GenFile::new();

    genfile.add_struct_imports()?;

    let assoc_request =
        Structure::new("assoc_request").add_bitfield("capability", "assoc_request_capability", 1);

    let assoc_response = Structure::new("assoc_response")
        .add_u16_field("short_address")
        .add_bitfield("status", "assoc_status", 1);

    genfile.add_struct(&assoc_request)?;

    genfile.write_file("out/mac_command/commands.rs")
}

pub fn render() -> Result<()> {
    render_mac_command_id()?;
    render_association_request_capability()?;
    render_association_status()?;
    render_commands()
}
