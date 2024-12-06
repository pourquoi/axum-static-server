Axum static file server
---

install linux
```bash
# linux x86_64
curl -o ass -L https://github.com/pourquoi/axum-static-server/raw/refs/heads/main/ass-x86_64
# arm 
# curl -o ass -L https://github.com/pourquoi/axum-static-server/raw/refs/heads/main/ass-arm
chmod +x ./ass
```

run
```bash
# serve static files in ./public directory
./ass --host=127.0.0.1 --port=3000 --dir=./public
```
