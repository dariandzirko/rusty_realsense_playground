//! Configure and stream a 435i sensor.
//!
//! Notice that the streaming configuration changes based on the USB speed of the sensor.
//! If one attemps to set a streaming configuration that is too much for the current USB
//! speed, RealSense will return with an error. However, that error is non-descript and will
//! not help identify the underlying problem, i.e. the bandwidth of the connection.

use anyhow::{ensure, Result};
use realsense_rust::{
    config::Config,
    context::Context,
    frame::{ColorFrame, DepthFrame, GyroFrame, InfraredFrame, PixelKind},
    kind::{Rs2CameraInfo, Rs2Format, Rs2ProductLine, Rs2StreamKind},
    pipeline::InactivePipeline,
};
use std::{
    collections::HashSet,
    convert::TryFrom,
    io::{self, Write},
    time::Duration,
};

pub fn stream_data() -> Result<()> {
    // Check for depth or color-compatible devices.
    let mut queried_devices = HashSet::new();
    queried_devices.insert(Rs2ProductLine::D400);
    let context = Context::new()?;
    let devices = context.query_devices(queried_devices);
    ensure!(!devices.is_empty(), "No devices found");

    // create pipeline
    let pipeline = InactivePipeline::try_from(&context)?;
    let mut config = Config::new();

    // Check the USB speed of our connection
    // CStr => str => f32
    let usb_cstr = devices[0].info(Rs2CameraInfo::UsbTypeDescriptor).unwrap();
    let usb_val: f32 = usb_cstr.to_str().unwrap().parse().unwrap();
    if usb_val >= 3.0 {
        config
            .enable_device_from_serial(devices[0].info(Rs2CameraInfo::SerialNumber).unwrap())?
            .disable_all_streams()?
            .enable_stream(Rs2StreamKind::Depth, None, 640, 0, Rs2Format::Z16, 30)?
            .enable_stream(Rs2StreamKind::Color, None, 640, 0, Rs2Format::Rgb8, 30)?
            // RealSense doesn't seem to like index zero for the IR cameras on D435i
            //
            // Really not sure why? This seems like an implementation issue, but in practice most
            // won't be after the IR image directly.
            .enable_stream(Rs2StreamKind::Infrared, Some(1), 640, 0, Rs2Format::Y8, 30)?
            .enable_stream(Rs2StreamKind::Infrared, Some(2), 640, 0, Rs2Format::Y8, 30)?;
        //.enable_stream(Rs2StreamKind::Gyro, None, 0, 0, Rs2Format::Any, 0)?;
    } else {
        config
            .enable_device_from_serial(devices[0].info(Rs2CameraInfo::SerialNumber).unwrap())?
            .disable_all_streams()?
            .enable_stream(Rs2StreamKind::Depth, None, 640, 0, Rs2Format::Z16, 30)?
            .enable_stream(Rs2StreamKind::Infrared, Some(1), 640, 0, Rs2Format::Y8, 30)?
            .enable_stream(Rs2StreamKind::Gyro, None, 0, 0, Rs2Format::Any, 0)?;
    }

    println!("before start");

    // Change pipeline's type from InactivePipeline -> ActivePipeline
    let mut pipeline = pipeline.start(Some(config))?;
    let mut distance = 0.0;
    let mut motion = [0.0, 0.0, 0.0];
    let mut color;
    let mut ir;
    // process frames
    for i in 0..1000 {
        println!("Loop");
        let timeout = Duration::from_millis(5000);
        let frames = pipeline.wait(Some(timeout))?;

        if i % 5 == 0 {
            // Get depth
            let mut depth_frames = frames.frames_of_type::<DepthFrame>();
            if !depth_frames.is_empty() {
                let depth_frame = depth_frames.pop().unwrap();
                let tmp_distance =
                    depth_frame.distance(depth_frame.width() / 2, depth_frame.height() / 2)?;
                if tmp_distance != 0.0 {
                    distance = tmp_distance;
                }
            }

            // Get gyro
            let motion_frames = frames.frames_of_type::<GyroFrame>();
            if !motion_frames.is_empty() {
                motion = *motion_frames[0].rotational_velocity();
            }

            let mut color_frames = frames.frames_of_type::<ColorFrame>();
            if !color_frames.is_empty() {
                let color_frame = color_frames.pop().unwrap();
                color = Option::take(
                    &mut color_frame.get(color_frame.width() / 2, color_frame.height() / 2),
                );
            }

            let mut ir_frames = frames.frames_of_type::<InfraredFrame>();
            if !ir_frames.is_empty() {
                let ir_frame = ir_frames.pop().unwrap();

                ir = ir_frame
                    .get(ir_frame.width() / 2, ir_frame.height() / 2)
                    .take();
            }
        }

        // Print our results
        // print!(
        //     "\rDistance of center pixel: {:<10} m | Gyro reading: {:<15}, {:<15}, {:<15}",
        //     distance, motion[0], motion[1], motion[2]
        // );

        // Print our results
        print!("\rDistance of center pixel: {:<10} m", distance);
        print!("\rColor value center pixel: {:?} m", color);
        print!("\rIR value of center pixel: {:?} m", ir);

        io::stdout().flush().unwrap();
    }

    Ok(())
}
