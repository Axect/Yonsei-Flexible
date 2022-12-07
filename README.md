# 연세대학교 전문연구요원 유연근무제 시간 계산기

## 설치 (Pre-built binaries)

[Releases](https://github.com/Axect/Yonsei-Flexible/releases) 링크로 들어가 자신의 OS에 맞는 zip 파일을 받아 사용하시면 됩니다.

## 설치 (via Cargo)

1. [한국 러스트 사용자 그룹](https://rust-kr.org/pages/install/)에서 권장하는대로 Rust를 설치합니다.
2. Rust의 패키지매니저인 `cargo`를 이용하여 yonsei-flexible을 설치합니다.
    ```sh
    cargo install yonsei-flexible
    ```

**주의** : 위의 방법이 안될 시 cargo 를 포함한 디렉토리가 환경변수에 입력되어 있는지 확인합니다. Linux의 경우에 `$HOME/.cargo/bin` 디렉토리가 `PATH`에 들어 있는지 확인해주세요.

## 사용

### Pre-build binary 로 설치했을 때

1. Terminal이나 cmd 등을 열어 압축을 푼 파일의 디렉토리로 이동한 후 받은 파일을 실행합니다.
    
    * Linux의 경우에는 `./yonsei-flexible` 로 실행하면 됩니다.
    * Windows CMD의 경우에는 `yonsei-flexible.exe`를 CMD에서 입력하여 실행하면 됩니다.
    * 기타 다른 OS의 경우에는 그 OS에 맞는 실행방법으로 실행하시면 됩니다.

2. 입력 창에 전문연 출근 시간과 퇴근 시간을 다음과 같이 입력합니다.

    ```sh
    > 09:00:00 - 18:00:00
    ```

3. `Total time`과 `Work time`이 계산되어 나옵니다. `Work time`이 실제로 인정되는 시간입니다.

4. 시간이 표시되고 또 다시 입력하라는 안내가 나올텐데, 추가적으로 입력하면 전체 합산 시간을 계산합니다. 합산 시간은 `Accumulated time`으로 표시됩니다.

5. 만일 종료하고 싶다면 `Ctrl + c`를 입력하거나 창을 종료하면 됩니다.


### Cargo로 설치했을 때

1. `yonsei-flexible` 을 터미널이나 CMD에서 실행합니다.

2. 입력 창에 전문연 출근 시간과 퇴근 시간을 다음과 같이 입력합니다.

    ```sh
    > 09:00:00 - 18:00:00
    ```

3. `Total time`과 `Work time`이 계산되어 나옵니다. `Work time`이 실제로 인정되는 시간입니다.

4. 시간이 표시되고 또 다시 입력하라는 안내가 나올텐데, 추가적으로 입력하면 전체 합산 시간을 계산합니다. 합산 시간은 `Accumulated time`으로 표시됩니다.

5. 만일 종료하고 싶다면 `Ctrl + c`를 입력하거나 창을 종료하면 됩니다.

## 버전 정보

[RELEASES.md](./RELEASES.md)
