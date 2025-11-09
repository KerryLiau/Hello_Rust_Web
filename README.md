# 服務要求
### Postgres
### Jaeger 或其他能接收 otel 的服務
##### otel quick start
```shell
docker rm -f jaeger
docker run -d --rm --name jaeger \
  -e COLLECTOR_OTLP_ENABLED=true \
  -p 16686:16686 \
  -p 4317:4317 \
  -p 4318:4318 \
  -p 5778:5778 \
  -p 9411:9411 \
  cr.jaegertracing.io/jaegertracing/jaeger:2.11.0
```