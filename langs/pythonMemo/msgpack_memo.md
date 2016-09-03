messagepack rpc
====

## 1. install

```sh
$ pip install msgpack-python msgpack-rpc-python
```


## mprpc (messagepack-rpc wrapper)

### server side

```python
from gevent.server import StreamServer
from mprpc import RPCServer

class FooServer(RPCServer):
    def func(self, x, y):
        return x + y

def main():
    server = StreamServer(("localhost", 3000), FooServer())
    server.serve_forever()

if __name__ == '__main__':
    main()
```

### client side

```python
from mprpc import RPCClient

def main():
    client = RPCClient("localhost", 3000)
    ans = client.call("func", 1, 2)
    print(ans)

if __name__ == '__main__':
    main()
```


