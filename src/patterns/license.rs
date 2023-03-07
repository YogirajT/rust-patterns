pub enum License {
  DRIVING
}

pub enum Authority {
  RTO
}

pub struct LicenseProcess {
  state: Option<Box<dyn ProcessState>>,
  name: String,
  license_type: License,
  signed: Option<Authority>
}

impl LicenseProcess {
  pub fn new(license_type: License) -> LicenseProcess {
    LicenseProcess {
      state: None,
      name: String::new(),
      license_type,
      signed: None
    }
  }
}

trait ProcessState {}

struct Registered {}

impl ProcessState for Registered {

}