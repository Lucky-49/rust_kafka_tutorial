Это полностью рабочий пример для создания кластеров Zookeeper и Kafka.

Настроен Prometheus на сбор метрик с Kafka.

Настроена Grafana на работу с Prometheus. 

Всё запускается в контейнере командой docker-compose up -d

Перед сборкой необходимо скачать JMX Prometheus Java Agent c https://repo1.maven.org/maven2/io/prometheus/jmx/jmx_prometheus_javaagent/

В проекте создаём папку jmx_exporter и туда помещаем файл jmx_prometheus_javaagent-1.0.1.jar (версия может быть, к примеру, 1.0.2)

Producer и Consumer запускается в отдельных терминалах, каждый из своей директории cargo run

Grafana просматривается на 127.0.0.1:3000

При первом замуске Grafana вводим username/password admin/admin. Будет предложена смена пароля. Далее переходим на вкладку Connections -> Add
![image](https://github.com/user-attachments/assets/305e3b11-097b-48ac-943c-ed2c0abd33c9)


При подключении Grafana к Prometheus используем http://<container_name_prometheus>:9090

В dashboard Grafana загружаем ID 7589

Обязательно нажимаем звезду - это поможет понять скольким людям я помог. Пользуйтесь на здоровье!!!
