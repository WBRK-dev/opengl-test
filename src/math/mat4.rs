use glm::{Mat4, Vector4};

pub trait ToColsArray {
    fn to_cols_array(&self) -> [f32; 16];

    fn as_cols_array(&self) -> &[f32; 16];
}

impl ToColsArray for Mat4 {
    fn to_cols_array(&self) -> [f32; 16] {
        let c0 = self.c0.as_array();
        let c1 = self.c1.as_array();
        let c2 = self.c2.as_array();
        let c3 = self.c3.as_array();

        [
            c0[0], c0[1], c0[2], c0[3],
            c1[0], c1[1], c1[2], c1[3],
            c2[0], c2[1], c2[2], c2[3],
            c3[0], c3[1], c3[2], c3[3],
        ]
    }

    fn as_cols_array(&self) -> &[f32; 16] {
        unsafe { &*(self as *const Mat4 as *const [f32; 16]) }
    }
}

pub trait Identity {
    fn one() -> Self;
}

impl Identity for Mat4 {
    fn one() -> Self {
        Self::new(
            Vector4::new(1.0, 0.0, 0.0, 0.0),
            Vector4::new(0.0, 1.0, 0.0, 0.0),
            Vector4::new(0.0, 0.0, 1.0, 0.0),
            Vector4::new(0.0, 0.0, 0.0, 1.0),
        )
    }
}
