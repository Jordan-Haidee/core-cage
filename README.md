# core-cage

A simple tool to limit core counts leveraged by specified process on Windows OS.

## Install from source

```powershell
cargo install --git https://github.com/Jordan-Haidee/core-cage.git
```

## Usage

```powershell
core-cage.exe 1234 3 # set process 1234 utilize 3 cores at most
```
