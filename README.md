Это полностью рабочий пример для создания кластеров Zookeeper и Kafka.
Настроен Prometheus на сбор метрик с Kafka.
Настроена Grafana на работу с Prometheus. 

Всё запускается в контейнере командой docker-compose up -d
Код rust запускается в терминале cargo run --quiet
Grafana просматривается на 127.0.0.1:3000
При подключении Grafana к Prometheus используем http://<container_name_prometheus>:9090
В dashboard Grafana загружаем ID 7589