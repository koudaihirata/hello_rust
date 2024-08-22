fn main() {
    // 数値型
    let a = 1;
    let b = 2.0;

    let c: u16 = 3;

    let d: f32 = 4.0f32;

    let e: i32 = 1 + 2; // 足し算
    let e: i32 = 1 - 2; // 引き算
    let e: i32 = 1 * 2; // 掛け算
    let e: i32 = 1 / 2; // 割り算
    let e: i32 = 1 % 2; // 余り

    let f: f64 = 1 as f64 + 2.0;

    // 論理型
    // true, false
    let g: bool = false;

    let h: bool = 1 == 2; // 等価比較
    let h: bool = 1 != 2; // 非等価比較
    let h: bool = 1 > 2; // 大なり
    let h: bool = 1 < 2; // 小なり
    let h: bool = 1 >= 2; // 大なりイコール
    let h: bool = 1 <= 2; // 小なりイコール
}
