#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|input: (u16, u16, bool, &[u8])| {
    let (w, h, is_4, data) = input;
    let (w, h) = (1 + w % 260, 1 + h % 260);
    let channels = if is_4 { 4 } else { 3 };

    let mut vec = vec![
        b'q',
        b'o',
        b'i',
        b'f',
        0,
        0,
        (w >> 8) as u8,
        (w & 0xff) as u8,
        0,
        0,
        (h >> 8) as u8,
        (h & 0xff) as u8,
        channels,
        0,
    ];
    vec.extend(&*data);
    vec.extend(&[0, 0, 0, 0, 0, 0, 0, 1]);

    let header_expected = qoi_fast::Header {
        width: w as u32,
        height: h as u32,
        channels: qoi_fast::Channels::try_from(channels).unwrap(),
        colorspace: qoi_fast::ColorSpace::try_from(0).unwrap(),
    };
    assert_eq!(qoi_fast::qoi_decode_header(&vec).unwrap(), header_expected);

    if let Ok((header, out)) = qoi_fast::qoi_decode_to_vec(&vec) {
        assert_eq!(header, header_expected);
        assert_eq!(out.len(), header.n_bytes());
    }
});
