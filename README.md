# 연세대학교 전문연구요원 시간 계산기

## 설치

### Rust 설치

[Rustup](https://rustup.rs/) 에 접속하여 Rust를 설치합니다.

혹시 어려움이 있다면 다음의 링크를 참고하여 설치해보세요.

[https://yoongrammer.tistory.com/12](https://yoongrammer.tistory.com/12)

### Yonsei-Flexible 설치

1. Linux나 MacOS 유저라면 Terminal을 열고 Windows 유저라면 cmd나 powershell을 실행합니다.

2. 다음 명령어를 실행하여 설치합니다.

    ```sh 
    cargo install yonsei-flexible
    ```

## 사용

1. Terminal이나 cmd 등을 열어 다음을 입력합니다.

    ```sh
    yonsei-flexible
    ```

2. 입력 창에 전문연 출근 시간과 퇴근 시간을 다음과 같이 입력합니다.

    ```sh
    09:00:00 - 18:00:00
    ```

3. `Total time`과 `Work time`이 계산되어 나옵니다. `Work time`이 실제로 인정되는 시간입니다.

