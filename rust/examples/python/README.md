# Python function example
```sh
cargo run --release

cd examples/python

# 压缩项目
tar -czvf source.tar.gz main.py

# 容器相关信息

JSON=$(cat <<EOF
{
    "name": "hello-python",
    "language": "python",
    "source": "$(base64 -w 0 source.tar.gz)",
    "method": "GET",
    "path": "/hello-python/",
    "cpu": "2",
    "memory": "512m",
    "uptime": "30"
}
EOF
)

# run
curl -X POST -H "Content-Type:application/json" -d "$JSON" "http://localhost:8080/function/"

# invoke

curl -X GET "http://localhost:8000/hello-python/"

# delete

curl -X DELETE -H "Content-Type:application/json" -d "$JSON" "http://localhost:8080/function/"

```
