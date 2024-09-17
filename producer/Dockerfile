# Используем официальный образ Rust для сборки
FROM rust:1.80 AS builder

# Устанавливаем рабочую директорию
WORKDIR /app/producer

# Устанавливаем CMake и другие зависимости, необходимые для сборки librdkafka
RUN apt-get update && apt-get install -y cmake libssl-dev pkg-config

# Копируем файлы проекта
COPY ./producer ./

# Копируем Cargo.lock
COPY Cargo.lock ./

# Собираем проект
RUN cargo build --release

# Переходим на минимальный образ для запуска
FROM debian:bookworm-slim

# Устанавливаем необходимые зависимости для запуска скомпилированного бинарника
RUN apt-get update && apt-get install -y ca-certificates libssl3

# Копируем скомпилированный бинарник из предыдущего шага
COPY --from=builder /app/producer/target/release/producer /usr/local/bin/producer

# Устанавливаем рабочую директорию
WORKDIR /app

# Копируем сертификаты
COPY ./secrets/producer /app/producer

# Указываем команду запуска
CMD ["producer"]
