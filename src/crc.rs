const fn generate_crc_table() -> [u32; 256] {
    let mut table = [0u32; 256];
    let mut i = 0;

    while i < 256 {
        let mut crc = i as u32;
        let mut j = 0;

        while j < 8 {
            let mask = (crc & 1).wrapping_neg();
            crc = (crc >> 1) ^ (0xEDB88320 & mask);
            j += 1;
        }

        table[i as usize] = crc;
        i += 1;
    }
    table
}

const CRC32_TABLE: [u32; 256] = generate_crc_table();

pub fn calculate_crc32_fast(data: &[u8]) -> u32 {
    let mut crc: u32 = !0xEADA2D49;
    for &byte in data {
        let index = ((crc ^ byte as u32) & 0xFF) as usize;
        crc = (crc >> 8) ^ CRC32_TABLE[index];
    }
    !crc
}