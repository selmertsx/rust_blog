## postgres docker 環境構築

```
$ docker compose up -d
```

## psql install

```
$ brew install libpq
$ echo 'export PATH="/usr/local/opt/libpq/bin:$PATH"' >> ~/.zshrc
$ exec $SHELL -l
```

## psql 接続

```
$ psql -h localhost -p 5432 -U user -d sample_service
// 開発環境ではpassword
Password for user user:
psql (14.1)
Type "help" for help.

sample_service=#
```

## diesel cli install

```
// default では mysql等のpluginも求められてしまうので、postgresのみinstallさせる。
$ cargo install diesel_cli --no-default-features --features postgres
```
