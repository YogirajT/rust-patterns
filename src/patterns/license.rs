pub enum License {
    DRIVING,
}

#[derive(Debug, PartialEq)]
pub enum Authority {
    RTO,
}

pub struct LicenseProcess {
    state: Option<Box<dyn ProcessState>>,
    name: String,
    license_type: License,
    signed: Option<Authority>,
}

impl LicenseProcess {
    pub fn new(license_type: License) -> LicenseProcess {
        LicenseProcess {
            state: Some(Box::new(Initiated {})),
            name: String::new(),
            license_type,
            signed: None,
        }
    }

    pub fn add_signature(&mut self, authority: Authority) {
        self.signed = Some(authority)
    }

    pub fn change_name(&mut self, new_name: &str) {
        self.name = new_name.to_owned()
    }

    pub fn get_signature(&self) -> Option<Authority> {
        self.signed
    }

    pub fn submit_id_proof(&mut self, name: &str) {
        if self.name.is_empty() {
            self.name.push_str(name);
        }
    }

    pub fn get_name() {
        todo!()
    }
}

trait ProcessState {}

struct Registered {}

struct Initiated {}

impl ProcessState for Initiated {}

impl ProcessState for Registered {}
