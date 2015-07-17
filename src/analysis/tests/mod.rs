use matrix::{Compressed, Conventional};

#[cfg(feature = "hotspot")]
mod hotspot;

#[test]
fn multiply_matrix_matrix() {
    let A = Compressed::from(Conventional::from_vec(vec![
        1.0, 2.0, 3.0, 4.0,
        5.0, 6.0, 6.0, 5.0,
        4.0, 3.0, 2.0, 1.0,
    ], (4, 3)));

    let B = vec![
        1.0, 2.0, 3.0,
        4.0, 5.0, 6.0,
    ];

    let mut C = vec![0.0; 4 * 2];

    super::multiply_matrix_matrix(&A, &B, &mut C);

    assert_eq!(C, vec![
        23.0, 23.0, 21.0, 17.0,
        53.0, 56.0, 54.0, 47.0,
    ]);
}
