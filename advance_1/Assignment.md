## Assignment

### Sun - 22-03-2026

#### Questions

1. Thực hành thêm column và kiểm tra code mới trên database của mình đảm bảo chạy được
2. Kiểm tra và sửa lỗi tại sao api get_users lại return là []
3. Thiết kế 2 tables: user và article theo yêu cầu từ file excalidraw

#### Answers

##### 2.A. Kiểm tra tại sao api get_users lại return là []

Checking Workflow:

- Run advance_1/backend --> ✅
- On Swagger UI, call `POST /api/user` --> ✅
- Check Swagger result --> ❌ 500 - Error: Internal Server Error

```json
Missing request extension: Extension of type `backend::models::user::User` was not found. Perhaps you forgot to add it? See `axum::Extension`.
```

- Check database result --> ❌

```bash
# access Postgres database
$ psql "postgresql://admin:admin123@localhost:5432/mydatabase"

mydatabase=# \dt
mydatabase=# \d users_demo

# list users --> 0 - No results
mydatabase=# SELECT id, name, email, password FROM users_demo ORDER BY id;
 id | name | email | password
----+------+-------+----------
(0 rows)
```

--> FAILED to create new user --> `GET /api/users` return []

Root cause:

- `auth.rs - line.25` - `auth_header` == None --> `auth_value` == None
- --> `trans.rs - line.36` - `response.status()` != is_success --> `tx.rollback()`
