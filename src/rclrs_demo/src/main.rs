use rclrs::*;
use rerun as rr;
use ros_pointcloud2::{points::PointXYZ, PointCloud2Msg};
use sensor_msgs::msg::PointCloud2;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rec = rr::RecordingStreamBuilder::new("lidar_visualizer").spawn()?;

    let context = Context::default_from_env()?;
    let mut executor = context.create_basic_executor();
    let node = executor.create_node("point_cloud_listener")?;

    let _subscription =
        node.create_subscription::<PointCloud2, _>("/scan/points", move |msg: PointCloud2| {
            let new_msg = PointCloud2Msg::from(msg);
            let cloud = new_msg
                .try_into_iter()
                .unwrap()
                .map(|point_xyz: PointXYZ| rr::Vec3D::new(point_xyz.x, point_xyz.y, point_xyz.z))
                .collect::<Vec<rr::Vec3D>>();
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
