# 3.1 변수와 가변성

rust 에서는 기본적으로 변수는 불변

~~~ rust 
//==>ERROR on running (cannot assign twice to immutable variable 'x')
fn main(){
    let x=5;
    println!(x);
    x=6;
    println!(x);
}  
~~~
변수명 앞에 mut 를 붙이면 가변변수
~~~ rust 
fn main(){
    let mut x=5;
    println!(x);
    x=6;
    println!(x);
}  

//==>output
5
6
~~~
## 3.1.1 상수
불변 변수와 비슷하나 let 대신 const 로 선언하며 mut 를 사용할수없다.
~~~ rust
//ex
const THREE_HOURS_IN_SECONDS: u32 =60*60*3;
~~~
## 3.1.2 섀도잉
새 변수의 변수명을 이전 변수명과 같게 선언할 수 있으며, scope 에서 나가면 섀도잉이 끝남
~~~ rust
fn main(){
    let x=5;
    let x=x+1;
    {
        let x= x*2;
        println!(x);
    }
    println!(x);
}
//==>output
12
6
~~~
mut 와 shadowing의 차이는 let 을 통해 효과적으로 새변수를 선언하고 값의 유형을 변경할수있다.
~~~ rust
//OK
let spaces = "   ";
let spaces = spaces.len();

//compile Error 정의후 변경하려는 형태가 다름 str->i32
let mut spaces ="   ";
spaces = spaces.len();
~~~

# 3.2 데이터 타입들
Rust 는 타입이 고정된 언어다 : 모든 변수의 타입은 컴파일시에 반드시 정해저 있어야한다. String 을 parse 통해 숫자로 변환했던경우처럼 타입의 선택폭이 넓은 경우 반드시 타입의 명시를 첨가해야한다.
~~~ rust
//ex
let guess: i32 = "42".parse().expect("not a number");
~~~
## 3.2.1 스칼라 타입들
### 정수형
rust에서의 정수 타입
| Lengtht | Signed | Unsigned |
| :-----: | :----: | :------: |
|  8-bit  |   i8   |    u8    |
| 16-bit  |  i16   |   u16    |
| 32-bit  |  i32   |   u32    |
| 64-bit  |  i64   |   u64    |
|  arch   | isize  |  usize   |

rust 에서의 정수형 리터럴들

| Number literals |   Example   |
| :-------------- | :---------- |
| Decimal         |   98_222    |
| Hex             |    0xff     |
| Octal           |    0o77     |
| Binary          | 0b1111_0000 |
| Byte(u8 only)   |    b'A'     |

### 부동 소수점 타입

f32 f64
### 수학적연산들
~~~ rust
    fn main() {
    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    // remainder
    let remainder = 43 % 5;
}
~~~
### Boolean 타입
C++ 과같이 bool 로 명시
### 문자타입
String =>"string" char=>'c' Rust 에서 char 타입은 Unicode Scalar 를 사용
### 복합타입들
#### tuple
tuple 은 다양한 타입의 숫자들을 집합시켜 복합타입으로 만드는 방법
~~~ rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
~~~
개별값을 튜플 밖으로 빼내오기위해서는 패턴매칭을 사용
~~~ rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
}

//output
The value of y is: 6.4
~~~

tup 에 . 을 붙여 튜플의 요소에 직접적으로 접근 가능
~~~ rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
}
~~~

#### array
배열은 튜플과 다르개 모든요소가 같은 타입이여야함 rust에서 배열은 길이가 고정 한번 선언되면 크기는 변하지 않는다.
~~~ rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
~~~
배열의요소는 [num] 을통해 접근한다.
~~~ rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    let index = 10;

    let element = a[index];

    println!("The value of element is: {}", element);
}
~~~
# 3.3 함수 동작 원리