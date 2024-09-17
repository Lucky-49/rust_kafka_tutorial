# Используем официальный образ Rust для сборки
FROM rust:1.80 AS builder

# Устанавливаем рабочую директорию
WORKDIR /app/consumer

# Устанавливаем CMake и другие зависимости, необходимые для сборки librdkafka
RUN apt-get update && apt-get install -y cmake libssl-dev pkg-config

# Копируем файлы проекта
COPY ./consumer ./

# Копируем Cargo.lock
COPY Cargo.lock ./

# Собираем проект
RUN cargo build --release

# Переходим на минимальный образ для запуска
FROM debian:bookworm-slim

# Устанавливаем необходимые зависимости для запуска скомпилированного бинарника
RUN apt-get update && apt-get install -y ca-certificates libssl3

# Копируем скомпилированный бинарник из предыдущего шага
COPY --from=builder /app/consumer/target/release/consumer /usr/local/bin/consumer

# Устанавливаем рабочую директорию
WORKDIR /app

# Копируем сертификаты
COPY ./secrets/consumer /app/consumer

# Указываем команду запуска
CMD ["consumer"]
