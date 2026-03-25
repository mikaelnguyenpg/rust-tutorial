## Assignment

### Sun - 22-03-2026

#### Questions

1. Thực hành thêm column và kiểm tra code mới trên database của mình đảm bảo chạy được
2. Kiểm tra và sửa lỗi tại sao api get_users lại return là []
3. Thiết kế 2 tables: user và article theo yêu cầu từ file excalidraw

```plaintext
Dự án: Xây dựng một WebApp đưa tin giống Facebook

- Homepage: liệt kê tất cả các bản tin của người dùng
dưới dạng là public
- Nếu user login: liệt kê hết tất cả các dạng bản tin

- User (người dùng)
    - ...
- Article (bản tin)
    - time_created
    - visibility: public / unlisted / draft
    - created_by_user
```

#### Answers

##### 1. Thực hành thêm column

Giải pháp:

```sql
-- add password to Schema
CREATE TABLE IF NOT EXISTS users_demo (
    id serial primary key,
    name TEXT NOT NULL,
    password TEXT NOT NULL,
    email TEXT
);

-- ALTER TABLE users_demo ADD password TEXT;
CREATE INDEX IF NOT EXISTS user_demo__name_index ON users_demo (name);
```

Result:

```bash
$ psql "postgresql://admin:admin123@localhost:5432/mydatabase"
psql (17.9)
Type "help" for help.

mydatabase=# \dt
          List of relations
 Schema |    Name    | Type  | Owner
--------+------------+-------+-------
 public | users_demo | table | admin
(1 row)

mydatabase=# \d users_demo
                             Table "public.users_demo"
  Column  |  Type   | Collation | Nullable |                Default
----------+---------+-----------+----------+----------------------------------------
 id       | integer |           | not null | nextval('users_demo_id_seq'::regclass)
 name     | text    |           | not null |
 password | text    |           | not null |
 email    | text    |           |          |
Indexes:
    "users_demo_pkey" PRIMARY KEY, btree (id)
    "user_demo__name_index" btree (name)

```

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

Further issue:

- Database requires NOT-NULL password <-> conflict create_user & login not pass password
- `POST /api/user` requires Token to complete API --> NOT appropriated

#### 2.B. Sửa lỗi tại sao api get_users lại return là []

**Giải pháp**:

- 2 APIs `POST /api/user` & `POST /api/auth/login` cần truyền password, 4 APIs còn lại ko-bắt-buộc truyền password
- 2 APIs `POST /api/user` & `POST /api/auth/login` ko cần truyền Authentication, 4 APIs còn lại nên có Authentication

**Fixing Phases**:

- Add required-password to APIs --> ✅
- Remove Authentication/security from `POST /api/user` --> ✅
- Remove required-password to 4 APIs --> ✅
- Add Authentication/security to 4 APIs --> ✅
- Remove returned-password from `GET /api/users`
