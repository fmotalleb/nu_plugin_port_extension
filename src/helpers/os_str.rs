pub trait ToStr {
    fn to_string(&self) -> String;
}

impl ToStr for std::ffi::OsStr {
    fn to_string(&self) -> String {
        self.to_string_lossy().to_string()
    }
}
