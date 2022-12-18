use anyhow::{ensure, Result};
use realsense_rust::{context::Context, device::Device, kind::Rs2CameraInfo};
use std::collections::HashSet;

fn match_info(device: &Device, info_param: Rs2CameraInfo) -> String {
    match device.info(info_param) {
        Some(s) => String::from(s.to_str().unwrap()),
        None => String::from("N/A"),
    }
}

pub fn run_sanity_check() -> Result<()> {
    println!("----\nEnumerating all devices compatible with RealSnse:\n----");
    // The blow code is the equivalent of creating a HashSet of query_devices that just containts Rs2ProductLine::Any
    // In other words, this will look for any connected device that is compatible with Realsense.
    let devices = Context::new()?.query_devices(HashSet::new());
    ensure!(!devices.is_empty(), "No devices found");

    for device in devices {
        let name = match_info(&device, Rs2CameraInfo::Name);
        let sn = match_info(&device, Rs2CameraInfo::SerialNumber);
        let fw = match_info(&device, Rs2CameraInfo::FirmwareVersion);
        let rec_fw = match_info(&device, Rs2CameraInfo::RecommendedFirmwareVersion);

        println!(
            "> {:25} | SN: {:15} | Curr Fw Ver: {:15} | Rec FW Ver: {:15}",
            name, sn, fw, rec_fw
        );
    }
    println!("---");
    Ok(())
}
