# Authd

An authentication service for multiple projects, organizaitons, and web services.

Authd is a REST api for self-hosted services, DIYers, and organizations that want to own their data.

## Setup

### Auth-cmd

```sh
auth-cmd setup ./db.sqlite
auth-cmd maintenance ./db.sqlite
```

## Start a service

It's recommended to use a service manager like `systemd`.

### Auth-server

Use the following command to run an api server:

```sh
auth_server ./db.sqlite 127.0.0.1:4000
```

## Todos

- last read at for data that needs cleanup
- a way to reset a "secret" from the admin api and reset that stuff

