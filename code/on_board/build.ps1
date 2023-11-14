[CmdletBinding()]
Param(
    [switch] $NoBuild,
    [switch] $DebugBuild,
    [switch] $ProgramOOCD
)

If (-not $NoBuild) {
    If ($DebugBuild) {
        & cargo build
    } Else {
        & cargo build --release
    }
}

$elfBinary = If ($DebugBuild) {
    ".\target\thumbv6m-none-eabi\debug\enocean-heater-control"
} Else {
    ".\target\thumbv6m-none-eabi\release\enocean-heater-control"
}

If (-not $NoBuild) {
    & rust-objcopy --output-target=binary "$elfBinary" ".\ehc.bin"
}

$kilobytes = (Get-Item -LiteralPath ".\ehc.bin").Length / 1024
Write-Output ("{0:#,##0.###} KiB" -f $kilobytes)

If ($ProgramOOCD) {
    & "C:\Program Files\OpenOCD\bin\openocd.exe" `
        -c "set BINFILE ehc.bin" `
        -c "source oocd-prog-jlink.cfg"
}
