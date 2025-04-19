use std::io;

use crate::MuxideError;

const EBML_VERSION: u32 = 0x4286;

#[derive(Debug, Default)]
struct EbmlHeader {
    version: u32,
    read_version: u32,
    max_id_length: u32,
    max_size_length: u32,
    doc_type: String,
    doc_type_version: u32,
    doc_type_read_version: u32,
}

fn validate_magic(input: &[u8]) -> Result<(&[u8], bool), MuxideError> {
    Ok((&input[4..], input[..4] == [0x1A, 0x45, 0xDF, 0xA3]))
}

fn parse_vint(input: &[u8]) -> Result<(&[u8], u64), MuxideError> {
    if input.is_empty() {
        return Err(MuxideError::UnexpectedEnd);
    }

    let v = input[0];
    let len = v.leading_zeros();

    if len == 8 {
        return Err(MuxideError::VintInavlidLength);
    }

    let mut value = u64::from(v ^ (1 << (7 - len)));

    for i in 0..len as usize {
        value = (value << 8) | u64::from(input[i + 1]);
    }
    Ok((&input[len as usize + 1..], value))
}

fn parse_ebml_header(input: &[u8]) -> Result<(&[u8], EbmlHeader), MuxideError> {
    let (input, valid) = validate_magic(input)?;
    assert!(valid);

    let (input, header_size) = parse_vint(input)?;
    assert_eq!(header_size, 35);

    Ok((input, EbmlHeader::default()))
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read, path::PathBuf};

    use super::{parse_vint, validate_magic};

    fn load_test_file(file: &str) -> File {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(file);

        eprintln!("{path:?}");
        File::open(&path).expect("Failed to open test file: {file}")
    }

    #[test]
    fn matroska_magic() {
        let mut file = load_test_file("../test_data/videos/canary_island-uhd_3840_2160_60fps.mkv");
        let mut input = vec![];
        let _ = file.read_to_end(&mut input);

        let (_, valid) = validate_magic(&mut input).unwrap();
        assert!(valid);
    }

    #[test]
    fn vint_parse() {
        let test_pairs: Vec<(Vec<u8>, u64)> = vec![(vec![0xA3], 35), (vec![0x81], 1), (vec![0x88], 8), (vec![0x9F], 31)];
        test_pairs.iter().for_each(|(input, expected)| {
            let (_, value) = parse_vint(&input).unwrap();
            assert_eq!(value, *expected);
        });
    }
}
