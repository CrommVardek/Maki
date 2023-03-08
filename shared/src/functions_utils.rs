pub fn bytes_to_u64(bytes: [u8; 32]) -> [u64; 4] {
    let mut result = [0; 4];

    for i in 0..4 {
        let bytes_array = <&[u8; 8]>::try_from(&bytes[i * 8..(i + 1) * 8]).unwrap();
        result[i] = u64::from_be_bytes(*bytes_array);
    }

    result
}