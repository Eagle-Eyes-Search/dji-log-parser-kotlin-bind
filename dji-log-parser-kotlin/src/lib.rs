use dji_log_parser::keychain::FeaturePoint;
use dji_log_parser::layout::details::Platform;
use dji_log_parser::layout::details::ProductType;
use dji_log_parser::DJILog;
use std::sync::Arc;
use std::vec::Vec;
use thiserror::Error;

uniffi::setup_scaffolding!();

// Define a proper error enum for UniFFI
#[derive(Debug, Clone, Error, uniffi::Error)]
pub enum DJIError {
    #[error("Failed to parse DJI log")]
    ParseError,
    #[error("Failed to fetch keychains")]
    KeychainError,
    #[error("Failed to process records")]
    RecordError,
    #[error("Failed to process frames")]
    FrameError,
}

// Define the UniFFI interface types

// ProductType enum
#[derive(Debug, Clone, Copy, uniffi::Enum)]
pub enum ProductTypeWrapper {
    None,
    Inspire1,
    Phantom3Standard,
    Phantom3Advanced,
    Phantom3Pro,
    OSMO,
    Matrice100,
    Phantom4,
    LB2,
    Inspire1Pro,
    A3,
    Matrice600,
    Phantom34K,
    MavicPro,
    ZenmuseXT,
    // ... other product types
    Unknown,
}

// Platform enum
#[derive(Debug, Clone, Copy, uniffi::Enum)]
pub enum PlatformWrapper {
    IOS,
    Android,
    DJIFly,
    Windows,
    Mac,
    Linux,
    Unknown,
}

// Convert from DJI types to our wrappers
impl From<ProductType> for ProductTypeWrapper {
    fn from(product_type: ProductType) -> Self {
        match product_type {
            ProductType::None => ProductTypeWrapper::None,
            ProductType::Inspire1 => ProductTypeWrapper::Inspire1,
            ProductType::Phantom3Standard => ProductTypeWrapper::Phantom3Standard,
            ProductType::Phantom3Advanced => ProductTypeWrapper::Phantom3Advanced,
            ProductType::Phantom3Pro => ProductTypeWrapper::Phantom3Pro,
            ProductType::OSMO => ProductTypeWrapper::OSMO,
            ProductType::Matrice100 => ProductTypeWrapper::Matrice100,
            ProductType::Phantom4 => ProductTypeWrapper::Phantom4,
            ProductType::LB2 => ProductTypeWrapper::LB2,
            ProductType::Inspire1Pro => ProductTypeWrapper::Inspire1Pro,
            ProductType::A3 => ProductTypeWrapper::A3,
            ProductType::Matrice600 => ProductTypeWrapper::Matrice600,
            ProductType::Phantom34K => ProductTypeWrapper::Phantom34K,
            ProductType::MavicPro => ProductTypeWrapper::MavicPro,
            ProductType::ZenmuseXT => ProductTypeWrapper::ZenmuseXT,
            _ => ProductTypeWrapper::Unknown,
        }
    }
}

impl From<Platform> for PlatformWrapper {
    fn from(platform: Platform) -> Self {
        match platform {
            Platform::IOS => PlatformWrapper::IOS,
            Platform::Android => PlatformWrapper::Android,
            Platform::DJIFly => PlatformWrapper::DJIFly,
            Platform::Windows => PlatformWrapper::Windows,
            Platform::Mac => PlatformWrapper::Mac,
            Platform::Linux => PlatformWrapper::Linux,
            _ => PlatformWrapper::Unknown,
        }
    }
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct DetailsWrapper {
    pub sub_street: String,
    pub street: String,
    pub city: String,
    pub area: String,
    pub is_favorite: u8,
    pub is_new: u8,
    pub needs_upload: u8,
    pub record_line_count: i32,
    pub detail_info_checksum: i32,
    pub start_time: String,
    pub longitude: f64,
    pub latitude: f64,
    pub total_distance: f32,
    pub total_time: f64,
    pub max_height: f32,
    pub max_horizontal_speed: f32,
    pub max_vertical_speed: f32,
    pub aircraft_name: String,
    pub aircraft_sn: String,
    pub camera_sn: String,
    pub rc_sn: String,
    pub battery_sn: String,
    pub app_platform: PlatformWrapper,
    pub app_version: String,
    pub product_type: ProductTypeWrapper,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct KeychainFeaturePointWrapper {
    pub feature_point: u32,
    pub aes_key: String,
    pub aes_iv: String,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct EncodedKeychainFeaturePointWrapper {
    pub feature_point: u32,
    pub aes_ciphertext: String,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct KeychainsRequestWrapper {
    pub version: u16,
    pub department: u8,
    pub keychains: Vec<Vec<EncodedKeychainFeaturePointWrapper>>,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct RecordWrapper {
    pub record_type: String,
    pub timestamp: u64,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct FrameWrapper {
    // OSD section
    pub fly_time: f32,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f32,
    pub height: f32,
    pub x_speed: f32,
    pub y_speed: f32,
    pub z_speed: f32,
    pub pitch: f32,
    pub roll: f32,
    pub yaw: f32,
    pub gps_num: u8,

    // Gimbal section
    pub gimbal_pitch: f32,
    pub gimbal_roll: f32,
    pub gimbal_yaw: f32,

    // Camera section
    pub is_recording: bool,
    pub is_taking_photo: bool,

    // RC section
    pub aileron: u16,
    pub elevator: u16,
    pub throttle: u16,
    pub rudder: u16,

    // Battery section
    pub battery_percent: u8,
    pub battery_voltage: f32,
    pub battery_current: f32,
    pub battery_temperature: f32,
    pub cell_voltages: Vec<f32>,

    // Home section
    pub home_latitude: f64,
    pub home_longitude: f64,
    pub home_altitude: f32,
}

// Helper function to convert u32 to FeaturePoint
fn u32_to_feature_point(value: u32) -> FeaturePoint {
    match value {
        1 => FeaturePoint::BaseFeature,
        2 => FeaturePoint::VisionFeature,
        3 => FeaturePoint::WaypointFeature,
        4 => FeaturePoint::AgricultureFeature,
        5 => FeaturePoint::AirLinkFeature,
        6 => FeaturePoint::AfterSalesFeature,
        7 => FeaturePoint::DJIFlyCustomFeature,
        8 => FeaturePoint::PlaintextFeature,
        9 => FeaturePoint::FlightHubFeature,
        10 => FeaturePoint::GimbalFeature,
        11 => FeaturePoint::RCFeature,
        12 => FeaturePoint::CameraFeature,
        13 => FeaturePoint::BatteryFeature,
        14 => FeaturePoint::FlySafeFeature,
        15 => FeaturePoint::SecurityFeature,
        _ => FeaturePoint::BaseFeature, // Default to base feature for unknown values
    }
}

/// A wrapper around the DJI log parser for Kotlin bindings
#[derive(uniffi::Object)]
pub struct DJILogWrapper {
    inner: DJILog,
}

#[uniffi::export]
impl DJILogWrapper {
    /// Constructs a `DJILog` from an array of bytes.
    ///
    /// This function parses the Prefix and Info blocks of the log file,
    /// and handles different versions of the log format.
    #[uniffi::constructor]
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Arc<Self>, DJIError> {
        DJILog::from_bytes(bytes)
            .map(|log| Arc::new(Self { inner: log }))
            .map_err(|_| DJIError::ParseError)
    }

    /// Get the log format version
    pub fn version(&self) -> u8 {
        self.inner.version
    }

    /// Get details about the log
    pub fn details(&self) -> DetailsWrapper {
        let details = &self.inner.details;
        DetailsWrapper {
            sub_street: details.sub_street.clone(),
            street: details.street.clone(),
            city: details.city.clone(),
            area: details.area.clone(),
            is_favorite: details.is_favorite,
            is_new: details.is_new,
            needs_upload: details.needs_upload,
            record_line_count: details.record_line_count,
            detail_info_checksum: details.detail_info_checksum,
            start_time: details.start_time.to_string(),
            longitude: details.longitude,
            latitude: details.latitude,
            total_distance: details.total_distance,
            total_time: details.total_time,
            max_height: details.max_height,
            max_horizontal_speed: details.max_horizontal_speed,
            max_vertical_speed: details.max_vertical_speed,
            aircraft_name: details.aircraft_name.clone(),
            aircraft_sn: details.aircraft_sn.clone(),
            camera_sn: details.camera_sn.clone(),
            rc_sn: details.rc_sn.clone(),
            battery_sn: details.battery_sn.clone(),
            app_platform: details.app_platform.clone().into(),
            app_version: details.app_version.clone(),
            product_type: details.product_type.into(),
        }
    }

    /// Creates a KeychainsRequest object by parsing KeyStorage records
    pub fn keychains_request(&self) -> Result<KeychainsRequestWrapper, DJIError> {
        self.inner
            .keychains_request()
            .map_err(|_| DJIError::KeychainError)
            .map(|req| {
                let keychains = req
                    .keychains
                    .into_iter()
                    .map(|chain| {
                        chain
                            .into_iter()
                            .map(|point| EncodedKeychainFeaturePointWrapper {
                                feature_point: point.feature_point as u32,
                                aes_ciphertext: point.aes_ciphertext,
                            })
                            .collect()
                    })
                    .collect();

                KeychainsRequestWrapper {
                    version: req.version,
                    department: req.department,
                    keychains,
                }
            })
    }

    /// Fetches keychains using the provided API key
    #[cfg(not(target_arch = "wasm32"))]
    pub fn fetch_keychains(
        &self,
        api_key: String,
    ) -> Result<Vec<Vec<KeychainFeaturePointWrapper>>, DJIError> {
        self.inner
            .fetch_keychains(&api_key)
            .map_err(|_| DJIError::KeychainError)
            .map(|chains| {
                chains
                    .into_iter()
                    .map(|chain| {
                        chain
                            .into_iter()
                            .map(|point| KeychainFeaturePointWrapper {
                                feature_point: point.feature_point as u32,
                                aes_key: point.aes_key,
                                aes_iv: point.aes_iv,
                            })
                            .collect()
                    })
                    .collect()
            })
    }

    /// Retrieves the parsed raw records from the DJI log
    pub fn records(
        &self,
        keychains: Option<Vec<Vec<KeychainFeaturePointWrapper>>>,
    ) -> Result<Vec<RecordWrapper>, DJIError> {
        // Convert wrapper keychains to the original format if provided
        let original_keychains = match keychains {
            Some(chains) => {
                let mut result = Vec::new();
                for chain in chains {
                    let mut original_chain = Vec::new();
                    for point in chain {
                        // Convert u32 to FeaturePoint using helper function
                        let feature_point = u32_to_feature_point(point.feature_point);
                        original_chain.push(dji_log_parser::keychain::KeychainFeaturePoint {
                            feature_point,
                            aes_key: point.aes_key,
                            aes_iv: point.aes_iv,
                        });
                    }
                    result.push(original_chain);
                }
                Some(result)
            }
            None => None,
        };

        self.inner
            .records(original_keychains)
            .map_err(|_| DJIError::RecordError)
            .map(|records| {
                records
                    .into_iter()
                    .map(|record| {
                        let record_type = format!("{:?}", record);
                        RecordWrapper {
                            record_type,
                            timestamp: 0,     // Simplified for interface
                            data: Vec::new(), // Simplified for interface
                        }
                    })
                    .collect()
            })
    }

    /// Retrieves the normalized frames from the DJI log
    pub fn frames(
        &self,
        keychains: Option<Vec<Vec<KeychainFeaturePointWrapper>>>,
    ) -> Result<Vec<FrameWrapper>, DJIError> {
        // Convert wrapper keychains to the original format if provided
        let original_keychains = match keychains {
            Some(chains) => {
                let mut result = Vec::new();
                for chain in chains {
                    let mut original_chain = Vec::new();
                    for point in chain {
                        // Convert u32 to FeaturePoint using helper function
                        let feature_point = u32_to_feature_point(point.feature_point);
                        original_chain.push(dji_log_parser::keychain::KeychainFeaturePoint {
                            feature_point,
                            aes_key: point.aes_key,
                            aes_iv: point.aes_iv,
                        });
                    }
                    result.push(original_chain);
                }
                Some(result)
            }
            None => None,
        };

        self.inner
            .frames(original_keychains)
            .map_err(|_| DJIError::FrameError)
            .map(|frames| {
                frames
                    .into_iter()
                    .map(|frame| {
                        // Convert cell voltages to Vec<f32>
                        let cell_voltages = if !frame.battery.cell_voltages.is_empty() {
                            frame.battery.cell_voltages.to_vec()
                        } else {
                            Vec::new()
                        };

                        FrameWrapper {
                            // OSD data
                            fly_time: frame.osd.fly_time,
                            latitude: frame.osd.latitude,
                            longitude: frame.osd.longitude,
                            altitude: frame.osd.altitude,
                            height: frame.osd.height,
                            x_speed: frame.osd.x_speed,
                            y_speed: frame.osd.y_speed,
                            z_speed: frame.osd.z_speed,
                            pitch: frame.osd.pitch,
                            roll: frame.osd.roll,
                            yaw: frame.osd.yaw,
                            gps_num: frame.osd.gps_num,

                            // Gimbal data
                            gimbal_pitch: frame.gimbal.pitch,
                            gimbal_roll: frame.gimbal.roll,
                            gimbal_yaw: frame.gimbal.yaw,

                            // Camera data
                            is_recording: frame.camera.is_video,
                            is_taking_photo: frame.camera.is_photo,

                            // RC data
                            aileron: frame.rc.aileron as u16,
                            elevator: frame.rc.elevator as u16,
                            throttle: frame.rc.throttle as u16,
                            rudder: frame.rc.rudder as u16,

                            // Battery data
                            battery_percent: frame.battery.charge_level,
                            battery_voltage: frame.battery.voltage,
                            battery_current: frame.battery.current,
                            battery_temperature: frame.battery.temperature,
                            cell_voltages,

                            // Home data
                            home_latitude: frame.home.latitude,
                            home_longitude: frame.home.longitude,
                            home_altitude: frame.home.altitude,
                        }
                    })
                    .collect()
            })
    }
}
