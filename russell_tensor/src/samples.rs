use super::SQRT_2;

/// Holds samples
pub struct SamplesTensor4;

impl SamplesTensor4 {
    pub const SAMPLE1: [[[[f64; 3]; 3]; 3]; 3] = [
        // [0]
        [
            // [0][0]
            [
                [1.0, 10.0, 12.0], // [0][0][0][...]
                [19.0, 2.0, 11.0], // [0][0][1][...]
                [21.0, 20.0, 3.0], // [0][0][2][...]
            ],
            // [0][1]
            [
                [28.0, 37.0, 39.0], // [0][1][0][...]
                [46.0, 29.0, 38.0], // [0][1][1][...]
                [48.0, 47.0, 30.0], // [0][1][2][...]
            ],
            // [0][2]
            [
                [34.0, 43.0, 45.0], // [0][2][0][...]
                [52.0, 35.0, 44.0], // [0][2][1][...]
                [54.0, 53.0, 36.0], // [0][2][2][...]
            ],
        ],
        // [1]
        [
            // [1][0]
            [
                [55.0, 64.0, 66.0], // [1][0][0][...]
                [73.0, 56.0, 65.0], // [1][0][1][...]
                [75.0, 74.0, 57.0], // [1][0][2][...]
            ],
            // [1][1]
            [
                [4.0, 13.0, 15.0], // [1][1][0][...]
                [22.0, 5.0, 14.0], // [1][1][1][...]
                [24.0, 23.0, 6.0], // [1][1][2][...]
            ],
            // [1][2]
            [
                [31.0, 40.0, 42.0], // [1][2][0][...]
                [49.0, 32.0, 41.0], // [1][2][1][...]
                [51.0, 50.0, 33.0], // [1][2][2][...]
            ],
        ],
        // [2]
        [
            // [2][0]
            [
                [61.0, 70.0, 72.0], // [2][0][0][...]
                [79.0, 62.0, 71.0], // [2][0][1][...]
                [81.0, 80.0, 63.0], // [2][0][2][...]
            ],
            // [2][1]
            [
                [58.0, 67.0, 69.0], // [2][1][0][...]
                [76.0, 59.0, 68.0], // [2][1][1][...]
                [78.0, 77.0, 60.0], // [2][1][2][...]
            ],
            // [2][2]
            [
                [7.0, 16.0, 18.0], // [2][2][0][...]
                [25.0, 8.0, 17.0], // [2][2][1][...]
                [27.0, 26.0, 9.0], // [2][2][2][...]
            ],
        ],
    ];

    #[rustfmt::skip]
    pub const SAMPLE1_STD_MATRIX: [[f64; 9]; 9] = [
        [ 1.0,  2.0,  3.0,  10.0, 11.0, 12.0,  19.0, 20.0, 21.0], // [0][0]...
        [ 4.0,  5.0,  6.0,  13.0, 14.0, 15.0,  22.0, 23.0, 24.0], // [1][1]...
        [ 7.0,  8.0,  9.0,  16.0, 17.0, 18.0,  25.0, 26.0, 27.0], // [2][2]...
        [28.0, 29.0, 30.0,  37.0, 38.0, 39.0,  46.0, 47.0, 48.0], // [0][1]...
        [31.0, 32.0, 33.0,  40.0, 41.0, 42.0,  49.0, 50.0, 51.0], // [1][2]...
        [34.0, 35.0, 36.0,  43.0, 44.0, 45.0,  52.0, 53.0, 54.0], // [0][2]...
        [55.0, 56.0, 57.0,  64.0, 65.0, 66.0,  73.0, 74.0, 75.0], // [1][0]...
        [58.0, 59.0, 60.0,  67.0, 68.0, 69.0,  76.0, 77.0, 78.0], // [2][1]...
        [61.0, 62.0, 63.0,  70.0, 71.0, 72.0,  79.0, 80.0, 81.0], // [2][0]...
    ];

    #[rustfmt::skip]
    pub const SAMPLE1_MANDEL_MATRIX:[[f64; 9]; 9] = [
        [         1.0 ,          2.0 ,          3.0 , 29.0/SQRT_2 , 31.0/SQRT_2 , 33.0/SQRT_2 , -9.0/SQRT_2 , -9.0/SQRT_2 , -9.0/SQRT_2 ],
        [         4.0 ,          5.0 ,          6.0 , 35.0/SQRT_2 , 37.0/SQRT_2 , 39.0/SQRT_2 , -9.0/SQRT_2 , -9.0/SQRT_2 , -9.0/SQRT_2 ],
        [         7.0 ,          8.0 ,          9.0 , 41.0/SQRT_2 , 43.0/SQRT_2 , 45.0/SQRT_2 , -9.0/SQRT_2 , -9.0/SQRT_2 , -9.0/SQRT_2 ],
        [ 83.0/SQRT_2 ,  85.0/SQRT_2 ,  87.0/SQRT_2 ,       110.0 ,       112.0 ,       114.0 ,        -9.0 ,        -9.0 ,        -9.0 ],
        [ 89.0/SQRT_2 ,  91.0/SQRT_2 ,  93.0/SQRT_2 ,       116.0 ,       118.0 ,       120.0 ,        -9.0 ,        -9.0 ,        -9.0 ],
        [ 95.0/SQRT_2 ,  97.0/SQRT_2 ,  99.0/SQRT_2 ,       122.0 ,       124.0 ,       126.0 ,        -9.0 ,        -9.0 ,        -9.0 ],
        [-27.0/SQRT_2 , -27.0/SQRT_2 , -27.0/SQRT_2 ,       -27.0 ,       -27.0 ,       -27.0 ,         0.0 ,         0.0 ,         0.0 ],
        [-27.0/SQRT_2 , -27.0/SQRT_2 , -27.0/SQRT_2 ,       -27.0 ,       -27.0 ,       -27.0 ,         0.0 ,         0.0 ,         0.0 ],
        [-27.0/SQRT_2 , -27.0/SQRT_2 , -27.0/SQRT_2 ,       -27.0 ,       -27.0 ,       -27.0 ,         0.0 ,         0.0 ,         0.0 ],
    ];

    pub const SAMPLE2: [[[[f64; 3]; 3]; 3]; 3] = [
        // [0]
        [
            // [0][0]
            [
                [1111_f64, 1112_f64, 1113_f64], // [0][0][0][...]
                [1121_f64, 1122_f64, 1123_f64], // [0][0][1][...]
                [1131_f64, 1132_f64, 1133_f64], // [0][0][2][...]
            ],
            // [0][1]
            [
                [1211_f64, 1212_f64, 1213_f64], // [0][1][0][...]
                [1221_f64, 1222_f64, 1223_f64], // [0][1][1][...]
                [1231_f64, 1232_f64, 1233_f64], // [0][1][2][...]
            ],
            // [0][2]
            [
                [1311_f64, 1312_f64, 1313_f64], // [0][2][0][...]
                [1321_f64, 1322_f64, 1323_f64], // [0][2][1][...]
                [1331_f64, 1332_f64, 1333_f64], // [0][2][2][...]
            ],
        ],
        // [1]
        [
            // [1][0]
            [
                [2111_f64, 2112_f64, 2113_f64], // [1][0][0][...]
                [2121_f64, 2122_f64, 2123_f64], // [1][0][1][...]
                [2131_f64, 2132_f64, 2133_f64], // [1][0][2][...]
            ],
            // [1][1]
            [
                [2211_f64, 2212_f64, 2213_f64], // [1][1][0][...]
                [2221_f64, 2222_f64, 2223_f64], // [1][1][1][...]
                [2231_f64, 2232_f64, 2233_f64], // [1][1][2][...]
            ],
            // [1][2]
            [
                [2311_f64, 2312_f64, 2313_f64], // [1][2][0][...]
                [2321_f64, 2322_f64, 2323_f64], // [1][2][1][...]
                [2331_f64, 2332_f64, 2333_f64], // [1][2][2][...]
            ],
        ],
        // [2]
        [
            // [2][0]
            [
                [3111_f64, 3112_f64, 3113_f64], // [2][0][0][...]
                [3121_f64, 3122_f64, 3123_f64], // [2][0][1][...]
                [3131_f64, 3132_f64, 3133_f64], // [2][0][2][...]
            ],
            // [2][1]
            [
                [3211_f64, 3212_f64, 3213_f64], // [2][1][0][...]
                [3221_f64, 3222_f64, 3223_f64], // [2][1][1][...]
                [3231_f64, 3232_f64, 3233_f64], // [2][1][2][...]
            ],
            // [2][2]
            [
                [3311_f64, 3312_f64, 3313_f64], // [2][2][0][...]
                [3321_f64, 3322_f64, 3323_f64], // [2][2][1][...]
                [3331_f64, 3332_f64, 3333_f64], // [2][2][2][...]
            ],
        ],
    ];

    pub const SYM_SAMPLE1: [[[[f64; 3]; 3]; 3]; 3] = [
        // [0]
        [
            // [0][0]
            [
                [1.0, 10.0, 12.0], // [0][0][0][...]
                [10.0, 2.0, 11.0], // [0][0][1][...]
                [12.0, 11.0, 3.0], // [0][0][2][...]
            ],
            // [0][1]
            [
                [19.0, 28.0, 30.0], // [0][1][0][...]
                [28.0, 20.0, 29.0], // [0][1][1][...]
                [30.0, 29.0, 21.0], // [0][1][2][...]
            ],
            // [0][2]
            [
                [25.0, 34.0, 36.0], // [0][2][0][...]
                [34.0, 26.0, 35.0], // [0][2][1][...]
                [36.0, 35.0, 27.0], // [0][2][2][...]
            ],
        ],
        // [1]
        [
            // [1][0]
            [
                [19.0, 28.0, 30.0], // [1][0][0][...]
                [28.0, 20.0, 29.0], // [1][0][1][...]
                [30.0, 29.0, 21.0], // [1][0][2][...]
            ],
            // [1][1]
            [
                [4.0, 13.0, 15.0], // [1][1][0][...]
                [13.0, 5.0, 14.0], // [1][1][1][...]
                [15.0, 14.0, 6.0], // [1][1][2][...]
            ],
            // [1][2]
            [
                [22.0, 31.0, 33.0], // [1][2][0][...]
                [31.0, 23.0, 32.0], // [1][2][1][...]
                [33.0, 32.0, 24.0], // [1][2][2][...]
            ],
        ],
        // [2]
        [
            // [2][0]
            [
                [25.0, 34.0, 36.0], // [2][0][0][...]
                [34.0, 26.0, 35.0], // [2][0][1][...]
                [36.0, 35.0, 27.0], // [2][0][2][...]
            ],
            // [2][1]
            [
                [22.0, 31.0, 33.0], // [2][1][0][...]
                [31.0, 23.0, 32.0], // [2][1][1][...]
                [33.0, 32.0, 24.0], // [2][1][2][...]
            ],
            // [2][2]
            [
                [7.0, 16.0, 18.0], // [2][2][0][...]
                [16.0, 8.0, 17.0], // [2][2][1][...]
                [18.0, 17.0, 9.0], // [2][2][2][...]
            ],
        ],
    ];

    #[rustfmt::skip]
    pub const SYM_SAMPLE1_STD_MATRIX: [[f64; 9]; 9] = [
        [ 1.0,  2.0,  3.0,  10.0, 11.0, 12.0,  10.0, 11.0, 12.0], // [0][0]...
        [ 4.0,  5.0,  6.0,  13.0, 14.0, 15.0,  13.0, 14.0, 15.0], // [1][1]...
        [ 7.0,  8.0,  9.0,  16.0, 17.0, 18.0,  16.0, 17.0, 18.0], // [2][2]...
        [19.0, 20.0, 21.0,  28.0, 29.0, 30.0,  28.0, 29.0, 30.0], // [0][1]...
        [22.0, 23.0, 24.0,  31.0, 32.0, 33.0,  31.0, 32.0, 33.0], // [1][2]...
        [25.0, 26.0, 27.0,  34.0, 35.0, 36.0,  34.0, 35.0, 36.0], // [0][2]...
        [19.0, 20.0, 21.0,  28.0, 29.0, 30.0,  28.0, 29.0, 30.0], // [1][0]...
        [22.0, 23.0, 24.0,  31.0, 32.0, 33.0,  31.0, 32.0, 33.0], // [2][1]...
        [25.0, 26.0, 27.0,  34.0, 35.0, 36.0,  34.0, 35.0, 36.0], // [2][0]...
    ];

    #[rustfmt::skip]
    pub const SYM_SAMPLE1_MANDEL_MATRIX:[[f64; 6]; 6] = [
        [ 1.0       ,  2.0       ,  3.0       , 10.0*SQRT_2, 11.0*SQRT_2, 12.0*SQRT_2],
        [ 4.0       ,  5.0       ,  6.0       , 13.0*SQRT_2, 14.0*SQRT_2, 15.0*SQRT_2],
        [ 7.0       ,  8.0       ,  9.0       , 16.0*SQRT_2, 17.0*SQRT_2, 18.0*SQRT_2],
        [19.0*SQRT_2, 20.0*SQRT_2, 21.0*SQRT_2, 56.0       , 58.0       , 60.0       ],
        [22.0*SQRT_2, 23.0*SQRT_2, 24.0*SQRT_2, 62.0       , 64.0       , 66.0       ],
        [25.0*SQRT_2, 26.0*SQRT_2, 27.0*SQRT_2, 68.0       , 70.0       , 72.0       ],
    ];

    pub const SYM_2D_SAMPLE1: [[[[f64; 3]; 3]; 3]; 3] = [
        // [0]
        [
            // [0][0]
            [
                [1.0, 10.0, 0.0], // [0][0][0][...]
                [10.0, 2.0, 0.0], // [0][0][1][...]
                [0.0, 0.0, 3.0],  // [0][0][2][...]
            ],
            // [0][1]
            [
                [19.0, 28.0, 0.0], // [0][1][0][...]
                [28.0, 20.0, 0.0], // [0][1][1][...]
                [0.0, 0.0, 21.0],  // [0][1][2][...]
            ],
            // [0][2]
            [
                [0.0, 0.0, 0.0], // [0][2][0][...]
                [0.0, 0.0, 0.0], // [0][2][1][...]
                [0.0, 0.0, 0.0], // [0][2][2][...]
            ],
        ],
        // [1]
        [
            // [1][0]
            [
                [19.0, 28.0, 0.0], // [1][0][0][...]
                [28.0, 20.0, 0.0], // [1][0][1][...]
                [0.0, 0.0, 21.0],  // [1][0][2][...]
            ],
            // [1][1]
            [
                [4.0, 13.0, 0.0], // [1][1][0][...]
                [13.0, 5.0, 0.0], // [1][1][1][...]
                [0.0, 0.0, 6.0],  // [1][1][2][...]
            ],
            // [1][2]
            [
                [0.0, 0.0, 0.0], // [1][2][0][...]
                [0.0, 0.0, 0.0], // [1][2][1][...]
                [0.0, 0.0, 0.0], // [1][2][2][...]
            ],
        ],
        // [2]
        [
            // [2][0]
            [
                [0.0, 0.0, 0.0], // [2][0][0][...]
                [0.0, 0.0, 0.0], // [2][0][1][...]
                [0.0, 0.0, 0.0], // [2][0][2][...]
            ],
            // [2][1]
            [
                [0.0, 0.0, 0.0], // [2][1][0][...]
                [0.0, 0.0, 0.0], // [2][1][1][...]
                [0.0, 0.0, 0.0], // [2][1][2][...]
            ],
            // [2][2]
            [
                [7.0, 16.0, 0.0], // [2][2][0][...]
                [16.0, 8.0, 0.0], // [2][2][1][...]
                [0.0, 0.0, 9.0],  // [2][2][2][...]
            ],
        ],
    ];

    #[rustfmt::skip]
    pub const SYM_2D_SAMPLE1_STD_MATRIX: [[f64; 9]; 9] = [
        [ 1.0,  2.0,  3.0,  10.0, 0.0, 0.0,  10.0, 0.0, 0.0], // [0][0]...
        [ 4.0,  5.0,  6.0,  13.0, 0.0, 0.0,  13.0, 0.0, 0.0], // [1][1]...
        [ 7.0,  8.0,  9.0,  16.0, 0.0, 0.0,  16.0, 0.0, 0.0], // [2][2]...
        [19.0, 20.0, 21.0,  28.0, 0.0, 0.0,  28.0, 0.0, 0.0], // [0][1]...
        [ 0.0,  0.0,  0.0,   0.0, 0.0, 0.0,   0.0, 0.0, 0.0], // [1][2]...
        [ 0.0,  0.0,  0.0,   0.0, 0.0, 0.0,   0.0, 0.0, 0.0], // [0][2]...
        [19.0, 20.0, 21.0,  28.0, 0.0, 0.0,  28.0, 0.0, 0.0], // [1][0]...
        [ 0.0,  0.0,  0.0,   0.0, 0.0, 0.0,   0.0, 0.0, 0.0], // [2][1]...
        [ 0.0,  0.0,  0.0,   0.0, 0.0, 0.0,   0.0, 0.0, 0.0], // [2][0]...
    ];

    #[rustfmt::skip]
    pub const SYM_2D_SAMPLE1_MANDEL_MATRIX:[[f64; 4]; 4] = [
        [ 1.0       ,  2.0       ,  3.0       , 10.0*SQRT_2],
        [ 4.0       ,  5.0       ,  6.0       , 13.0*SQRT_2],
        [ 7.0       ,  8.0       ,  9.0       , 16.0*SQRT_2],
        [19.0*SQRT_2, 20.0*SQRT_2, 21.0*SQRT_2, 56.0       ],
    ];
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::SamplesTensor4;
    use crate::constants::IJKL_TO_MN;

    #[test]
    fn sample1_is_ok() {
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    for l in 0..3 {
                        let (m, n) = IJKL_TO_MN[i][j][k][l];
                        let val = SamplesTensor4::SAMPLE1_STD_MATRIX[m][n];
                        assert_eq!(SamplesTensor4::SAMPLE1[i][j][k][l], val);
                    }
                }
            }
        }
    }

    #[test]
    fn sample2_is_ok() {
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    for l in 0..3 {
                        let val = (i + 1) * 1000 + (j + 1) * 100 + (k + 1) * 10 + (l + 1);
                        assert_eq!(SamplesTensor4::SAMPLE2[i][j][k][l], val as f64);
                    }
                }
            }
        }
    }

    #[test]
    fn sample1_sym_is_ok() {
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    for l in 0..3 {
                        let (a, b) = IJKL_TO_MN[i][j][k][l];
                        let val = SamplesTensor4::SYM_SAMPLE1_STD_MATRIX[a][b];
                        assert_eq!(SamplesTensor4::SYM_SAMPLE1[i][j][k][l], val);
                    }
                }
            }
        }
    }

    #[test]
    fn sample1_sym_2d_is_ok() {
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    for l in 0..3 {
                        let (m, n) = IJKL_TO_MN[i][j][k][l];
                        let val = SamplesTensor4::SYM_2D_SAMPLE1_STD_MATRIX[m][n];
                        assert_eq!(SamplesTensor4::SYM_2D_SAMPLE1[i][j][k][l], val);
                    }
                }
            }
        }
    }
}
