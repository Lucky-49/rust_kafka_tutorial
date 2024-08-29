Это полностью рабочий пример для создания кластеров Zookeeper и Kafka (ОС Windows).

Возможно, Вам потребуется настроить своё окружение: необходимо наличие CMake, GCC или Clang, Make, pkg-config.

Проверка:

cmake --version
gcc --version

clang --version

make --version

pkg-config --version


Настроен Prometheus на сбор метрик с Kafka.

Настроена Grafana на работу с Prometheus. 

Настроен Kafdrop для мониторинга и управления кластерами Kafka 

Всё запускается в контейнере командой docker-compose up -d

Перед сборкой необходимо скачать JMX Prometheus Java Agent c https://repo1.maven.org/maven2/io/prometheus/jmx/jmx_prometheus_javaagent/ файл jmx_prometheus_javaagent-1.0.1.jar (или более поздней версии).

В проекте создаём папку jmx_exporter и туда помещаем файл jmx_prometheus_javaagent-1.0.1.jar (версия может быть, к примеру, 1.0.2)

Producer и Consumer запускается в отдельных терминалах, каждый из своей директории cargo run, перед настройкой Grafana.

Prometheus просматривается на 127.0.0.1:9090

Grafana просматривается на 127.0.0.1:3000

Kafdrop просматривается на 127.0.0.1:9000

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

Этот минимальный набор действий выведет информацию, подтверждающую подключение всех эдементов системы.

Обязательно нажимаем звезду - это поможет понять скольким людям я помог. Пользуйтесь на здоровье!!!
