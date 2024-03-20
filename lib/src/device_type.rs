pub enum DeviceType {
    Part,
    Disk,
}

impl DeviceType {
    pub fn as_str(&self) -> &str{
        match self {
            DeviceType::Disk => "disk",
            DeviceType::Part => "part",
        }
    }
}