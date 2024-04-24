FROM rust:latest as builder

COPY . .

WORKDIR /rs

RUN  cargo install --path .

WORKDIR /tools

RUN bash deploy.sh 

WORKDIR /

RUN tools/tailwindcss-linux-x64 -i /src/input.css -o /dist/output.css --minify

FROM httpd:2.4-alpine

COPY --from=builder ./dist/ /usr/local/apache2/htdocs/