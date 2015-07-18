use matrix::format::{Compressed, Conventional};

#[cfg(feature = "hotspot")]
mod hotspot;

#[test]
fn multiply_matrix_matrix() {
    let A = Compressed::from(Conventional::from_vec((4, 3), vec![
        1.0, 2.0, 3.0, 4.0,
        5.0, 6.0, 6.0, 5.0,
        4.0, 3.0, 2.0, 1.0,
    ]));

    let B = vec![
        1.0, 2.0, 3.0,
        4.0, 5.0, 6.0,
    ];

    let mut C = vec![1.0; 4 * 2];

    super::multiply_matrix_matrix(&A, &B, &mut C);

    assert_eq!(C, vec![
        24.0, 24.0, 22.0, 18.0,
        54.0, 57.0, 55.0, 48.0,
    ]);
}
