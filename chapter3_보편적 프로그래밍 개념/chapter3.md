* # 3.1 변수와 가변성

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
* ## 3.1.1 상수
불변 변수와 비슷하나 let 대신 const 로 선언하며 mut 를 사용할수없다.
~~~ rust
//ex
const THREE_HOURS_IN_SECONDS: u32 =60*60*3;
~~~
* ## 3.1.2 섀도잉
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
