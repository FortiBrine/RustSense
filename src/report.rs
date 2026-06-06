use bytemuck::{
    Pod,
    Zeroable
};
use crate::crc::calculate_crc32_fast;

#[repr(C, packed)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct DualSenseUsbPacket {
    report_id: u8,
    padding: u8,
    tag: u8,
    seq: u8,
    unknown_data: [u8; 7],
    payload_tag: u8,
    payload_length: u8,
    audio_data: [u8; 64],
    empty_space: [u8; 61],
    crc32: u32,
}

impl DualSenseUsbPacket {
    pub fn new() -> Self {
        Self {
            report_id: 0x32,
            padding: 0,
            tag: 0x91,
            seq: 7,
            unknown_data: [0xFE, 0, 0, 0, 0, 0xFF, 0],
            payload_tag: 0x92,
            payload_length: 64,
            audio_data: [0; 64],
            empty_space: [0; 61],
            crc32: 0,
        }
    }

    pub fn audio_slice_mut(&mut self) -> &mut [u8] {
        self.unknown_data[6] = self.unknown_data[6].wrapping_add(1);

        &mut self.audio_data
    }

    pub fn finalize(&mut self) -> &[u8] {
        let bytes = bytemuck::bytes_of(self);

        let crc = calculate_crc32_fast(&bytes[0..138]);
        self.crc32 = crc;

        bytemuck::bytes_of(self)
    }
}