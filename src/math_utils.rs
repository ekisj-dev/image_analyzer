pub fn as_u32_be(array: &[u8; 4]) -> u32 {
    ((array[0] as u32) << 24) +
        ((array[1] as u32) << 16) +
        ((array[2] as u32) << 8) +
        ((array[3] as u32) << 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_u32_be() {
        let input_array: [u8; 4] = [0, 0, 0, 1];

        let actual_result: u32 = as_u32_be(&input_array);

        assert_eq!(actual_result, 1);
    }

    #[test]
    fn test_as_u32_be_with_large_number() {
        let input_array: [u8; 4] = [255, 255, 255, 255];

        let actual_result: u32 = as_u32_be(&input_array);

        assert_eq!(actual_result, 4294967295);
    }
}