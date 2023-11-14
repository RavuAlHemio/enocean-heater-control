$patch_yaml = "stm32g0b0.yaml"
$patched_svd = "stm32g0b0_patched.svd"

# blow up at first problem
$ErrorActionPreference = "Stop"

# clean src subdirectory
Remove-Item -Recurse -Force -Path "src"
New-Item -Type Directory -Path "src"

# patch SVD
& svdtools patch "$patch_yaml" "$patched_svd"
If ($LASTEXITCODE -ne 0) { Throw "Failed to patch SVD" }

# turn into Rust code
& svd2rust `
    --strict `
    --pascal_enum_values `
    --max_cluster_size `
    --atomics `
    --atomics_feature atomics `
    -i "$patched_svd"
If ($LASTEXITCODE -ne 0) { Throw "Failed to generate Rust code from patched SVD" }

# break into modules
& form `
    -i "lib.rs" `
    -o "src"
If ($LASTEXITCODE -ne 0) { Throw "Failed to split Rust code into modules" }

# beautify
& cargo fmt
If ($LASTEXITCODE -ne 0) { Throw "Failed to beautify Rust code" }
