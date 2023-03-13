#[derive(Debug, PartialEq)]
pub enum License {
    DRIVING,
}

pub struct LicenseProcess {
    state: Option<Box<dyn ProcessState>>,
    name: String,
    license_type: License,
    signed_by: String,
}

pub trait ProcessState {
    fn id_proof_submitted(self: Box<Self>) -> Box<dyn ProcessState>;
    fn initiate_training(self: Box<Self>) -> Box<dyn ProcessState>;
    fn register(self: Box<Self>) -> Box<dyn ProcessState>;
    fn signed_by<'a>(&self, _license_process: &'a LicenseProcess) -> &'a str {
        ""
    }
}
