# Small Auth

Example of using bcrypt hashing to allow a user to login and do actions.

The Sqlite Schema is:

```sql
CREATE TABLE users (id INTEGER PRIMARY KEY AUTOINCREMENT, username TEXT NOT NULL, password TEXT NOT NULL, hash TEXT NOT NULL, created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP, updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP, UNIQUE(username), UNIQUE(hash));
```

On account creation, the password is hashed, and a random string 30 character string is created. You would use this as a token, and you can invalidate it arbitrarily.
