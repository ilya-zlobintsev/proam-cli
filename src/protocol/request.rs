/*use crc16::{State, MODBUS};

pub fn request_to_buf(value: &[u8]) -> Vec<u8> {
    let value_trimmed = trim_slice_start(value);
    let value_size = value_trimmed.len();

    let mut base_value = vec![0; 2 + value_size];
    base_value[0..2].copy_from_slice(&0xA1C0u16.to_be_bytes());
    base_value[2..].copy_from_slice(value_trimmed);

    let crc = State::<MODBUS>::calculate(&base_value);

    let mut output = vec![0; 6 + value_size];

    output[0..4].copy_from_slice(&0x5AA5A1C0u32.to_be_bytes());
    output[4..4 + value_size].copy_from_slice(value_trimmed);
    output[4 + value_size..].copy_from_slice(&crc.to_le_bytes());

    output
}

fn trim_slice_start(value: &[u8]) -> &[u8] {
    let mut result = value;
    while let [first, rest @ ..] = result {
        if *first == 0 {
            result = rest;
        } else {
            break;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::request_to_buf;

    #[test]
    fn disable_light() {
        let value = [0x13, 0x01, 0x00, 0x00];
        let output = request_to_buf(&value);

        let expected_data = vec![0x5a, 0xa5, 0xa1, 0xc0, 0x13, 0x01, 0x00, 0x00, 0x4c, 0x3f];
        assert_eq!(expected_data, output);
    }

    #[test]
    fn enable_light() {
        let value = [0x13, 0x01, 0x00, 0x01];
        let output = request_to_buf(&value);

        let expected_data = vec![0x5a, 0xa5, 0xa1, 0xc0, 0x13, 0x01, 0x00, 0x01, 0x8d, 0xff];
        assert_eq!(expected_data, output);
    }

    #[test]
    fn enable_light_3() {
        let value = [0x13, 0x01, 0x00, 0x03];
        let output = request_to_buf(&value);

        let expected_data = vec![0x5a, 0xa5, 0xa1, 0xc0, 0x13, 0x01, 0x00, 0x03, 0x0c, 0x3e];
        assert_eq!(expected_data, output);
    }

    #[test]
    fn get_version() {
        let value = [0x06, 0x00, 0x00];
        let output = request_to_buf(&value);

        let expected_data = vec![0x5a, 0xa5, 0xa1, 0xc0, 0x06, 0x00, 0x00, 0x45, 0xd8];
        assert_eq!(expected_data, output);
    }

    #[test]
    fn idk() {
        let value = [0x20, 0x00, 0x00];
        let output = request_to_buf(&value);
        let expected_data = vec![0x5a, 0xa5, 0xa1, 0xc0, 0x20, 0x00, 0x00, 0xa4, 0x13];
        assert_eq!(expected_data, output);
    }
}*/
