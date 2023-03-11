use crate::patterns::license::{License, LicenseProcess};

pub fn get_driving_license() {
    let mut driving_license_process = LicenseProcess::new(License::DRIVING);

    assert_eq!("", driving_license_process.get_signature());

    let driver_name = "TEST_NAME".to_owned();

    driving_license_process.submit_id_proof(&driver_name);

    assert_eq!(driver_name, driving_license_process.get_name());

    assert_eq!("", driving_license_process.get_signature());

    driving_license_process.take_test();

    assert_eq!("", driving_license_process.get_signature());

    let signee = "RTO";

    driving_license_process.approve(signee);

    assert_eq!(signee, driving_license_process.get_signature());
}
