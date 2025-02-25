use pinceau_rs::math::vec3::Vec3;

#[test]
fn creation() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    assert_eq!(v.x, 1.0);
    assert_eq!(v.y, 2.0);
    assert_eq!(v.z, 3.0);
}

#[test]
fn addition() {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(4.0, 5.0, 6.0);
    let v3 = v1 + v2;
    assert_eq!(v3, Vec3::new(5.0, 7.0, 9.0));
}

#[test]
fn addition_scalar() {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let scalar = 1.0;
    let v3 = v1 + scalar;
    assert_eq!(v3, Vec3::new(2.0, 3.0, 4.0));
}

#[test]
fn subtraction() {
    let v1 = Vec3::new(4.0, 5.0, 6.0);
    let v2 = Vec3::new(1.0, 2.0, 3.0);

    let v3 = v1 - v2;
    assert_eq!(v3, Vec3::new(3.0, 3.0, 3.0));
}

#[test]
fn subtraction_scalar() {
    let v1 = Vec3::new(4.0, 5.0, 6.0);
    let scalar = 1.0;

    let v3 = v1 - scalar;
    assert_eq!(v3, Vec3::new(3.0, 4.0, 5.0));
}

#[test]
fn scalar_multiplication() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    let v2 = v * 2.0;
    assert_eq!(v2, Vec3::new(2.0, 4.0, 6.0));
}

#[test]
fn scalar_division() {
    let v = Vec3::new(2.0, 4.0, 6.0);
    let v2 = v / 2.0;
    assert_eq!(v2, Vec3::new(1.0, 2.0, 3.0));
}

#[test]
#[should_panic(expected = "Divid by 0")]
fn division_by_zero() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    let _ = v / 0.0;
}

#[test]
fn dot_product() {
    let v1 = Vec3::new(1.0, 2.0, 3.0);
    let v2 = Vec3::new(4.0, 5.0, 6.0);
    let result = v1.dot(&v2);

    assert_eq!(result, 32.0);
}

#[test]
fn cross_product() {
    let v1 = Vec3::new(1.0, 0.0, 0.0);
    let v2 = Vec3::new(0.0, 1.0, 0.0);
    let cross = v1.cross(&v2);
    assert_eq!(cross, Vec3::new(0.0, 0.0, 1.0));
}

#[test]
fn magnitude() {
    let v = Vec3::new(3.0, 4.0, 0.0);
    assert_eq!(v.magnitude(), 5.0);
}

#[test]
fn normalize() {
    let v = Vec3::new(3.0, 4.0, 0.0);
    let normalized = v.normalize();
    let expected = Vec3::new(0.6, 0.8, 0.0);
    assert!((normalized.x - expected.x).abs() < 1e-6);
    assert!((normalized.y - expected.y).abs() < 1e-6);
    assert!((normalized.z - expected.z).abs() < 1e-6);
}

#[test]
fn normalize_zero_vector() {
    let v = Vec3::new(0.0, 0.0, 0.0);

    let normalized = v.normalize();
    assert_eq!(normalized, Vec3::new(0.0, 0.0, 0.0));
}
