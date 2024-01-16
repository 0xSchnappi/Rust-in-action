fn main() {
    println!("Hello, world!");
    let n = u8::MAX;
    println!("{}", n);
    mock_rand(8);
}

fn mock_rand(n: u8) -> f32 {
    // 255是u8能表示的最大值
    let base: u32 = 0b0_01111110_00000000000000000000000;

    let large_n = (n as u32) << 15;

    let f32_bits = base | large_n;

    let m = f32::from_bits(f32_bits);
    // m max = 0b0_01111110_11111111000000000000000     0.9980469
    // m min = 0b0_01111110_00000000000000000000000     0.5
    let m_max = f32::from_bits(0b0_01111110_11111111000000000000000);
    let m_min = f32::from_bits(0b0_01111110_00000000000000000000000);
    println!("m max = {}", m_max);
    println!("m min = {}", m_min);

    2.0 * (m - 0.5)     // 放大到0～0.996之间
}
