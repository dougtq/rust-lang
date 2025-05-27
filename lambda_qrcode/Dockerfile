# Etapa 1: build
FROM rust:1.77-slim as builder

# Instalar dependências necessárias
RUN apt-get update && \
  apt-get install -y musl-tools pkg-config libssl-dev && \
  rustup target add x86_64-unknown-linux-musl

# Diretório de trabalho
WORKDIR /app

# Copiar manifestos
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build em release com target compatível Lambda
RUN cargo build --release --target x86_64-unknown-linux-musl

# Etapa 2: imagem final
FROM alpine:3.18

# Instalar libc compatível
RUN apk add --no-cache ca-certificates

# Copiar binário da etapa de build
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/lambda_qrcode /bootstrap

# AWS Lambda Runtime Interface Emulator (opcional local test)
# COPY --from=public.ecr.aws/lambda/provided:al2 /opt/bootstrap /opt/bootstrap

# Tornar binário executável
RUN chmod +x /bootstrap

# Definir ponto de entrada para Lambda
ENTRYPOINT ["/bootstrap"]
