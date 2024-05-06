# Cài đặt môi trường, code rust cơ bản
  https://github.com/Callum-A/cosmwasm-zero-to-hero (phần 01, 02, 03)
# Khởi tạo contract theo template 
  ''' cargo generate --git https://github.com/CosmWasm/cw-template.git --name Tên_thư_mục '''
# Optimize code 
  Cài đặt docker: https://www.docker.com/products/docker-desktop/\\
  Chạy lệnh optimize:\\
    Linux: """docker run --rm -v "$(pwd)":/code \ \\
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/target \ \\
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \ \\
  cosmwasm/optimizer:0.15.0""" \\
    Window: """docker run --rm -v ${PWD}:/code ` \\
  --mount type=volume,source="$(Get-Location | ForEach-Object { $_.Path.Substring($_.Path.LastIndexOf('\')+1) })_cache",target=/code/target ` \\
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry `\\
  cosmwasm/rust-optimizer:0.15.0""" \\
# Deploy(Instantiate) contract
   Cài đặt module: npm install cosmwasm \\
   Instantiate/Execute/Query contract: /scripts/script.js \\
