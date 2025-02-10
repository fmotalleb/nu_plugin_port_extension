use nu_plugin::EvaluatedCall;

pub trait FlagHelper {
    fn has_flag_or(&self, flag: &str, default: bool) -> bool;
    fn missing_flag_or(&self, flag: &str, default: bool) -> bool {
        !self.has_flag_or(flag, !default)
    }
}
impl FlagHelper for &EvaluatedCall {
    fn has_flag_or(&self, flag: &str, default: bool) -> bool {
        self.has_flag(flag).unwrap_or(default)
    }
}
