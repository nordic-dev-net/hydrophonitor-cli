// Is used to select which devices  of which type should be shown in the connect wizard.
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