pub struct Quaternion {
    w: f32, //実部
    x: f32, //虚部i
    y: f32, //虚部j
    z: f32, //虚部k
}

impl Quaternion {
    // コンストラクタ
    pub fn new(w: f32, x: f32, y: f32, z: f32) -> Self {
        Self {
            w,
            x,
            y,
            z,
        }
    }
    // クオータニオン積の定義
    pub fn mul(&self, rhs: &Self) -> Self {
        let lhs: &Quaternion = self;
        let result: Quaternion = Quaternion {
            w: lhs.w * rhs.w - lhs.x * rhs.x - lhs.y * rhs.y - lhs.z * rhs.z,
            x: lhs.w * rhs.x + lhs.x * rhs.w + lhs.y * rhs.z - lhs.z * rhs.y,
            y: lhs.w * rhs.y - lhs.x * rhs.z + lhs.y * rhs.w + lhs.z * rhs.x,
            z: lhs.w * rhs.z + lhs.x * rhs.y - lhs.y * rhs.x + lhs.z * rhs.w,
        };
        return result;
    }
    //クオータニオンの加算の定義
    pub fn sum(&self, rhs: &Self) -> Self {
        let lhs: &Quaternion = self;
        let result: Quaternion = Quaternion {
            w: lhs.w + rhs.w,
            x: lhs.x + rhs.x,
            y: lhs.y + rhs.y,
            z: lhs.z + rhs.z,
        };
        return result;
    }
    //クオータニオンのスカラー倍の定義
    pub fn scale(&mut self, scalar: f32) {
        self.w *= scalar;
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
    //クオータニオンの共役の定義
    pub fn conjugate(&self) -> Self {
        let result: Quaternion = Quaternion {
            w: self.w,
            x: - self.x,
            y: - self.y,
            z: - self.z,
        };
        return result;
    }
    //クオータニオンのノルムの計算
    fn norm(&self) -> f32 {
        let squared_norm = self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z;
        let norm: f32 = squared_norm.sqrt();
        return norm;
    }
    //クオータニオンの正規化
    pub fn normalize(&mut self) {
        let norm: f32 = self.norm();
        self.w /= norm;
        self.x /= norm;
        self.y /= norm;
        self.z /= norm;
    }
    //クオータニオンの表示
    pub fn print(&self) {
        println!("({}, {}, {}, {})", self.w, self.x, self.y, self.z)
    }
}