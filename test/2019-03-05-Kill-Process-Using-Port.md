---
title: Kill the process using port
createdAt: 2019-03-05
category: ['devlogs']
keyword: ['macos', 'kill', 'port']
---

# Kill the process occupying a specific port

by Moondaddi on 2019-03-05

---

Sometimes you can encounter this error which says another process is already occupying the port. You can kill the app or process if you can, the problem is, sometimes it is not available. For example when app was teminated by abnormal ways, it remains and occupy the communication port in background.

## Check the current process using specific port

```shell
$ lsof -i:3000

# output
COMMAND     PID   USER   FD   TYPE             DEVICE SIZE/OFF NODE NAME
rust_grap 92304 myuser    3u  IPv4 0xffffffffffffffff      0t0  TCP localhost:arepa-cas (LISTEN)
```

## Kill the process

```shell
$ kill -9 92304
```

> use PID number (eg. 92304)
