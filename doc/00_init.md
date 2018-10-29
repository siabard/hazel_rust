# Git init

```
git remote add origin https://github.com/siabard/hazel_rust.git
```

# 프로젝트 준비 

Hazel 엔진은 lib 형태로 존재할 것이라 main.rs는 필요없을 것이다.

# Example 폴더

examples/sandbox/main.rs 를 만들고 Cargo.toml에 example 섹션을 추가한다.

```
[[example]]
name = "sandbox"
path = "examples/sandbox/main.rs"

```
# prelude 모듈

src/prelude.rs 를 만들고 lib.rs 에 해당 모듈을 선언한다.

```
pub mod prelude;
```

prelude 모듈은 간편하게 로딩을 하기 위한 wrapping module이다.

# Test case 생성

lib.rs에 테스트 케이스를 생성한다.

```
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(true);
    }
}
```

라이브러리 함수를 구성해본다.

```
pub fn print() {
    println!("Welcome to hazel engine");
}
```

이후 `cargo test`를 통해 테스트 케이스가 잘 통과하는지 확인하면 된다.

# sandbox 스크립트 작성

examples/sandbox/main.rs 에 라이브러리 모듈 로딩이 잘 되는지 확인한다.

```
extern crate hazel_rust;

use hazel_rust::prelude::*;

fn main() {
    hazel_rust::print():
}
```

```
cargo run --example sandbox
```
