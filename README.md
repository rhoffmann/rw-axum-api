## Dev Setup

### Initial Docker Setup
```
docker run --name <dbname> \
  -e POSTGRES_PASSWORD=<db_password> \
  -e POSTGRES_USER=<db_user> \
  -e POSTGRES_DB=<db_name> \
  -p 5432:5432 \
  -d postgres:15
```
