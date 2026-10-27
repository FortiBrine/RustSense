Name:           rustsense
Version:        0.1.1
Release:        0
Summary:        Stream raw PCM audio to a Sony DualSense controller over USB HID
License:        MIT OR Apache-2.0
URL:            https://github.com/FortiBrine/RustSense
BuildRequires:  cargo
BuildRequires:  rust >= 1.85
BuildRequires:  pkgconfig
BuildRequires:  pkgconfig(libudev)

%description
RustSense reads signed 16-bit little-endian stereo PCM at 48000 Hz
from stdin and plays it through the headphone jack of a DualSense or
DualSense Edge controller connected over USB.

%prep

%build
cargo build --release --offline

%install
install -Dm755 target/release/RustSense %{buildroot}%{_bindir}/rustsense

%files
%license LICENSE-MIT LICENSE-APACHE
%doc README.md
%{_bindir}/rustsense

%changelog
