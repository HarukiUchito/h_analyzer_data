use std::collections::HashMap;

pub mod grpc_fs {
    tonic::include_proto!("grpc_fs");
}

pub mod grpc_data_transfer {
    tonic::include_proto!("grpc_data_transfer");
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct WheelVelocity {
    value: f64,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Imu {
    angular_rates: Option<[f64; 3]>,
    accelerations: Option<[f64; 3]>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Point2D {
    x: f64,
    y: f64,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Pose2D {
    position: Point2D,
    theta: f64,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum Measurement {
    WheelVelocity(WheelVelocity),
    IMU(Imu),
    Pose2D(Pose2D),
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum Estimate {
    Pose2D(Pose2D),
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Entity {
    measurement_map: std::collections::HashMap<String, Measurement>,
    estimate_map: std::collections::HashMap<String, Estimate>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct WorldFrame {
    timestamp: f64, // unix time in sec
    entity_map: std::collections::HashMap<String, Entity>,
}

impl WorldFrame {
    fn new(timestamp: f64) -> Self {
        Self {
            timestamp: timestamp,
            entity_map: HashMap::new(),
        }
    }
}
