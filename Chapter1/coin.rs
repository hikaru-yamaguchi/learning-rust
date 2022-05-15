// Rustでコインのおつり枚数を計算
fn main() {
  // 期待する金額
  let price = 3950;
  // 500円玉の枚数を繰り返して調べる
  for i500 in 0..11 {
    // 100円玉の枚数を繰り返して調べる
    for i100 in 0..11 {
      // 50円玉の枚数を繰り返して調べる
      for i50 in 0..11 {
        // 総数を計算
        let total = i50 * 50 + i100 * 100 + i500 * 500;
        // 総額が期待する金額になるか
        if price == total {
          println!("500円x{}+100円x{}+50円x{}={}",
            i500, i100, i50, total);
        }
      }
    }
  }
}