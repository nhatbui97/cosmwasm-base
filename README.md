## Cài đặt môi trường, code rust cơ bản
  https://github.com/Callum-A/cosmwasm-zero-to-hero (phần 01, 02, 03)
## Khởi tạo contract theo template 
  ``` cargo generate --git https://github.com/CosmWasm/cw-template.git --name project_name ```
## Optimize code 
  1. Cài đặt docker: https://www.docker.com/products/docker-desktop/
  2. Chạy lệnh optimize: <br />
Linux: <br />
```
docker run --rm -v "$(pwd)":/code \ 
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
   1. Cài đặt module: npm install cosmwasm 
   2. Instantiate/Execute/Query contract: /scripts/script.js 
