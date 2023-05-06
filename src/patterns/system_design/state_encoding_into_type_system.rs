use crate::patterns::license::License;


pub struct LicenseProcessInitiate {
    license_type: License,
}

pub struct LicenseProcessInProgress {
    name: String,
    license_type: License,
}

pub struct LicenseProcessInTraining {
    name: String,
    license_type: License,
}

pub struct LicenseProcessRegistered {
    name: String,
    license_type: License,
    signed_by: String,
}

impl LicenseProcessInitiate {
    pub fn new(license_type: License) -> LicenseProcessInitiate {
        LicenseProcessInitiate { license_type }
    }

    pub fn submit_id_proof(self, applicant_name: &str) -> LicenseProcessInProgress {
        LicenseProcessInProgress {
            name: applicant_name.to_owned(),
            license_type: self.license_type,
        }
    }

    pub fn get_license_type(&self) -> &License {
        &self.license_type
    }
}

impl LicenseProcessInProgress {
    pub fn take_test(self) -> LicenseProcessInTraining {
        LicenseProcessInTraining {
            license_type: self.license_type,
            name: self.name,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_license_type(&self) -> &License {
        &self.license_type
    }
}

impl LicenseProcessInTraining {
    pub fn approve(self, signing_authority: &str) -> LicenseProcessRegistered {
        LicenseProcessRegistered {
            license_type: self.license_type,
            name: self.name,
            signed_by: signing_authority.to_owned(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_license_type(&self) -> &License {
        &self.license_type
    }
}

impl LicenseProcessRegistered {
    pub fn get_signature(&self) -> &str {
        &self.signed_by
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_license_type(&self) -> &License {
        &self.license_type
    }
}

pub fn get_driving_license() {
    let license_process = LicenseProcessInitiate::new(License::DRIVING);

    assert_eq!(License::DRIVING, *license_process.get_license_type());

    let driver_name = "TEST_NAME";

    let license_process = license_process.submit_id_proof(driver_name);

    assert_eq!(driver_name, license_process.get_name());

    assert_eq!(License::DRIVING, *license_process.get_license_type());

    let license_process = license_process.take_test();

    assert_eq!(driver_name, license_process.get_name());

    assert_eq!(License::DRIVING, *license_process.get_license_type());

    // this will cause error in type check
    // license_process.get_signature()

    let signee = "RTO";

    let license_process = license_process.approve(signee);

    assert_eq!(driver_name, license_process.get_name());

    assert_eq!(License::DRIVING, *license_process.get_license_type());

    assert_eq!(signee, license_process.get_signature());
}
