pub enum License {
    DRIVING,
}

pub struct LicenseProcess {
    state: Option<Box<dyn ProcessState>>,
    name: String,
    license_type: License,
    signed_by: String,
}

impl LicenseProcess {
    pub fn new(license_type: License) -> LicenseProcess {
        LicenseProcess {
            state: Some(Box::new(Initiated {})),
            name: String::new(),
            license_type,
            signed_by: String::new(),
        }
    }

    pub fn change_name(&mut self, new_name: &str) {
        self.name = new_name.to_owned()
    }

    pub fn get_signature(&self) -> &str {
        self.state.as_ref().unwrap().signed_by(self)
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn submit_id_proof(&mut self, name: &str) {
        if self.name.is_empty() {
            if let Some(state) = self.state.take() {
                self.state = Some(state.id_proof_submitted());
            }
            self.name.push_str(name);
        }
    }

    pub fn take_test(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.initiate_training());
        }
    }

    pub fn approve(&mut self, authority: &str) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.register());
            self.signed_by.push_str(authority);
        }
    }
}

trait ProcessState {
    fn id_proof_submitted(self: Box<Self>) -> Box<dyn ProcessState>;
    fn initiate_training(self: Box<Self>) -> Box<dyn ProcessState>;
    fn register(self: Box<Self>) -> Box<dyn ProcessState>;
    fn signed_by<'a>(&self, _license_process: &'a LicenseProcess) -> &'a str {
        ""
    }
}

struct Initiated {}

struct InProgress {}

struct InTraining {}

struct Registered {}

impl ProcessState for Initiated {
    fn id_proof_submitted(self: Box<Self>) -> Box<dyn ProcessState> {
        Box::new(InProgress {})
    }
    fn initiate_training(self: Box<Self>) -> Box<dyn ProcessState> {
        self
    }
    fn register(self: Box<Self>) -> Box<dyn ProcessState> {
        self
    }
}

impl ProcessState for InProgress {
    fn id_proof_submitted(self: Box<Self>) -> Box<dyn ProcessState> {
        self
    }
    fn initiate_training(self: Box<Self>) -> Box<dyn ProcessState> {
        Box::new(InTraining {})
    }
    fn register(self: Box<Self>) -> Box<dyn ProcessState> {
        self
    }
}

impl ProcessState for InTraining {
    fn id_proof_submitted(self: Box<Self>) -> Box<dyn ProcessState> {
        self
    }
    fn initiate_training(self: Box<Self>) -> Box<dyn ProcessState> {
        self
    }
    fn register(self: Box<Self>) -> Box<dyn ProcessState> {
        Box::new(Registered {})
    }
}

impl ProcessState for Registered {
    fn id_proof_submitted(self: Box<Self>) -> Box<dyn ProcessState> {
        self
    }
    fn initiate_training(self: Box<Self>) -> Box<dyn ProcessState> {
        self
    }
    fn register(self: Box<Self>) -> Box<dyn ProcessState> {
        self
    }
    fn signed_by<'a>(&self, license_process: &'a LicenseProcess) -> &'a str {
        &license_process.signed_by
    }
}
