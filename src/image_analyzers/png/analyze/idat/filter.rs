pub fn apply_filter(filter_type: &u8, cur_value: u8, prev_value: u8, upper_value: u8, upper_left_value: u8) -> u8 {
    match filter_type {
        1 => sub_filter(cur_value, prev_value),
        2 => up_filter(cur_value, upper_value),
        3 => average_filter(cur_value, prev_value, upper_value),
        4 => paeth_filter(cur_value, prev_value, upper_value, upper_left_value),
        _ => {
            // NONE
            cur_value
        }
    }
}

fn sub_filter(cur_value: u8, prev_value: u8) -> u8 {
    //((cur_value as u16 + prev_value.clone() as u16) % 256) as u8
    u8::wrapping_add(cur_value, prev_value)
}

fn up_filter(cur_value: u8, upper_value: u8) -> u8 {
    u8::wrapping_add(cur_value, upper_value)
}

fn average(a: u8, b: u8) -> u8 {
    ((a as u16 + b as u16) / 2) as u8
}

fn average_filter(cur_value: u8, left_value: u8, upper_value: u8) -> u8 {
    u8::wrapping_add(cur_value, average(left_value, upper_value))
}

fn paeth(a: u8, b: u8, c: u8) -> u8 {
    let large_a: i16 = a as i16;
    let large_b: i16 = b as i16;
    let large_c: i16 = c as i16;
    let paeth: i16 = (large_a + large_b - large_c);

    let paeth_a = (paeth - large_a).abs();
    let paeth_b = (paeth - large_b).abs();
    let paeth_c = (paeth - large_c).abs();

    if paeth_a <= paeth_b && paeth_a <= paeth_c {
        a
    } else if paeth_b <= paeth_c {
        b
    } else {
        c
    }
}

fn paeth_filter(cur_value: u8, left_value: u8, upper_value: u8, upper_left_value: u8) -> u8 {
    u8::wrapping_add(cur_value, paeth(left_value, upper_value, upper_left_value))
}