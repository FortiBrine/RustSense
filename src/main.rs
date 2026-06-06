mod crc;
mod report;

use std::error::Error;
use hidapi::HidApi;
use spin_sleep_util::{interval, MissedTickBehavior};
use std::io::{self, Read};
use std::time::Duration;
use log::{error, info, trace};
use report::DualSenseUsbPacket;

const SONY_VID: u16 = 0x054C;
const DUALSENSE_PID: u16 = 0x0CE6;
const DUALSENSE_EDGE_PID: u16 = 0x0DF2;

const SAMPLE_SIZE: usize = 64;
const SAMPLE_RATE: u32 = 3000;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let api = HidApi::new()?;
    let device_info = api.device_list().find(|info| {
        info.vendor_id() == SONY_VID &&
            (info.product_id() == DUALSENSE_PID || info.product_id() == DUALSENSE_EDGE_PID)
    }).ok_or("DualSense not found! Please check the connection.")?;

    info!("Device found: {} (Path: {:?})",
             device_info.product_string().unwrap_or("DualSense"),
             device_info.path()
    );
    let gamepad = device_info.open_device(&api)?;

    let mut packet = DualSenseUsbPacket::new();
    let mut stdin = io::stdin().lock();

    let tick_duration = Duration::from_nanos(1_000_000_000 * SAMPLE_SIZE as u64 / (SAMPLE_RATE as u64 * 2));
    let mut ticker = interval(tick_duration)
        .with_missed_tick_behavior(MissedTickBehavior::Skip);

    info!("Connected successfully. Waiting for audio data from stdin...");

    loop {
        ticker.tick();

        if stdin.read_exact(packet.audio_slice_mut()).is_err() {
            info!("Audio stream finished.");
            break;
        }

        trace!("Sending packet...");

        if let Err(e) = gamepad.write(packet.finalize()) {
            error!("Error writing to controller: {}", e);
            break;
        }
    }

    Ok(())
}