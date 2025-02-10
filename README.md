# core-cage

A simple tool to limit core counts leveraged by specified process on Windows OS.

## Install from source

```powershell
cargo install --git https://github.com/Jordan-Haidee/core-cage.git
```

## Usage

```powershell
coca 1234 3 # pin process 1234 at 3 random cores
```
