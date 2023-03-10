use crate::patterns::license::{Authority, License, LicenseProcess};

pub fn get_driving_license() {
    let mut driving_license_process = LicenseProcess::new(License::DRIVING);

    assert_eq!(None, driving_license_process.get_signature());

    let driver_name = "TEST_NAME".to_owned();

    driving_license_process.submit_id_proof(&driver_name);

    assert_eq!(driver_name, driving_license_process.get_name());

    assert_eq!(None, driving_license_process.get_signature());

    driving_license_process.take_test();

    assert_eq!(None, driving_license_process.get_signature());

    driving_license_process.get_approval();

    assert_eq!(Authority::RTO, driving_license_process.get_signature());
}
