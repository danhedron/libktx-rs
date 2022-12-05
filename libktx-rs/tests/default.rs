// Copyright (C) 2021 Paolo Jovon <paolo.jovon@gmail.com>
// SPDX-License-Identifier: Apache-2.0

use libktx_rs::{
    sources::{Ktx1CreateInfo, Ktx2CreateInfo},
    Texture,
};

#[test]
fn create_default_ktx1() {
    let texture = Texture::new(Ktx1CreateInfo::default()).expect("a default KTX1 texture");

    // 1x1 RGBA8 texel
    assert_eq!(texture.element_size(), 4);
    assert_eq!(texture.row_pitch(0), 4);
    assert_eq!(texture.data_size(), 4);

    texture
        .iterate_levels(|mip, face, width, height, depth, pixel_data| {
            dbg!(mip, face, width, height, depth, pixel_data);
            Ok(())
        })
        .expect("mip/face iteration to succeed");
}

#[test]
fn create_default_ktx2() {
    let texture = Texture::new(Ktx2CreateInfo::default()).expect("a default KTX2 texture");

    // 1x1 RGBA8 texel
    assert_eq!(texture.element_size(), 4);
    assert_eq!(texture.row_pitch(0), 4);
    assert_eq!(texture.data_size(), 4);

    texture
        .iterate_levels(|mip, face, width, height, depth, pixel_data| {
            dbg!(mip, face, width, height, depth, pixel_data);
            Ok(())
        })
        .expect("mip/face iteration to succeed");
}

#[test]
fn write_read_metadata() {
    let mut texture = Texture::new(Ktx2CreateInfo::default()).expect("a default KTX2 texture");

    texture
        .add_key_value_pair_str("KTXorientation", "rd")
        .expect("Adding key-value to succeed");
    let orientation = texture
        .find_key_value_pair("KTXorientation")
        .expect("Find to have a result");
    assert_eq!(orientation, vec!['r' as u8, 'd' as u8, '\0' as u8]);

    let bytes_src = [0u8, 1u8, 2u8, 3u8];
    texture
        .add_key_value_pair("_test_bytes", &bytes_src)
        .expect("adding key-value pair to succeed");
    let bytes = texture
        .find_key_value_pair("_test_bytes")
        .expect("Find to have a result");
    assert_eq!(bytes, bytes_src);

    texture.delete_key_value_pair("_test_bytes");
    assert!(texture.find_key_value_pair("_test_bytes").is_none());
}
