struct Matrix64 {
    len: usize,
    mat: Box<[[i32; 64]; 64]>,
}

impl Matrix64 {
    fn new_with_array(mat: Box<[[i32; 64]; 64]>, len: usize) -> Matrix64 {
        Matrix64 {
            len,
            mat
        }
    }

    // Bonus implement a new_with_value or new_random or a macro to create
    // a matrix.
    fn new() -> Matrix64 {
        Self::new_with_array(
            Box::new([[0i32; 64]; 64]),
            64,
        )
    }

    fn len(&self) -> usize {
        self.len
    }

    /// Multiply two matrix
    ///
    /// This function panics if the length of squared matrix `self` is not
    /// equals to colomn number of matrix `other`.
    /// Bonus: implementing Mul trait https://doc.rust-lang.org/std/ops/trait.Mul.html
    ///
    /// Further reading:
    /// https://fr.wikipedia.org/wiki/Produit_matriciel
    fn mul(&self, other: &Matrix64) -> Matrix64 {
        let len = self.len();

        let mut res = Box::new([[0i32; 64]; 64]);

        let a = self.mat;
        let b = self.mat;

        let squared_len = len * len;

        for i in 0..squared_len {
            for j in 0..squared_len {
                for k in 0..squared_len {
                    res[i][k] = a[j][i] * b[k][j];
                }
            }
        }
        Self::new_with_array(res, 64)
    }
}

fn process() {
    
}