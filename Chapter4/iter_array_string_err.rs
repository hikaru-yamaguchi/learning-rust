// 問題があるプログラム
// 所有権システムに注意せよ
fn main() {
    let array = [
        "Apple".to_string(),
        "Banana".to_string(),
        "Mango".to_string(),
        "Tomato".to_string(),
    ];
    // 配列を直接指定すると暗黙でinto_iter()が使われる
    // つまり所有権が移動している
    for a in array {
        println!("{}", a);
    }
    // コンパイルエラー
    // arrayが移動している
    println!("len={}", array.len());
}