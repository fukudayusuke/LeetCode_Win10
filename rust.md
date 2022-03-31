## :bulb: Rust Index
 
 ---
#### :bulb: 演算子
##### - ...obj  
   
| 式名称               | 書き方                  | 例                         |
|----------------------|-------------------------|----------------------------|
| 配列リテラル         | [,   , …]              | [3,   1, 4]                |
| 繰り返し配列リテラル | [;]                     | [0; 20]                    |
| タプル               | (, , …)                | (82, “Wanigame”)         |
| グループ化           | ()                      | (17   + 90)                |
| ブロック             | {}                      | { f(); g() }               |
| 　                   | 　                      | 　                         |
| 制御フロー           | if condition {} else {} | if x == 0 {3} else {4}     |
|                      | math x {}               | match x { 1=>2, _=>5 }     |
|                      | for e in vec {}         | for a in x { f(a) }        |
|                      | while {}                | while { f(); }             |
|                      | loop {}                 | loop { f(); }              |
|                      | break                   | 　                         |
|                      | continue                | 　                         |
|                      | return                  | return 9                   |
|                      | 　                      | 　                         |
| マクロ               | !()                     | println!(“{}”, 0)        |
| パス                 | ::                      | std::f64::const::PI        |
| 構造体リテラル       | {:,   :, …}             | Point{   x: 1080, y:1920 } |
| タプルアクセス       | .                       | taple.0                    |
| 構造体アクセス       | .                       | struct.x                   |
| メソッド呼び出し     | .()                     | struct.f()                 |
| 関数呼び出し         | ()                      | f()                        |
| 配列アクセス         | []                      | array[0]                   |
| エラーチェック       | ?                       | create_dir(path)?          |
| ビット/論理NOT       | !                       | !a                         |
| 符号反転             | –                       | -x                         |
| 参照解決             | *                       | *p                         |
| 借用                 | &                       | &a                         |
| 型キャスト           | as                      | a   as u8                  |
| 掛け算               | *                       | 8   * 9                    |
| 割り算               | /                       | 12 / 8                     |
| 剰余算               | %                       | 90 % 7                     |
| 足し算               | +                       | 4   + 8                    |
| 引き算               | –                       | 9 – 6                      |
| 左シフト             | <<                      | 0b100   >> 2               |
| 右シフト             | >>                      | 0b001 << 2                 |
| ビットAND            | &                       | 0b100   & 0b010            |
| ビットXOR            | ^                       | 0b100   ^ 0b111            |
| ビットOR             | \|                      | 0b100   \| 0b001           |
| 比較（小なり）       | <,   <=                 | 0   < 9                    |
| 比較（大なり）       | >, >=                   | 8 >= 9                     |
| 比較（等価）         | ==, !=                  | 4 != 5                     |
| 論理AND              | &&                      | true   && false            |
| 論理OR               | \|\|                    | false   \|\| false         |
| 範囲                 | ..                      | 9   .. 10                  |
| 代入                 | =                       | x   = y                    |
| 　                   | 　                      | 　                         |
| 複合代入             | *=                      | x *= y                     |
| 　                   | /=                      | x /= y                     |
| 　                   | %=                      | x %= y                     |
| 　                   | +=                      | x += y                     |
| 　                   | -=                      | x -= y                     |
| 　                   | <<=                     | x <<= y                    |
| 　                   | >>=                     | x >>=y                     |
| 　                   | &=                      | x &= y                     |
| 　                   | ^=                      | x ^= y                     |
| 　                   | \|=                     | x \|= y                    |
| クロージャ           | \|\|                    | \|x,   y\| x + y           |
 
 ---
#### :bulb: クロージャ
即時関数、無名関数のこと
マクロみたいに登録しておくと便利？
---

#### :bulb: Method

---
##### -   Struct std::vec::Vec
動的配列宣言
静的は今まで通り
```rust
let mut vec = vec![1, 2, 3];
vec.push(4);
assert_eq!(vec, [1, 2, 3, 4]);
```

---

##### -   checked_mul
Checked integer multiplication. Computes self * rhs, returning None if overflow occurred.
```rust
pub const fn checked_mul(self, rhs: u32) -> Option<u32>

assert_eq!(5u32.checked_mul(1), Some(5));
assert_eq!(u32::MAX.checked_mul(2), None);
```

---

##### -   checked_add
```rust
```
---

##### -   unwrap_or_default
Returns the contained Some value or a default

Consumes the self argument then, if Some, returns the contained value, otherwise if None, returns the default value for that type.

Converts a string to an integer, turning poorly-formed strings into 0 (the default value for integers). parse converts a string to any other type that implements FromStr, returning None on error.
```rust
let good_year_from_input = "1909";
let bad_year_from_input = "190blarg";
let good_year = good_year_from_input.parse().ok().unwrap_or_default();
let bad_year = bad_year_from_input.parse().ok().unwrap_or_default();

assert_eq!(1909, good_year);
assert_eq!(0, bad_year);
```
---

##### -   abs
絶対値取得
```rust
num = n.abs();
}
```
---

#### - Module std::option
Type Option represents an optional value: every Option is either Some and contains a value, or None, and does not. Option types are very common in Rust code, as they have a number of uses:

```rust
```
---

##### -   chars
文字列変換
```rust
str = n.chars();
}
```

#### - unwrap_or_else
Returns the contained Ok value or a provided default.

Arguments passed to unwrap_or are eagerly evaluated; if you are passing the result of a function call, it is recommended to use unwrap_or_else, which is lazily evaluated.
---

#### - binary_search
Binary searches this sorted slice for a given element.

If the value is found then Result::Ok is returned, containing the index of the matching element. If there are multiple matches, then any one of the matches could be returned. If the value is not found then Result::Err is returned, containing the index where a matching element could be inserted while maintaining sorted order.

### - td::convert::TryInto  


### - x.sort()  
昇順に自動ソートしてくれる　　
