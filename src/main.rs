use std::process;

use log::error;

use anyhow::Result;

use prot2rust::generate::structure::{
    AlternativeOptions, Alternatives, SimpleStructure, Structure,
};
use prot2rust::{
    file::GenFile,
    generate::{bitfield, structure},
};

mod render_beacon;

fn render_mac() -> Result<()> {
    let mut genfile = GenFile::new();

    let addr_none = Structure::new("addr_none");
    let addr_short = SimpleStructure::new("addr_short", "address", 2);
    let addr_extended = SimpleStructure::new("addr_extended", "address", 8);

    genfile.add_struct(&addr_none)?;
    genfile.add_struct_simple(&addr_short)?;
    genfile.add_struct_simple(&addr_extended)?;

    let address = AlternativeOptions::new("address", &addr_none)
        .insert_type(&addr_short)
        .insert_type(&addr_extended);

    let pan_none = Structure::new("pan_none");
    let pan_short = SimpleStructure::new("pan_short", "pan", 2);

    genfile.add_struct(&pan_none)?;
    genfile.add_struct_simple(&pan_short)?;

    let panid = AlternativeOptions::new("panid", &pan_none).insert_type(&pan_short);

    let alternatives = Alternatives::new().insert(&address).insert(&panid);
    genfile.add_alternatives(&alternatives)?;

    let structure = Structure::new("mhr")
        .add_bitfield("frame_control", "frame_control", 2)
        .add_u8_field("sequence_number")
        .add_alt_field("dest_pan", &panid)
        .add_alt_field("dest_address", &address)
        .add_alt_field("source_pan", &panid)
        .add_alt_field("source_address", &address);

    genfile.add_struct_with_alts(&structure, &alternatives)?;

    genfile.write_file("out/mac_frame.rs")
}

fn render_frame_control() -> Result<()> {
    let mut genfile = GenFile::new();

    let bitfield = bitfield::BitField::new(
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
                .add_enum_value("Multipurpose", 0b101)
                .add_enum_value("Fragment", 0b110)
                .add_enum_value("Extended", 0b111)
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
        "PAN_Compression",
        "Specifies whether the MAC frame is to be sent within the same PAN.",
        1,
        |v| {
            v.add_enum_value("Uncompressed", 0)
                .add_enum_value("Compressed", 1)
        },
    )
    .add_reserved(1)
    .add_bit_field(
        "Seq_nr_suppression",
        "Specifies if the sequence number should be suppressed.",
        1,
        |v| {
            v.add_enum_value("included", 0)
                .add_enum_value("suppressed", 1)
        },
    )
    .add_bit_field(
        "IE_Present",
        "Specified if Information Elements (IEs) are contained in the frame.",
        1,
        |v| v.add_enum_value("none", 0).add_enum_value("present", 1),
    )
    .add_bit_field(
        "Dest_addr_mode",
        "Specifies the type of the destination address.",
        2,
        |v| {
            v.add_enum_value("Not_present", 0)
                .add_enum_value("Address_16bit", 0b10)
                .add_enum_value("Address_64bit_extended", 0b11)
        },
    )
    .add_bit_field(
        "Frame_version",
        "Specifies the version of the frame",
        2,
        |v| {
            v.add_enum_value("version_2003", 0b00)
                .add_enum_value("version_2006", 0b01)
                .add_enum_value("current", 0b10)
        },
    )
    .add_bit_field(
        "Source_addr_mode",
        "Specifies the type of the source address.",
        2,
        |v| {
            v.add_enum_value("Not_present", 0)
                .add_enum_value("Address_16bit", 0b10)
                .add_enum_value("Address_64bit_extended", 0b11)
        },
    );

    genfile.add_bitfield(&bitfield)?;

    genfile.write_file("out/frame_control.rs")
}

fn render_security_control() -> Result<()> {
    let mut genfile = GenFile::new();

    let bitfield = bitfield::BitField::new(
        "Security_control",
        "This field provides information about what protection is applied to the frame.",
    )
    .add_bit_field(
        "Security_level",
        "Indicates the actual frame protection that is provided",
        3,
        |v| {
            v.add_enum_value_desc("NONE", "Security level 0, no encryption.", 0b000)
                .add_enum_value_desc("MIC_32", "Security level 1, uses a 4 byte MIC for data authenticity.", 0b001)
                .add_enum_value_desc("MIC_64", "Security level 2, uses an 8 byte MIC for data authenticity.", 0b010)
                .add_enum_value_desc("MIC_128", "Security level 3, uses a 16 byte MIC for data authenticity.", 0b011)
                .add_enum_value_desc("ENC_MIC_32", "Security level 5, uses a 4 byte MIC for data encryption.", 0b101)
                .add_enum_value_desc("ENC_MIC_64", "Security level 6, uses an 8 byte MIC for data encryption.", 0b110)
                .add_enum_value_desc("ENC_MIC_128", "Security level 7, uses a 16 byte MIC for data encryption.", 0b111)
        }
    )
    .add_bit_field(
        "Key_identifier_mode",
        "Specifies whether the key that is used to protect the frame can be derived implicitly or explicitly.",
        2,
        |v| {
            v.add_enum_value_desc("implicit", "Key is determined implicitly.", 0b00)
                .add_enum_value_desc("key_index", "Key is determined from the key index field.", 0b01)
                .add_enum_value_desc("Key_source_4", "Key is determined explicitly from the 4-octet key source and key index fields.", 0b10)
                .add_enum_value_desc("Key_source_8", "Key is determined explicitly from the 8-octet key source and key index fields.", 0b11)
        })
    .add_bit_field(
        "Frame_counter_suppresion",
        "Specifies if the frame counter should be suppressed from the frame.",
        1,
        |v| v.add_enum_value_desc("present", "The frame counter is included in the frame.", 0)
            .add_enum_value_desc("suppressed", "The frame counter is suppressed from the frame.", 1))
    .add_bit_field(
        "ASN_in_nonce",
        "Specifies if the absolute number slot (ASN) is used to generate the Nonce.",
        1,
        |v| v.add_enum_value_desc("frame_counter_nonce", "The frame counter is used to generate the Nonce.", 0)
            .add_enum_value_desc("asn_nonce", "The ASN is used to generate the Nonce.", 1))
    .add_reserved(1);

    genfile.add_bitfield(&bitfield)?;

    genfile.write_file("out/security_control.rs")
}

fn render_auxiliary_security_header() -> Result<()> {
    let mut genfile = GenFile::new();

    let frame_counter_none = Structure::new("frame_counter_none");
    let frame_counter_present = SimpleStructure::new("frame_counter_present", "frame_counter", 4);

    genfile.add_struct(&frame_counter_none)?;
    genfile.add_struct_simple(&frame_counter_present)?;

    let frame_counter = AlternativeOptions::new("frame_counter_type", &frame_counter_none)
        .insert_type(&frame_counter_present);

    let key_id_none = Structure::new("key_id_none");
    let key_id_only = SimpleStructure::new("key_id_only", "key_id", 1);
    let key_id_short = Structure::new("key_id_short")
        .add_u32_field("key_source_1")
        .add_u8_field("key_id_1");
    let key_id_long = Structure::new("key_id_long")
        .add_u64_field("key_source_2")
        .add_u8_field("key_id_2");

    genfile.add_struct(&key_id_none)?;
    genfile.add_struct_simple(&key_id_only)?;
    genfile.add_struct(&key_id_short)?;
    genfile.add_struct(&key_id_long)?;

    let key_id = AlternativeOptions::new("key_id", &key_id_none)
        .insert_type(&key_id_only)
        .insert_type(&key_id_short)
        .insert_type(&key_id_long);

    let alternatives = Alternatives::new().insert(&frame_counter).insert(&key_id);
    genfile.add_alternatives(&alternatives)?;

    let structure = structure::Structure::new("Auxiliary_security_header")
        .add_bitfield("security_control", "security_control", 1)
        .add_alt_field("frame_counter", &frame_counter);

    genfile.add_struct_with_alts(&structure, &alternatives)?;

    genfile.write_file("out/auxiliary_security_header.rs")
}

pub fn render_ie_control() -> Result<()> {
    let mut genfile = GenFile::new();

    let bitfield = bitfield::BitField::new("IE Control", "Specifies the type of an IE header.")
        .add_bit_field(
            "Length",
            "Specifies the length of the IE header contents.",
            7,
            |v| v,
        )
        .add_bit_field(
            "Element_id",
            "Specifies the type of the IE header.",
            8,
            |v| {
                v.add_enum_value_desc("vendor_specific", "Vendor Specific Header IE", 0b00)
                    .add_enum_value_desc("CSL_IE", "CSL IE", 0x1a)
                    .add_enum_value_desc("RIT_IE", "RIT IE", 0x1b)
                    .add_enum_value_desc("DSME_PAN", "DSME PAN descriptor IE", 0x1c)
                    .add_enum_value_desc("Rendezvous Time IE", "Rendezvous Time IE", 0x1d)
                    .add_enum_value_desc("Time_Correction_IE", "Time Correction IE", 0x1e)
                    .add_enum_value_desc("Ext_DSME_PAN", "Extended DSME PAN descriptor IE", 0x21)
                    .add_enum_value_desc(
                        "Frag_seq_context",
                        "Fragment Sequence Context Description (FSCD) IE",
                        0x22,
                    )
                    .add_enum_value_desc(
                        "Simpl_Superframe",
                        "Simplified Superframe Specification IE",
                        0x23,
                    )
                    .add_enum_value_desc("Simpl_GTS", "Simplified GTS Specification IE", 0x24)
                    .add_enum_value_desc("LECIM_Capabilities", "LECIM Capabilities IE", 0x25)
                    .add_enum_value_desc("TRLE_Descr", "TRLE Descriptor IE", 0x26)
                    .add_enum_value_desc("RCC", "RCC Capabilities IE", 0x27)
                    .add_enum_value_desc("RCCN", "RCCN Descriptor IE", 0x28)
                    .add_enum_value_desc("Global_Time", "Global Time IE", 0x29)
                    .add_enum_value_desc("External_ANA", "Assigned to external organization", 0x2a)
                    .add_enum_value_desc("DA", "DA IE", 0x2b)
                    .add_enum_value_desc("Header_termination_1", "Header Termination 1 IE", 0x7e)
                    .add_enum_value_desc("Header_termination_2", "Header Termination 2 IE", 0x7f)
            },
        )
        .add_bit_field("Type", "Specifies the type of the IE header.", 1, |v| {
            v.add_enum_value("default", 0)
        });

    genfile.add_bitfield(&bitfield)?;

    genfile.write_file("out/ie_control.rs")
}

pub fn run() -> Result<()> {
    render_frame_control()?;
    render_mac()?;
    render_ie_control()?;
    render_security_control()?;
    render_auxiliary_security_header()?;

    render_beacon::render()?;

    Ok(())
}

fn main() {
    if let Err(ref e) = run() {
        error!("{:?}", e);

        process::exit(1);
    }
}
