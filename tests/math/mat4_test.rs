use pinceau_rs::math::{vec3::Vec3, mat4::Mat4};

#[test]
fn new_matrix() {
    let m = Mat4::new(
        1.0, 2.0, 3.0, 4.0,
        5.0, 6.0, 7.0, 8.0,
        9.0, 10.0, 11.0, 12.0,

        13.0, 14.0, 15.0, 16.0,
    );
    assert_eq!(m.data[0], 1.0);
    assert_eq!(m.data[15], 16.0);
}

#[test]
fn zero_matrix() {
    let zero = Mat4::<f32>::zero();
    assert!(zero.data.iter().all(|&x| x == 0.0));
}

#[test]
fn identity_matrix() {
    let identity = Mat4::<f32>::identity();
    assert_eq!(identity.data[0], 1.0);
    assert_eq!(identity.data[5], 1.0);

    assert_eq!(identity.data[10], 1.0);
    assert_eq!(identity.data[15], 1.0);
}

#[test]
fn addition() {
    let m1 = Mat4::<f32>::identity();
    let m2 = Mat4::<f32>::identity();
    let sum = m1 + m2;
    assert_eq!(sum.data[0], 2.0);
    assert_eq!(sum.data[5], 2.0);
}

#[test]
fn subtraction() {
    let m1 = Mat4::<f32>::identity();

    let m2 = Mat4::<f32>::identity();
    let diff = m1 - m2;
    assert!(diff.data.iter().all(|&x| x == 0.0));
}

#[test]
fn multiplication_scalar() {
    let m = Mat4::<f32>::identity() * 2.0;
    assert_eq!(m.data[0], 2.0);
    assert_eq!(m.data[5], 2.0);

}


#[test]
fn multiplication_matrix() {
    let m1 = Mat4::<f32>::identity();
    let m2 = Mat4::<f32>::identity();
    let prod = m1 * m2;
    assert_eq!(prod, Mat4::<f32>::identity());
}

#[test]
fn transpose() {
    let m = Mat4::new(
        1.0, 2.0, 3.0, 4.0,
        5.0, 6.0, 7.0, 8.0,
        9.0, 10.0, 11.0, 12.0,
        13.0, 14.0, 15.0, 16.0,
    );
    let t = m.transpose();
    assert_eq!(t.data[1], 5.0);
    assert_eq!(t.data[4], 2.0);
}

#[test]
fn determinant() {
    let identity = Mat4::<f32>::identity();
    assert_eq!(identity.determinant(), 1.0);
}

#[test]
fn invertible_matrix() {
    let identity = Mat4::<f32>::identity();
    let inv = identity.invert();
    assert!(inv.is_some());
    assert_eq!(inv.unwrap(), identity);
}

#[test]
fn non_invertible_matrix() {
    let zero = Mat4::<f32>::zero();
    let inv = zero.invert();
    assert!(inv.is_none());
}

#[test]
fn rotation() {
    let axis = Vec3::new(0.0, 1.0, 0.0);
    let rotated = Mat4::<f32>::identity().rotate(axis, 90.0);
    assert!(rotated.data.iter().any(|&x| x != 0.0));
}

#[test]
fn scaling() {
    let scaled = Mat4::<f32>::identity().scale(2.0, 3.0, 4.0);
    assert_eq!(scaled.data[0], 2.0);
    assert_eq!(scaled.data[5], 3.0);
    assert_eq!(scaled.data[10], 4.0);
}

#[test]
fn translation() {
    let translated = Mat4::<f32>::identity().translate(1.0, 2.0, 3.0);

    assert_eq!(translated.data[3], 1.0);
    assert_eq!(translated.data[7], 2.0);
    assert_eq!(translated.data[11], 3.0);
}

#[test]
fn perspective_projection() {
    let persp = Mat4::<f32>::perspective(90.0, 1.0, 0.1, 100.0);
    assert!(persp.data.iter().any(|&x| x != 0.0));
}

#[test]
fn orthogonal_projection() {
    let ortho = Mat4::<f32>::orthogonal(1.0, -1.0, -1.0, 1.0, 0.1, 100.0);
    assert!(ortho.data.iter().any(|&x| x != 0.0));
}
