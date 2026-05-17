# Yomichan/Yomitan Audio Server

Fast local HTTP audio for Yomitan-compatible readers, including Yomitan and よむ.

This is a source-published fork of `aramrw/yomichan_audio_server`. The fork is kept because large local audio collections should feel instant in daily lookup: the server uses the bundled entries database, source ordering, and faster local file resolution. The caching/lookup work was not merged upstream, so this repo tracks the version I actually use.

## URL to Paste Into Your Reader

Run the server, then add this as a custom JSON audio source:

```text
http://localhost:9393/?term={term}&reading={reading}
```

Port `9393` is just a safe example. The original default is `8080`, but many local tools also use `localhost:8080`, so pick another port if needed:

```bash
yomichan_audio_server --port 9393 --audio ./audio
```

To test the server in a browser:

```text
http://localhost:9393/?term=猫&reading=ねこ
```

The response should be JSON with `type: "audioSourceList"` and an `audioSources` array.

## Audio Folder Layout

Put your audio folders under one `audio` directory next to the binary, or pass the folder with `--audio`.

```text
yomitan-audio/
├── audio/
│   ├── daijisen/media/
│   ├── jpod/media/
│   ├── nhk16/media/
│   ├── shinmeikai8/media/
│   ├── forvo_jp/
│   └── forvo_zh/
└── yomichan_audio_server
```

You can use audio folders from the upstream release assets or from your own Ultimate Yomitan Audio Source download, as long as the folder names match the server sources.

## Download or Build

Prebuilt downloads for this fork are on the release page:

https://github.com/HRussellZFAC023/yomichan_audio_server/releases/latest

Choose the download for your computer:

- Windows: `windows-x86_64.zip`
- Apple Silicon Mac: `macos-aarch64.tar.gz`
- Intel Mac: `macos-x86_64.tar.gz`
- Linux: `linux-x86_64.tar.gz`

The downloaded server already has the entries database embedded. Audio files are not bundled; put your audio folders in an `audio` folder next to the server, or pass a folder with `--audio`.

If you build this fork from source, download `entries.db` from the upstream release assets first and place it in the repo root. Source builds need the file at compile time because `src/main.rs` includes it in the executable.

```bash
git clone https://github.com/HRussellZFAC023/yomichan_audio_server.git
cd yomichan_audio_server
curl -L -o entries.db https://github.com/aramrw/yomichan_audio_server/releases/download/v0.0.3/entries.db
cargo build --release
./target/release/yomichan_audio_server --port 9393 --audio ./audio --log full
```

On Windows, run the same command shape from PowerShell after installing Rust:

```powershell
git clone https://github.com/HRussellZFAC023/yomichan_audio_server.git
cd yomichan_audio_server
Invoke-WebRequest -Uri "https://github.com/aramrw/yomichan_audio_server/releases/download/v0.0.3/entries.db" -OutFile "entries.db"
cargo build --release
.\target\release\yomichan_audio_server.exe --port 9393 --audio .\audio --log full
```

## Add It to よむ

1. Open a page where よむ is running.
2. Open settings with the floating よむ button or `Alt+Shift+J`.
3. Go to Audio.
4. Press Add audio source.
5. Set Type to Custom URL JSON.
6. Paste `http://localhost:9393/?term={term}&reading={reading}`.
7. Save, then try a lookup and press the speaker button.

Move this source above the built-in sources if you want it to be tried first.

## Add It to Yomitan

1. Open Yomitan settings.
2. Go to Audio.
3. Add an audio source.
4. Set Type to Custom URL JSON.
5. Paste the server URL, for example:

```text
http://localhost:9393/?term={term}&reading={reading}
```

## Run on Startup

Use a full path in startup commands. That avoids surprises when the operating system starts the task from a different working directory.

### Windows

Create a small launcher file next to the server:

```bat
@echo off
cd /d "C:\Tools\yomitan-audio"
"C:\Tools\yomitan-audio\yas-x86_64-pc-windows.exe" --port 9393 --audio "C:\Tools\yomitan-audio\audio" --log headless
```

Save it as:

```text
C:\Tools\yomitan-audio\start-yomu-audio.cmd
```

Then register it to run when you log in:

```powershell
schtasks /Create /TN "Yomu Local Audio" /SC ONLOGON /TR "`"C:\Tools\yomitan-audio\start-yomu-audio.cmd`"" /F
```

Remove it later with:

```powershell
schtasks /Delete /TN "Yomu Local Audio" /F
```

### macOS

Build or place the binary somewhere stable, for example:

```text
/Users/you/Tools/yomitan-audio/yomichan_audio_server
```

Create `~/Library/LaunchAgents/com.yomu.audio-server.plist`:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>Label</key>
  <string>com.yomu.audio-server</string>
  <key>WorkingDirectory</key>
  <string>/Users/you/Tools/yomitan-audio</string>
  <key>ProgramArguments</key>
  <array>
    <string>/Users/you/Tools/yomitan-audio/yomichan_audio_server</string>
    <string>--port</string>
    <string>9393</string>
    <string>--audio</string>
    <string>/Users/you/Tools/yomitan-audio/audio</string>
    <string>--log</string>
    <string>headless-instance</string>
  </array>
  <key>RunAtLoad</key>
  <true/>
  <key>KeepAlive</key>
  <true/>
  <key>StandardOutPath</key>
  <string>/tmp/yomu-audio.log</string>
  <key>StandardErrorPath</key>
  <string>/tmp/yomu-audio.err</string>
</dict>
</plist>
```

Load it:

```bash
launchctl bootstrap gui/$(id -u) ~/Library/LaunchAgents/com.yomu.audio-server.plist
launchctl enable gui/$(id -u)/com.yomu.audio-server
```

Restart it after edits:

```bash
launchctl kickstart -k gui/$(id -u)/com.yomu.audio-server
```

Remove it later with:

```bash
launchctl bootout gui/$(id -u) ~/Library/LaunchAgents/com.yomu.audio-server.plist
```

## Use It From an iPad or Another Device With Tailscale

The server listens on `localhost`, so another device cannot use `http://localhost:9393` directly. Use Tailscale Serve on the computer running the audio server:

```bash
tailscale serve --bg --https=443 http://127.0.0.1:9393
tailscale serve status
```

`tailscale serve status` prints the private tailnet URL. It usually looks like:

```text
https://desktop.your-tailnet.ts.net
```

Use that as the base URL in よむ or Yomitan:

```text
https://desktop.your-tailnet.ts.net/?term={term}&reading={reading}
```

Every device must be signed in to the same Tailscale network. Keep the computer running; the iPad only reaches the audio while that computer and Tailscale are online.

## Sorting Sources

Run this to see source names:

```bash
yomichan_audio_server --sources
```

Create `sort.txt` next to the binary with one source per line:

```text
daijisen
nhk16
shinmeikai8
forvo_jp
forvo_zh
jpod
```

The first source in the file is tried first.

## Troubleshooting

Run with full logs while testing:

```bash
yomichan_audio_server --port 9393 --audio ./audio --log full
```

Common fixes:

- If port `8080` is already taken, use `--port 9393` and paste the matching URL into your reader.
- If the browser returns no audio, check that the source folders are inside the `audio` folder and that each source name matches the expected folder name.
- If another device cannot connect, use the Tailscale Serve URL rather than `localhost`.
- If you build from source and compilation cannot find `entries.db`, download it into the repo root before running `cargo build`.
