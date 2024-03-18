use lambdaworks_math::{
    cyclic_group::IsGroup,
    elliptic_curve::{
        short_weierstrass::{
            curves::bls12_381::curve::BLS12381Curve, point::ShortWeierstrassProjectivePoint,
        },
        traits::IsEllipticCurve,
    },
};

// Double-and-add algorithm from wikipedia
// https://en.wikipedia.org/wiki/Elliptic_curve_point_multiplication
fn scalar_mul(
    point: ShortWeierstrassProjectivePoint<BLS12381Curve>,
    scalar: u64,
) -> ShortWeierstrassProjectivePoint<BLS12381Curve> {
    let mut result = ShortWeierstrassProjectivePoint::<BLS12381Curve>::neutral_element();
    let bits: Vec<bool> = (0..64).map(move |i| (scalar >> i) & 1 == 1).collect();
    let mut temp = point;
    for bit in bits {
        if bit == true {
            result = result.operate_with(&temp);
        }
        temp = temp.operate_with(&temp);
    }
    result
}

fn main() {
    let secret_key: u64 = 0x6C616D6264617370;
    let generator = BLS12381Curve::generator();

    let public_key = scalar_mul(generator, secret_key);
    println!("x: {:?}", public_key.x().to_hex());
    println!("y: {:?}", public_key.y().to_hex());
    println!("z: {:?}", public_key.z().to_hex());
}
