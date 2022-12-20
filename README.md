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

1. 받은 압축파일의 압축을 푼 후에 실행하면 됩니다.

2. 원하는 모드를 고른 후 설명에 맞게 시간이나 시각을 입력하시면 됩니다.

    * `Work time` : 시작시각과 끝시각을 입력하면 시간을 계산해줍니다. 계속 입력하면 누적시간 역시 계산해줍니다.
    * `Minimum finish time` : 시작시각을 입력하면 하루 최소 근무시간인 4시간을 만족하는 최단 퇴근 시각을 계산해줍니다.
    * `Calculate finish time` : 시작시각과 원하는 근무시간을 입력하면 그에 맞는 퇴근 시각을 계산해줍니다.

3. 만일 종료하고 싶다면 `Ctrl + c`를 입력하거나 창을 종료하면 됩니다.

### Cargo로 설치했을 때

1. `yonsei-flexible` 을 터미널이나 CMD에서 실행합니다.

2. 원하는 모드를 고른 후 설명에 맞게 시간이나 시각을 입력하시면 됩니다.

    * `Work time` : 시작시각과 끝시각을 입력하면 시간을 계산해줍니다. 계속 입력하면 누적시간 역시 계산해줍니다.
    * `Minimum finish time` : 시작시각을 입력하면 하루 최소 근무시간인 4시간을 만족하는 최단 퇴근 시각을 계산해줍니다.
    * `Calculate finish time` : 시작시각과 원하는 근무시간을 입력하면 그에 맞는 퇴근 시각을 계산해줍니다.

3. 만일 종료하고 싶다면 `Ctrl + c`를 입력하거나 창을 종료하면 됩니다.

## 버전 정보

[RELEASES.md](./RELEASES.md)
