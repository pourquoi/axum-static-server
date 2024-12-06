Axum static file server
---

install linux
```bash
# linux x86_64
curl -o ass https://github.com/pourquoi/axum-static-server/ass-x86_64
# arm 
# curl -o ass https://github.com/pourquoi/axum-static-server/ass-arm
chmod a+x ./ass
```

run
```bash
# serve static files in ./public directory
./ass --host=127.0.0.1 --port=3000 --dir=./public
```
