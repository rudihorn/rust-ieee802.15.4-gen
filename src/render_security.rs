use anyhow::Result;

use prot2rust::{
    file::GenFile,
    generate::{
        bitfield,
        structure::{self, AlternativeOptions, Alternatives, SimpleStructure, Structure},
    },
};

pub fn render_security_control() -> Result<()> {
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

pub fn render_auxiliary_security_header() -> Result<()> {
    let mut genfile = GenFile::new();

    genfile.add_struct_imports()?;

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

    let key_id = AlternativeOptions::new("key_identifier", &key_id_none)
        .insert_type(&key_id_only)
        .insert_type(&key_id_short)
        .insert_type(&key_id_long);

    let alternatives = Alternatives::new().insert(&frame_counter).insert(&key_id);
    genfile.add_alternatives(&alternatives)?;

    let structure = structure::Structure::new("Auxiliary_security_header")
        .add_bitfield("security_control", "security_control", 1)
        .add_alt_field("frame_counter", &frame_counter)
        .add_alt_field("key_id", &key_id);

    genfile.add_struct_with_alts(&structure, &alternatives)?;

    genfile.write_file("out/auxiliary_security_header.rs")
}
