#!/usr/bin/env bash
set -u

BIN_DIR=bin
export CARGO_TARGET_DIR=.target
TARGET_DIR=.target
TARGET=thumbv6m-none-eabi
FLASH_BASE=0x00000000
VECTOR_SUM_WORDS=7
VECTOR_SUM_OFFSET=28
OPENOCD=${OPENOCD:-openocd}
OCD_IF=${OCD_IF:-interface/cmsis-dap.cfg}
OCD_CFG=${OCD_CFG:-target/lpc11xx.cfg}

device_load() {
    DEVICE=$(sed -n 's/^default *= *\[ *"\([a-z0-9]*\)".*/\1/p' \
        Cargo.toml | head -n 1)
    NAME=$(sed -n 's/^name *= *"\([^"]*\)".*/\1/p' Cargo.toml |
        head -n 1)
    VERSION=$(sed -n 's/^version *= *"\([^"]*\)".*/\1/p' \
        Cargo.toml | head -n 1)
    FIRMWARE="$DEVICE-$NAME-$VERSION.bin"
}

setup() {
    command -v rustup >/dev/null || {
        echo "rustup not found: https://rustup.rs"
        return 1
    }
    rustup target add "$TARGET" || return 1
    rustup component add llvm-tools || return 1
    if ! command -v cargo-size >/dev/null ||
        ! command -v cargo-objcopy >/dev/null; then
        cargo install cargo-binutils || return 1
    fi
    echo "setup done"
}

menu_show() {
    echo "s -- setup"
    echo "b -- build release"
    echo "f -- flash target"
    echo "e -- erase target"
    echo "x -- shutdown"
}

checksum_word_read() {
    od -An -tu4 --endian=little -j "$(( $2 * 4 ))" -N4 "$1" |
        tr -d ' '
}

checksum_sum_get() {
    local file=$1 sum=0 i word
    for (( i = 0; i < VECTOR_SUM_WORDS; i++ )); do
        word=$(checksum_word_read "$file" "$i")
        sum=$(( (sum + word) & 0xFFFFFFFF ))
    done
    echo $(( (0 - sum) & 0xFFFFFFFF ))
}

checksum_bytes_write() {
    local file=$1 hex=$2
    printf "\\x${hex:6:2}\\x${hex:4:2}\\x${hex:2:2}\\x${hex:0:2}" |
        dd of="$file" bs=1 seek="$VECTOR_SUM_OFFSET" \
            conv=notrunc status=none
}

checksum_patch() {
    local file=$1 value hex
    if [[ ! -f "$file" ]]; then
        echo "missing $file"
        return 1
    fi
    value=$(checksum_sum_get "$file") || return 1
    printf -v hex '%08x' "$value"
    checksum_bytes_write "$file" "$hex" || return 1
    echo "vector checksum 0x$hex"
}

export_bin() {
    local out=$1 stem stamp dest
    mkdir -p "$BIN_DIR" || return 1
    stem=${out%.bin}
    stamp=$(date +%d%m%y-%H%M)
    dest="$BIN_DIR/fw-$stem-$stamp.bin"
    cp "$TARGET_DIR/$out" "$dest" || return 1
    echo "exported $dest"
}

build() {
    local flags=(--release --target "$TARGET" \
        --no-default-features --features "$DEVICE")
    cargo -q build "${flags[@]}" || return 1
    mkdir -p "$TARGET_DIR"
    cargo -q objcopy "${flags[@]}" -- \
        -O binary "$TARGET_DIR/$FIRMWARE" || return 1
    checksum_patch "$TARGET_DIR/$FIRMWARE" || return 1
    export_bin "$FIRMWARE"
}

ocd() {
    "$OPENOCD" -f "$OCD_IF" -c "transport select swd" \
        -f "$OCD_CFG" -c "init" "$@"
}

erase_target() {
    ocd -c "reset halt; flash probe 0; \
        flash erase_sector 0 0 last; shutdown"
}

firmware_require() {
    [[ -f "$TARGET_DIR/$FIRMWARE" ]] && return 0
    echo "missing $FIRMWARE, build first"
    return 1
}

flash_target() {
    firmware_require || return 1
    echo "flashing $FIRMWARE"
    ocd -c "reset halt" \
        -c "program $TARGET_DIR/$FIRMWARE verify reset exit \
$FLASH_BASE"
}

main() {
    device_load
    menu_show
    local cmd
    while true; do
        printf "> "
        read -r cmd || break
        case "$cmd" in
            s) setup ;;
            b) build ;;
            f) flash_target ;;
            e) erase_target ;;
            x) break ;;
            *) : ;;
        esac
    done
}

if [ -t 0 ]; then
    main "$@"
else
    title="$(basename $(pwd))"
    xfce4-terminal\
        --title="$title"\
        -e "bash -c '$0 $@; exec bash'"
fi