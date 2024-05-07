## Cài đặt môi trường, code rust cơ bản
  https://github.com/Callum-A/cosmwasm-zero-to-hero (phần 01, 02, 03)
## Khởi tạo contract theo template 
  ``` cargo generate --git https://github.com/CosmWasm/cw-template.git --name project_name ```
## Optimize code 
  1. Cài đặt docker: https://www.docker.com/products/docker-desktop/
  2. Chạy lệnh optimize: <br />
  
Linux: <br />
```
sudo docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/optimizer:0.15.0
```

Window: <br />
```
docker run --rm -v ${PWD}:/code `
  --mount type=volume,source="$(Get-Location | ForEach-Object { $_.Path.Substring($_.Path.LastIndexOf('\')+1) })_cache",target=/code/target `
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry `
  cosmwasm/rust-optimizer:0.15.0
``` 
## Deploy (Instantiate) contract
  1. Cài đặt module: ```npm install cosmwasm ```
  2. Instantiate/Execute/Query contract: ```/scripts/script.js```

## Lưu ý
  1. Tuyệt đối ko push file ```.env``` lên git -> cần thêm vào file ```.gitignore```
  2. Sau khi khởi tạo contract theo template cần chỉnh file ```cargo.toml``` theo contract mẫu vì Oraichain chưa tương thích với bản ```cosmwasm 2.0.0```
