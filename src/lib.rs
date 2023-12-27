use std::collections::HashMap;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Vec3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x: x, y: y, z: z }
    }
}

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
    angular_rates: Option<Vec3D>,
    accelerations: Option<Vec3D>,
}

impl Imu {
    pub fn new(ar_opt: Option<Vec3D>, acc_opt: Option<Vec3D>) -> Self {
        Self {
            angular_rates: ar_opt,
            accelerations: acc_opt,
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Point2D {
    x: f64,
    y: f64,
}

impl Point2D {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x: x, y: y }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Pose2D {
    position: Point2D,
    theta: f64,
}

impl Pose2D {
    pub fn new(x: f64, y: f64, theta: f64) -> Self {
        Self {
            position: Point2D::new(x, y),
            theta: theta,
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct PointCloud2D {
    points: Vec<Point2D>,
}

impl PointCloud2D {
    pub fn new(x_vec: Vec<f64>, y_vec: Vec<f64>) -> Self {
        Self {
            points: x_vec
                .iter()
                .zip(y_vec.iter())
                .map(|(&x, &y)| Point2D::new(x, y))
                .collect(),
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum Measurement {
    WheelVelocity(WheelVelocity),
    IMU(Imu),
    PointCloud2D(PointCloud2D),
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

impl Entity {
    pub fn new() -> Self {
        Self {
            measurement_map: HashMap::new(),
            estimate_map: HashMap::new(),
        }
    }

    pub fn add_measurement(&mut self, name: String, m: Measurement) {
        self.measurement_map.insert(name, m);
    }

    pub fn add_estimate(&mut self, name: String, estimate: Estimate) {
        self.estimate_map.insert(name, estimate);
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct WorldFrame {
    timestamp: f64, // unix time in sec
    entity_map: std::collections::HashMap<String, Entity>,
}

impl WorldFrame {
    pub fn new(timestamp: f64) -> Self {
        Self {
            timestamp: timestamp,
            entity_map: HashMap::new(),
        }
    }

    pub fn add_entity(&mut self, name: String, entity: Entity) {
        self.entity_map.insert(name, entity);
    }
}
