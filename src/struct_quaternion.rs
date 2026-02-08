pub struct Quaternion {
    pub w: f32, //実部
    pub x: f32, //虚部i
    pub y: f32, //虚部j
    pub z: f32, //虚部k
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
    pub fn scale(&self, scalar: f32) -> Self {
        let result: Quaternion = Quaternion {
            w: self.w * scalar,
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        };
        return result;
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
    pub fn normalize(&self) -> Self {
        let norm: f32 = self.norm();
        let result= Quaternion {
            w: self.w / norm,
            x: self.x / norm,
            y: self.y / norm,
            z: self.z / norm,
        };
        return result;
    }   
    //クオータニオンの表示
    pub fn print(&self) {
        println!("({}, {}, {}, {})", self.w, self.x, self.y, self.z)
    }
}