use os_info;
use platform_info::{PlatformInfo, PlatformInfoAPI, UNameAPI};

pub struct DeviceInfo {
    pub os_type: String,
    pub os_version: String,
    pub arch: String,
    pub kernel_version: String,
}

impl DeviceInfo {
    pub fn new() -> Self {
        let info = os_info::get();

        let kernel_version = match PlatformInfo::new() {
            Ok(p) => p.release().to_string_lossy().into_owned(),
            Err(_) => "unknown_kernel".to_string(),
        };

        let os_type = format!("{:?}", info.os_type());
        let os_version = info.version().to_string();
        let arch = info.architecture().unwrap_or("unknown").to_string();

        Self {
            os_type,
            os_version,
            arch,
            kernel_version,
        }
    }

    pub fn id(&self) -> String {
        format!(
            "{}-{}-{}-{}",
            self.os_type.to_uppercase(),
            self.os_version,
            self.arch,
            self.kernel_version
        )
    }

    pub fn name(&self) -> String {
        format!("{} {} ({})", self.os_type, self.os_version, self.arch)
    }
}
