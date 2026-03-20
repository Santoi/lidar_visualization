use rclrs::*;
use rerun::{self as rr, external::re_grpc_server};
use sensor_msgs::msg::PointCloud2;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rec = rr::RecordingStreamBuilder::new("lidar_visualizer").serve_web(
        "0.0.0.0",
        Default::default(),
        re_grpc_server::DEFAULT_SERVER_PORT,
        rr::MemoryLimit::UNLIMITED,
        true,
    )?;

    let context = Context::default_from_env()?;
    let mut executor = context.create_basic_executor();
    let node = executor.create_node("point_cloud_listener")?;

    let _subscription =
        node.create_subscription::<PointCloud2, _>("/scan/points", move |msg: PointCloud2| {
            let mut cloud = Vec::new();
            // Each point occupies `point_step` bytes in the flat data buffer.
            let point_step = msg.point_step as usize;
            let data = &msg.data;

            // Look up the byte offset of each coordinate field within a single point.
            // Defaults assume the common layout: three consecutive f32s (x at 0, y at 4, z at 8).
            let x_offset = msg
                .fields
                .iter()
                .find(|f| f.name == "x")
                .map(|f| f.offset as usize)
                .unwrap_or(0);
            let y_offset = msg
                .fields
                .iter()
                .find(|f| f.name == "y")
                .map(|f| f.offset as usize)
                .unwrap_or(4);
            let z_offset = msg
                .fields
                .iter()
                .find(|f| f.name == "z")
                .map(|f| f.offset as usize)
                .unwrap_or(8);

            // Iterate over each point in the buffer, stepping by point_step bytes.
            for i in (0..data.len()).step_by(point_step) {
                if i + point_step > data.len() {
                    continue;
                }
                let point_data = &data[i..i + point_step];
                // Bounds-check before reading 4 bytes (size of f32) at each offset.
                if x_offset + 4 > point_data.len()
                    || y_offset + 4 > point_data.len()
                    || z_offset + 4 > point_data.len()
                {
                    continue;
                }
                // Read each coordinate as a little-endian f32 from the point's byte slice.
                let x = f32::from_le_bytes(point_data[x_offset..x_offset + 4].try_into().unwrap());
                let y = f32::from_le_bytes(point_data[y_offset..y_offset + 4].try_into().unwrap());
                let z = f32::from_le_bytes(point_data[z_offset..z_offset + 4].try_into().unwrap());
                cloud.push(rr::Vec3D::new(x, y, z));
            }

            rec.log("point_cloud", &rr::Points3D::new(cloud))
                .expect("Failed to log points");
        })?;

    rclrs::log_info!(
        "point_cloud_listener",
        "Waiting for messages on '/scan/points'..."
    );
    executor.spin(SpinOptions::default()).first_error()?;

    Ok(())
}
