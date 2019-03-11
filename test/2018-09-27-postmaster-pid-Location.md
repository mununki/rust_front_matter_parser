---

# 'postmaster.pid' File Location

by Moondaddi on 2018-09-27

## Where is the 'postmaster.pid'?

Time to time, I encounter the error when I try to start PostgreSQL DB again. Especially, right after cold power-off, it comes a lot to me. If you use GUI interface of PostgreSQL, it gives you a long instruction failed to start properly. But, simply, you can start again after delete this 'postmaster.pid' file in your macOS system.

<br />

## Location of 'postmaster.pid'

```shell
/Users/user_name/Library/Application Support/Postgres/var-10
```

> Note: PostgreSQL ver. 10, it can be changed depends on what version of postgres you use.
