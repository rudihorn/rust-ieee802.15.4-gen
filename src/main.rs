#![recursion_limit = "128"]

use generate::structure::SimpleStructure;
use log::error;
use proc_macro2::TokenStream;
use crate::generate::structure::AlternativeOptions;
use crate::generate::structure::Alternatives;

use std::fs::File;
use std::io::Write;
use std::process;

use anyhow::{Context, Result};

use crate::generate::structure::Structure;

fn render_mac() -> Result<()> {
    let filename = "out/mac_frame.rs";
    let mut file = File::create(filename).expect("Could not create output file.");

    let mut items = TokenStream::new();

    let addr_none = Structure::new("addr_none");
    let addr_short = SimpleStructure::new("addr_short", "address", 2);
    let addr_extended = SimpleStructure::new("addr_extended", "address", 8);

    items.extend(generate::structure::render(&addr_none)?);
    items.extend(generate::structure::render_simple(&addr_short)?);
    items.extend(generate::structure::render_simple(&addr_extended)?);

    let address = AlternativeOptions::new("address", &addr_none)
        .insert_type(&addr_short)
        .insert_type(&addr_extended);

    let pan_none = Structure::new("pan_none");
    let pan_short = SimpleStructure::new("pan_short", "pan", 2);

    items.extend(generate::structure::render(&pan_none)?);
    items.extend(generate::structure::render_simple(&pan_short)?);

    let panid = AlternativeOptions::new("panid", &pan_none).insert_type(&pan_short);

    let alternatives = Alternatives::new().insert(&address).insert(&panid);
    items.extend(generate::structure::render_alternatives(&alternatives)?);

    let structure = Structure::new("mhr")
        .add_bitfield("frame_control", "frame_control", 2)
        .add_u8_field("sequence_number")
        .add_alt_field("dest_pan", &panid)
        .add_alt_field("dest_address", &address)
        .add_alt_field("source_pan", &panid)
        .add_alt_field("source_address", &address);

    items.extend(generate::structure::render_with_alts(&structure, &alternatives)?);

    let data = items.to_string().replace("] ", "]\n");
    file.write_all(data.as_ref())
        .expect("Could not write code to lib.rs");

    Ok(())
}

fn render_fields() -> Result<()> {
    let filename = "out/frame_control.rs";
    let mut file = File::create(filename).expect("Could not create output file.");

    let bitfield = generate::bitfield::BitField::new(
        "Frame_control",
        "This field contains information about the frame type, addressing and control flags.",
    )
    .add_bit_field(
        "Frame_type",
        "This field contains information about the frame type, addressing and control flags.",
        3,
        |v| {
            v.add_enum_value("Beacon", 0b000)
                .add_enum_value("Data", 0b001)
                .add_enum_value("Acknowledgement", 0b010)
                .add_enum_value("MAC_command", 0b011)
        },
    )
    .add_bit_field(
        "Security_enabled",
        "Specifies if the frame is encrypted using the key stored in the PIB.",
        1,
        |v| {
            v.add_enum_value("Unencrypted", 0)
                .add_enum_value("Encrypted", 1)
        },
    )
    .add_bit_field(
        "Frame_pending",
        "Specifies if the sender has additional data to send to the recipient.",
        1,
        |v| {
            v.add_enum_value("No_frame_pending", 0)
                .add_enum_value("Frame_pending", 1)
        },
    )
    .add_bit_field(
        "Ack_request",
        "Specifies whether an acknowledgement is required from the recipient device.",
        1,
        |v| {
            v.add_enum_value("Ack_not_requested", 0)
                .add_enum_value("Ack_requested", 1)
        },
    )
    .add_bit_field(
        "Intra_PAN",
        "Specifies whether the MAC frame is to be sent within the same PAN.",
        1,
        |v| {
            v.add_enum_value("Pan_present", 0)
                .add_enum_value("Inter_pan", 1)
        },
    )
    .add_reserved(3)
    .add_bit_field(
        "Dest_addr_mode",
        "Specifies the type of the destination address.",
        2,
        |v| {
            v.add_enum_value("Not_present", 0)
                .add_enum_value("Address_16bit", 1)
                .add_enum_value("Address_64bit_extended", 3)
        },
    )
    .add_reserved(2)
    .add_bit_field(
        "Source_addr_mode",
        "Specifies the type of the source address.",
        2,
        |v| {
            v.add_enum_value("Not_present", 0)
                .add_enum_value("Address_16bit", 1)
                .add_enum_value("Address_64bit_extended", 3)
        },
    );

    let items =
        generate::bitfield::render(&bitfield).with_context(|| "Error rendering structure")?;

    let data = items.to_string().replace("] ", "]\n");
    file.write_all(data.as_ref())
        .expect("Could not write code to lib.rs");

    Ok(())
}

pub fn run() -> Result<()> {
    render_fields()?;
    render_mac()?;

    Ok(())
}

fn main() {
    if let Err(ref e) = run() {
        error!("{:?}", e);

        process::exit(1);
    }
}
