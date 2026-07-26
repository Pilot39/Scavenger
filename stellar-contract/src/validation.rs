pub fn validate_coordinates(latitude: i128, longitude: i128) {
    const MAX_LAT: i128 = 90_000_000;
    const MAX_LON: i128 = 180_000_000;

    if latitude < -MAX_LAT || latitude > MAX_LAT {
        panic!("Latitude must be between -90 and +90 degrees");
    }

    if longitude < -MAX_LON || longitude > MAX_LON {
        panic!("Longitude must be between -180 and +180 degrees");
    }
}
