# RustSense

Stream raw PCM audio from stdin to a Sony DualSense (PS5) or DualSense Edge controller over USB HID, playing it through the controller's 3.5mm headphone jack.

## Requirements

- Rust **1.85 or newer** (edition 2024)
- A DualSense or DualSense Edge controller connected via **USB** (Bluetooth is not supported)

**Linux** — install the hidapi system library:

```sh
# Debian / Ubuntu
sudo apt install libhidapi-dev

# Fedora
sudo dnf install hidapi-devel

# Arch
sudo pacman -S hidapi
```

**Windows** — no extra dependencies. `hidapi` links against `hid.dll`, which ships with Windows. A working C compiler is required for the build (MSVC via Visual Studio Build Tools, or MinGW via MSYS2).

## Building

```sh
cargo build --release
```

The binary ends up at `target/release/RustSense` (Linux) or `target\release\RustSense.exe` (Windows).

## Connecting the controller

Plug the DualSense into a USB port. Do **not** use a USB hub if you experience write errors — connect directly.

### Linux: grant access to the HID device

By default `/dev/hidraw*` nodes require root. Either run with `sudo`, or add a persistent udev rule:

```sh
sudo tee /etc/udev/rules.d/70-dualsense.rules <<'EOF'
# DualSense
SUBSYSTEM=="hidraw", ATTRS{idVendor}=="054c", ATTRS{idProduct}=="0ce6", MODE="0660", GROUP="plugdev"
# DualSense Edge
SUBSYSTEM=="hidraw", ATTRS{idVendor}=="054c", ATTRS{idProduct}=="0df2", MODE="0660", GROUP="plugdev"
EOF

sudo udevadm control --reload-rules
sudo udevadm trigger
```

Make sure your user is in the `plugdev` group:

```sh
sudo usermod -aG plugdev $USER   # log out and back in after this
```

### Windows

No driver installation is needed. Windows recognises the DualSense as a standard HID device automatically. Run the binary from a normal command prompt or PowerShell — no elevation required.

## Usage

RustSense reads **signed 16-bit little-endian stereo PCM at 48 000 Hz** from stdin and forwards it to the controller.

**Linux:**

```sh
# From a file
ffmpeg -i input.mp3 -f s16le -ar 48000 -ac 2 - | ./target/release/RustSense

# From a running audio source (PipeWire / PulseAudio)
pacat --record --raw --rate=48000 --channels=2 --format=s16le | ./target/release/RustSense
```

**Windows (PowerShell):**

```powershell
# From a file
ffmpeg -i input.mp3 -f s16le -ar 48000 -ac 2 - | .\target\release\RustSense.exe
```

**Windows (Command Prompt):**

```cmd
ffmpeg -i input.mp3 -f s16le -ar 48000 -ac 2 - | .\target\release\RustSense.exe
```

### Logging

Set the `RUST_LOG` environment variable to control log verbosity (`error`, `warn`, `info`, `debug`, `trace`):

```sh
# Linux
RUST_LOG=info ./target/release/RustSense

# Windows PowerShell
$env:RUST_LOG="info"; .\target\release\RustSense.exe

# Windows Command Prompt
set RUST_LOG=info && .\target\release\RustSense.exe
```

## How it works

RustSense discovers the controller by matching Sony's USB vendor ID (`0x054C`) against the known DualSense product IDs, then enters a tick loop timed to drain exactly 64 audio bytes per tick.

**Tick rate:** `1 000 000 000 × 64 / (3000 × 2)` ns ≈ 10.667 ms per tick (3 000 ticks/s × 2 channels). If a tick is missed it is skipped, not queued.

Each tick the program reads 64 bytes from stdin and packs them into a 142-byte USB HID output report:

| Offset | Size | Field | Value |
|--------|------|-------|-------|
| 0 | 1 B | `report_id` | `0x32` |
| 1 | 1 B | `padding` | `0x00` |
| 2 | 1 B | `tag` | `0x91` |
| 3 | 1 B | `seq` | `7` (fixed) |
| 4 | 7 B | `unknown_data` | `[0xFE, 0, 0, 0, 0, 0xFF, counter]` |
| 11 | 1 B | `payload_tag` | `0x92` |
| 12 | 1 B | `payload_length` | `64` |
| 13 | 64 B | `audio_data` | PCM from stdin |
| 77 | 61 B | `empty_space` | zeroed |
| 138 | 4 B | `crc32` | CRC-32 of bytes `[0..138]` |

`counter` in `unknown_data[6]` is incremented (wrapping) on every packet. The `seq` field is intentionally fixed at `7` and is never incremented.

**CRC:** standard CRC-32 polynomial (`0xEDB88320`), but with a **non-standard initial value of `!0xEADA2D49`** instead of the usual `0xFFFFFFFF`. The firmware validates this exact seed and will reject packets with a standard CRC.

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.
