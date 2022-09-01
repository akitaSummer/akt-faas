
# Rust function example
```sh
cargo run --release

cd examples/rust

# 压缩
tar -czvf source.tar.gz -C hello/ .

# config

JSON=$(cat <<EOF
{
    "name": "hello-rust",
    "language": "rust",
    "source": "$(base64 -w 0 source.tar.gz)",
    "method": "GET",
    "path": "/hello-rust/",
    "cpu": "2",
    "memory": "512m",
    "uptime": "30"
}
EOF
)

# run
curl -X POST -H "Content-Type:application/json" -d "$JSON" "http://localhost:8080/function/"


# invoke
curl -X GET "http://localhost:8080/hello-rust/"

# delete

curl -X DELETE -H "Content-Type:application/json" -d "$JSON" "http://localhost:8080/function/"

```