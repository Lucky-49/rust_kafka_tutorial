Это рабочий пример для создания защищенного по SSL/TLS кластера Kafka (Kraft - без zookeepr) (ОС Windows).

Возможно, Вам потребуется настроить своё окружение: необходимо наличие CMake, GCC или Clang, Make, pkg-config, OpenSSL.

Проверка: cmake --version, gcc --version, clang --version, make --version, pkg-config --version, openssl --version


Настроен Prometheus на сбор метрик с Kafka.

Настроена Grafana на работу с Prometheus. 

Настроен UI for Apache Kafka для мониторинга и управления кластером Kafka 

Всё запускается в контейнере командой docker-compose up -d

Перед сборкой необходимо:

1. Скачать JMX Prometheus Java Agent c https://repo1.maven.org/maven2/io/prometheus/jmx/jmx_prometheus_javaagent/ файл jmx_prometheus_javaagent-1.0.1.jar (или более поздней версии).

В проекте создаём папку jmx_exporter и туда помещаем файл jmx_prometheus_javaagent-1.0.1.jar (версия может быть, к примеру, 1.0.2)

2. Создать ключи и сертификаты в соответствии с инструкцией (Create keys for SSL.docx)
3. Разместить ключи и сертификаты в соответствии со структурой проекта

Prometheus просматривается на 127.0.0.1:9090

Grafana просматривается на 127.0.0.1:3000

UI for Apache Kafka просматривается на 127.0.0.1:8080

WARNING!!! Проты по которым Вы будете подключаться к Prometheus, Grafana и UI for Apache Kafka не защищены!!!

При первом замуске Grafana вводим username/password admin/admin. Будет предложена смена пароля. Далее
![image](https://github.com/user-attachments/assets/305e3b11-097b-48ac-943c-ed2c0abd33c9)
![image](https://github.com/user-attachments/assets/ac0d99f3-70b4-45b6-abb3-9a28efdf6a30)
![image](https://github.com/user-attachments/assets/ba1a0864-bf52-41de-a225-dc4508092c15)

При подключении Grafana к Prometheus используем http://<container_name_prometheus>:9090

![image](https://github.com/user-attachments/assets/b651867f-3d8b-48f5-9ea8-3c2096894706)
![image](https://github.com/user-attachments/assets/cb387a66-f4eb-4cf6-a7c8-a8dbf414e78d)

В dashboard Grafana загружаем ID 7589

![image](https://github.com/user-attachments/assets/5442c349-0230-48c6-8403-bb353a3ca122)
![image](https://github.com/user-attachments/assets/35362f8f-421f-407c-aa62-51754e2d09c5)

После первого запуска всех контейнеров в корневой директории проекта будет создана папка 'grafana-data' 
которая будет связана с такой же папкой в контейнере grafana.

Этот минимальный набор действий выведет информацию, подтверждающую подключение и корректную работу всех элементов системы.

Обязательно ставим звезду.
![img](https://github.com/user-attachments/assets/5824242f-defe-4cfe-8c12-318f5f9378b6)

Пользуйтесь на здоровье!!!
