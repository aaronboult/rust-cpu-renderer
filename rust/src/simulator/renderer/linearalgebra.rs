use std::fmt;
use std::cmp::{PartialEq};
use std::ops::{Add, Index, IndexMut, Mul, Sub, Neg};

pub trait MatrixBase {
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;
    fn combine(&self, other: &Matrix, sub: bool) -> Result<Matrix, ()>;
    fn dot(&self, other: &Matrix) -> Result<Matrix, ()>;
    fn scale(&self, other: f32) -> Self;
}

//#region Matrix
#[derive(Clone, PartialEq)]
pub struct Matrix {
    width: usize,
    height: usize,
    contents: Vec<f32>,
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        for row in 0..self.height {
            output.push_str("[");
            for col in 0..self.width {
                output.push_str(&format!("{}", self[row][col]));
                if col != self.width - 1 {
                    output.push_str(", ");
                }
            }
            output.push_str("]\n");
        }
        write!(f, "{}", output)
    }
}

impl Matrix {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            contents: vec![0.0; width * height],
        }
    }

    pub fn from_vec(width: usize, height: usize, mut contents: Vec<f32>) -> Self {
        if width * height < contents.len() {
            panic!("Cannot construct matrix where width and height are less than the supplied vector");
        }
        contents.resize(width * height, 0.0);
        Self {
            width,
            height,
            contents
        }
    }
}

impl MatrixBase for Matrix {
    fn get_width(&self) -> usize {
        self.width
    }

    fn get_height(&self) -> usize {
        self.height
    }

    fn combine(&self, other: &Matrix, sub: bool) -> Result<Self, ()> {
        if self.width == other.width && self.height == other.height {
            let mut result = Matrix::new(self.width, self.height);
            for row in 0..self.height {
                for col in 0..self.width {
                    result[row][col] = self[row][col]
                        + if sub {
                            -other[row][col]
                        } else {
                            other[row][col]
                        };
                }
            }
            return Ok(result);
        }
        Err(())
    }

    fn dot(&self, other: &Matrix) -> Result<Self, ()> {
        let left: &Matrix;
        let right: &Matrix;
        // check for presence of column vectors
        if self.width == 1 && other.width > 1 {
            left = other;
            right = self;
        }
        else{
            left = self;
            right = other;
        }

        let mut result: Matrix;

        // if two matrices are being multiplied
        if left.width == right.height {
            result = Matrix::new(right.width, left.height);
            for row in 0..left.height {
                for r_col in 0..right.width {
                    let mut colum_sum = 0.0;
                    for col in 0..left.width {
                        colum_sum += left[row][col] * right[col][r_col];
                    }
                    result[row][r_col] = colum_sum;
                }
            }
        }
        // if a vector is being multiplied by either a matrix or a vector
        else if left.width == 1 && left.height == right.height {
            result = Matrix::new(right.width, right.height);
            for row in 0..left.height {
                for r_col in 0..right.width {
                    result[row][r_col] = left[row][0] * right[row][r_col];
                }
            }
        } else {
            return Err(());
        }

        Ok(result)
    }

    fn scale(&self, scaler: f32) -> Self {
        let mut result = Matrix::new(self.width, self.height);
        for row in 0..self.height {
            for col in 0..self.width {
                result[row][col] = self[row][col] * scaler;
            }
        }
        result
    }
}

impl Add<Self> for Matrix {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.combine(&rhs, false).expect(
            &format!(
                "Cannot add matrix (l) of Height: {}, Width: {} with matrix (r) of dimensions Height: {}, Width: {}",
                self.height, self.width,
                rhs.height, rhs.width
            ).to_string()
        )
    }
}

impl Sub<Self> for Matrix {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.combine(&rhs, true).expect(
            &format!(
                "Cannot subtract matrix (l) of Height: {}, Width: {} with matrix (r) of dimensions Height: {}, Width: {}",
                self.height, self.width,
                rhs.height, rhs.width
            ).to_string()
        )
    }
}

impl Mul<Self> for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self.dot(&rhs).expect(
            &format!(
                "Cannot multiply matrix (l) of Height: {}, Width: {} with matrix (r) of dimensions Height: {}, Width: {}",
                self.height, self.width,
                rhs.height, rhs.width
            ).to_string()
        )
    }
}

impl Mul<Vector3D> for Matrix {
    type Output = Self;

    fn mul(self, rhs: Vector3D) -> Self::Output {
        self.dot(&Matrix::from(rhs)).expect(
            &format!(
                "Cannot multiply matrix of height {} with vector of height 3",
                self.height
            ).to_string()
        )
    }
}

impl Mul<Vector2D> for Matrix {
    type Output = Self;

    fn mul(self, rhs: Vector2D) -> Self::Output {
        self.dot(&Matrix::from(rhs)).expect(
            &format!(
                "Cannot multiply matrix of height {} with vector of height 2",
                self.height
            ).to_string()
        )
    }
}

impl Mul<f32> for Matrix {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        self.scale(rhs)
    }
}

impl Mul<Matrix> for f32 {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        rhs * self
    }
}

impl Index<usize> for Matrix {
    type Output = [f32];

    fn index(&self, i: usize) -> &Self::Output {
        &self.contents[self.width * i..(self.width * i + self.width)]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.contents[self.width * i..(self.width * i + self.width)]
    }
}

impl From<Vector3D> for Matrix {
    fn from(vector: Vector3D) -> Self {
        Matrix::from_vec(1, 3, vec![vector.x, vector.y, vector.z])
    }
}

impl From<Vector2D> for Matrix {
    fn from(vector: Vector2D) -> Self {
        Matrix::from_vec(1, 2, vec![vector.x, vector.y])
    }
}
//#endregion

//#region Vector3D
// vector3d is a special case of a matrix
// where the width is 1 and the height is 3
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Vector3D {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vector3D {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0, z: 0.0 };
    pub const ONE: Self = Self { x: 1.0, y: 1.0, z: 1.0 };

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

impl fmt::Display for Vector3D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(X: {}, Y: {}, Z: {})", self.x, self.y, self.z)
    }
}

macro_rules! vector3d_combine {
    ($self:ident, $rhs:ident, $sub:literal) => (
        {
            let result = Matrix::from($self).combine(&Matrix::from($rhs), $sub).unwrap();
            Self {
                x: result[0][0],
                y: result[1][0],
                z: result[2][0]
            }
        }
    )
}

impl Add<Self> for Vector3D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        vector3d_combine!(self, rhs, false)
    }
}

impl Sub<Self> for Vector3D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        vector3d_combine!(self, rhs, true)
    }
}

impl Mul<Self> for Vector3D {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let result = Matrix::from(self).dot(&Matrix::from(rhs)).unwrap();
        Self {
            x: result[0][0],
            y: result[1][0],
            z: result[2][0]
        }
    }
}

impl Mul<Matrix> for Vector3D {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        rhs * self
    }
}

impl Mul<f32> for Vector3D {
    type Output = Vector3D;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs
        }
    }
}

impl Mul<Vector3D> for f32 {
    type Output = Vector3D;

    fn mul(self, rhs: Vector3D) -> Self::Output {
        rhs * self
    }
}

impl Neg for Vector3D {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl From<Matrix> for Vector3D {
    fn from(other: Matrix) -> Self {
        if other.width != 1 || other.height != 3 {
            panic!(
                "Cannot cast Matrix to Vector3D with Height {} and Width {}",
                other.height,
                other.width
            );
        }
        Self {
            x: other[0][0],
            y: other[1][0],
            z: other[2][0]
        }
    }
}

impl From<Vector2D> for Vector3D {
    fn from(other: Vector2D) -> Self {
        Vector3D::new(other.x, other.y, 0.0)
    }
}
//#endregion

//#region Vector2D
// vector2d is a special case of a matrix
// where the width is 1 and the height is 2
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Vector2D {
    pub x: f32,
    pub y: f32
}

impl Vector2D {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
    pub const ONE: Self = Self { x: 1.0, y: 1.0 };

    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl fmt::Display for Vector2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(X: {}, Y: {})", self.x, self.y)
    }
}

macro_rules! vector2d_combine {
    ($self:ident, $rhs:ident, $sub:literal) => (
        {
            let result = Matrix::from($self).combine(&Matrix::from($rhs), $sub).unwrap();
            Self {
                x: result[0][0],
                y: result[1][0]
            }
        }
    )
}

impl Add<Self> for Vector2D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        vector2d_combine!(self, rhs, false)
    }
}

impl Sub<Self> for Vector2D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        vector2d_combine!(self, rhs, true)
    }
}

impl Mul<Self> for Vector2D {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let result = Matrix::from(self).dot(&Matrix::from(rhs)).unwrap();
        Self {
            x: result[0][0],
            y: result[1][0]
        }
    }
}

impl Mul<Matrix> for Vector2D {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        rhs * self
    }
}

impl Mul<f32> for Vector2D {
    type Output = Vector2D;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl Mul<Vector2D> for f32 {
    type Output = Vector2D;

    fn mul(self, rhs: Vector2D) -> Self::Output {
        rhs * self
    }
}

impl Neg for Vector2D {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y
        }
    }
}

impl From<Matrix> for Vector2D {
    fn from(other: Matrix) -> Self {
        if other.width != 1 || other.height != 2 {
            panic!(
                "Cannot cast Matrix to Vector2D with Height {} and Width {}",
                other.height,
                other.width
            );
        }
        Self {
            x: other[0][0],
            y: other[1][0]
        }
    }
}

macro_rules! vector2d_into_t {
    ($tp:ty) => (
        impl From<Vector2D> for ($tp, $tp) {
            fn from(other: Vector2D) -> ($tp, $tp) {
                (other.x as $tp, other.y as $tp)
            }
        }
    )
}

vector2d_into_t!(u32);
vector2d_into_t!(i32);
vector2d_into_t!(f32);
//#endregion