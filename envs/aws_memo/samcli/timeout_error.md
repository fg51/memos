## summary
docker pull するとき, timeout の型エラーが発生する。

"./requests/packages/urllib3/connectionpool.py", line 302, in _get_timeout
if isinstance(timeout, Timeout)  が機能していない。

##
<python path>/site-packages/requests/packages/urllib3/connectionpool.py", line 302, in _get_timeout

    def _get_timeout(self, timeout):
        """ Helper that always returns a :class:`urllib3.util.Timeout` """
        if timeout is _Default:
            return self.timeout.clone()

        if isinstance(timeout, Timeout):
            return timeout.clone()
        else:
            # User passed us an int/float. This is for backwards compatibility,
            # can be removed later
            return Timeout.from_float(timeout)

if isinstance(timeout, Timeout)  が機能していない。
timeout: requests.packages.urllib3.util.timeout.Timeout
Timeout: type


/home/flamefly/.anyenv/envs/pyenv/versions/anaconda3-4.4.0/envs/neovim3/lib/python3.6/site-packages/samcli

1. error at invoke_context.py line at 142

```python
self._container_manager.is_docker_reachable:
```


3. no pint at manager.py

```python
    @property
    def is_docker_reachable(self):
        """
        Checks if Docker daemon is running. This is required for us to invoke the function locally

        Returns
        -------
        bool
            True, if Docker is available, False otherwise
        """
        try:
            self.docker_client.ping()

            return True

        # When Docker is not installed, a request.exceptions.ConnectionError is thrown.
        except (docker.errors.APIError, requests.exceptions.ConnectionError):
            LOG.debug("Docker is not reachable", exc_info=True)
            return False
```

## 3. error at manager.py

```python
def __init__(self,
             docker_network_id=None,
             docker_client=None,
             skip_pull_image=False):
    """
    Instantiate the container manager

    :param docker_network_id: Optional Docker network to run this container in.
    :param docker_client: Optional docker client object
    :param bool skip_pull_image: Should we pull new Docker container image?
    """

    self.skip_pull_image = skip_pull_image
    self.docker_network_id = docker_network_id

    self.docker_client = docker_client or docker.from_env()
```

docker_client is None


## 4. in site-packages/docker/daemon.py
self._url('/_ping')
http+docker://localhost/v1.35/_ping


## 5. site-packages/requests/sessions.py
url: http+docker://localhost/v1.35/_ping

546: resp = self.send(prep, **send_kwargs)
prep:
<PreparedRequest [GET]>

646: r = adapter.send(request, **kwargs)
