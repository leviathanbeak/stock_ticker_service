# Stock Ticker Service

### Info

current Stocks Supported ["GOOG", "APPL", "TSLA", "AMZN", "MSFT", "FB"]

# Instructions

```shell
$ cargo build
$ cargo run
```

### Get Summary

Make a GET request to

```
http://127.0.0.1:3000/summary?stocks=APPL,GOOG
```

### Connect via websocket

- open static/websocket.html in your browser
- click "Connect" button
- send messages in this format "/subscribe APPL,GOOG"
