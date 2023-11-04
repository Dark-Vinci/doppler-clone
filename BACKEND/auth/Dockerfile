FROM alpine3.17 as build_sdk

WORKDIR /app/user/sdk

COPY ./sdk /app/user/sdk

RUN cargo run build

FROM alpine3.17

WORKDIR /app/user/auth

ARG port=3001
ARG environment=development

COPY ./auth/cargo.toml /app/user/auth
COPY ./cargo.lock ./

RUN cargo build

COPY --from=build_sdk / /app/user/sdk

COPY ./auth /app/user/auth

ENV PORT=${port}
ENV ENVIRONMENT=${environment}

CMD if [ enviroment == production ]; then \
     cargo build --release && ./app/user/auth/target/release/auth; \
else \
     cargo run --watch;
fi

EXPOSE $PORT