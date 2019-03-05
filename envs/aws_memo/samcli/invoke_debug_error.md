```sh
2019-03-05 23:21:07 Using SAM Template at /home/flamefly/work/aws-sample/sam-app-node/template.yaml
2019-03-05 23:21:07 Changing event name from creating-client-class.iot-data to creating-client-class.iot-data-plane
2019-03-05 23:21:07 Changing event name from before-call.apigateway to before-call.api-gateway
2019-03-05 23:21:07 Changing event name from request-created.machinelearning.Predict to request-created.machine-learning.Predict
2019-03-05 23:21:07 Changing event name from before-parameter-build.autoscaling.CreateLaunchConfiguration to before-parameter-build.auto-scaling.CreateLaunchConfiguration
2019-03-05 23:21:07 Changing event name from before-parameter-build.route53 to before-parameter-build.route-53
2019-03-05 23:21:07 Changing event name from request-created.cloudsearchdomain.Search to request-created.cloudsearch-domain.Search
2019-03-05 23:21:07 Changing event name from docs.*.autoscaling.CreateLaunchConfiguration.complete-section to docs.*.auto-scaling.CreateLaunchConfiguration.complete-section
2019-03-05 23:21:07 Changing event name from before-parameter-build.logs.CreateExportTask to before-parameter-build.cloudwatch-logs.CreateExportTask
2019-03-05 23:21:07 Changing event name from docs.*.logs.CreateExportTask.complete-section to docs.*.cloudwatch-logs.CreateExportTask.complete-section
2019-03-05 23:21:07 Changing event name from before-parameter-build.cloudsearchdomain.Search to before-parameter-build.cloudsearch-domain.Search
2019-03-05 23:21:07 Changing event name from docs.*.cloudsearchdomain.Search.complete-section to docs.*.cloudsearch-domain.Search.complete-section
2019-03-05 23:21:07 Changing event name from creating-client-class.iot-data to creating-client-class.iot-data-plane
2019-03-05 23:21:07 Changing event name from before-call.apigateway to before-call.api-gateway
2019-03-05 23:21:07 Changing event name from request-created.machinelearning.Predict to request-created.machine-learning.Predict
2019-03-05 23:21:07 Changing event name from before-parameter-build.autoscaling.CreateLaunchConfiguration to before-parameter-build.auto-scaling.CreateLaunchConfiguration
2019-03-05 23:21:07 Changing event name from before-parameter-build.route53 to before-parameter-build.route-53
2019-03-05 23:21:07 Changing event name from request-created.cloudsearchdomain.Search to request-created.cloudsearch-domain.Search
2019-03-05 23:21:07 Changing event name from docs.*.autoscaling.CreateLaunchConfiguration.complete-section to docs.*.auto-scaling.CreateLaunchConfiguration.complete-section
2019-03-05 23:21:07 Changing event name from before-parameter-build.logs.CreateExportTask to before-parameter-build.cloudwatch-logs.CreateExportTask
2019-03-05 23:21:07 Changing event name from docs.*.logs.CreateExportTask.complete-section to docs.*.cloudwatch-logs.CreateExportTask.complete-section
2019-03-05 23:21:07 Changing event name from before-parameter-build.cloudsearchdomain.Search to before-parameter-build.cloudsearch-domain.Search
2019-03-05 23:21:07 Changing event name from docs.*.cloudsearchdomain.Search.complete-section to docs.*.cloudsearch-domain.Search.complete-section
2019-03-05 23:21:07 local invoke command is called
2019-03-05 23:21:07 No Parameters detected in the template
2019-03-05 23:21:07 2 resources found in the template
2019-03-05 23:21:07 Found Serverless function with name='HelloWorldFunction' and CodeUri='hello-world/'
2019-03-05 23:21:07 Trying paths: ['/home/flamefly/.docker/config.json', '/home/flamefly/.dockercfg']
2019-03-05 23:21:07 No config file found
2019-03-05 23:21:07 Trying paths: ['/home/flamefly/.docker/config.json', '/home/flamefly/.dockercfg']
2019-03-05 23:21:07 No config file found
2019-03-05 23:21:07 Converted retries value: Retry(total=0, connect=None, read=False, redirect=None, status=None) -> Retry(total=Retry(total=0, connect=None, read=False, redirect=None, status=None), connect=None, read=None, redirect=0, status=None)
2019-03-05 23:21:07 Connection pool is full, discarding connection: localhost
Traceback (most recent call last):
  File "/home/flamefly/.anyenv/envs/pyenv/versions/neovim3/lib/python3.6/site-packages/requests/packages/urllib3/util/timeout.py", line 124, in _validate_timeout
    float(value)
TypeError: float() argument must be a string or a number, not 'Timeout'

During handling of the above exception, another exception occurred:
```

```python
Traceback (most recent call last):
  File "/home/flamefly/.anyenv/envs/pyenv/versions/neovim3/bin/sam", line 10, in <module>
    sys.exit(cli())
  File "/home/flamefly/.anyenv/envs/pyenv/versions/neovim3/lib/python3.6/site-packages/click/core.py", line 722, in __call__
    return self.main(*args, **kwargs)
  File "/home/flamefly/.anyenv/envs/pyenv/versions/neovim3/lib/python3.6/site-packages/click/core.py", line 697, in main
    rv = self.invoke(ctx)
  File "/home/flamefly/.anyenv/envs/pyenv/versions/neovim3/lib/python3.6/site-packages/click/core.py", line 1066, in invoke
    return _process_result(sub_ctx.command.invoke(sub_ctx))
  File "/home/flamefly/.anyenv/envs/pyenv/versions/neovim3/lib/python3.6/site-packages/click/core.py", line 1066, in invoke
    return _process_result(sub_ctx.command.invoke(sub_ctx))
  File "/home/flamefly/.anyenv/envs/pyenv/versions/neovim3/lib/python3.6/site-packages/click/core.py", line 895, in invoke
    return ctx.invoke(self.callback, **ctx.params)
  File "/home/flamefly/.anyenv/envs/pyenv/versions/neovim3/lib/python3.6/site-packages/click/core.py", line 535, in invoke
    return callback(*args, **kwargs)
  File "/home/flamefly/.anyenv/envs/pyenv/versions/neovim3/lib/python3.6/site-packages/click/decorators.py", line 64, in new_func
    return ctx.invoke(f, obj, *args[1:], **kwargs)
  File "/home/flamefly/.anyenv/envs/pyenv/versions/neovim3/lib/python3.6/site-packages/click/core.py", line 535, in invoke
    return callback(*args, **kwargs)
  File "/home/flamefly/.anyenv/envs/pyenv/versions/neovim3/lib/python3.6/site-packages/samcli/commands/local/invoke/cli.py", line 56, in cli
    parameter_overrides)  # pragma: no cover
  File "/home/flamefly/.anyenv/envs/pyenv/versions/neovim3/lib/python3.6/site-packages/samcli/commands/local/invoke/cli.py", line 93, in do_cli
    aws_region=ctx.region) as context:
  File "/home/flamefly/.anyenv/envs/pyenv/versions/neovim3/lib/python3.6/site-packages/samcli/commands/local/cli_common/invoke_context.py", line 142, in __enter__
    if not self._container_manager.is_docker_reachable:
  File "/home/flamefly/.anyenv/envs/pyenv/versions/neovim3/lib/python3.6/site-packages/samcli/local/docker/manager.py", line 50, in is_docker_reachable
    self.docker_client.ping()
  File "/home/flamefly/.anyenv/envs/pyenv/versions/neovim3/lib/python3.6/site-packages/docker/client.py", line 187, in ping
    return self.api.ping(*args, **kwargs)
  File "/home/flamefly/.anyenv/envs/pyenv/versions/neovim3/lib/python3.6/site-packages/docker/api/daemon.py", line 174, in ping
    b = self._get(a)
  File "/home/flamefly/.anyenv/envs/pyenv/versions/neovim3/lib/python3.6/site-packages/docker/utils/decorators.py", line 46, in inner
    return f(self, *args, **kwargs)
  File "/home/flamefly/.anyenv/envs/pyenv/versions/neovim3/lib/python3.6/site-packages/docker/api/client.py", line 198, in _get
    return self.get(url, **self._set_request_timeout(kwargs))
  File "/home/flamefly/.anyenv/envs/pyenv/versions/neovim3/lib/python3.6/site-packages/requests/sessions.py", line 546, in get
    return self.request('GET', url, **kwargs)
  File "/home/flamefly/.anyenv/envs/pyenv/versions/neovim3/lib/python3.6/site-packages/requests/sessions.py", line 533, in request
    resp = self.send(prep, **send_kwargs)
  File "/home/flamefly/.anyenv/envs/pyenv/versions/neovim3/lib/python3.6/site-packages/requests/sessions.py", line 646, in send
    r = adapter.send(request, **kwargs)
  File "/home/flamefly/.anyenv/envs/pyenv/versions/neovim3/lib/python3.6/site-packages/requests/adapters.py", line 449, in send
    timeout=timeout
  File "/home/flamefly/.anyenv/envs/pyenv/versions/neovim3/lib/python3.6/site-packages/requests/packages/urllib3/connectionpool.py", line 587, in urlopen
    timeout_obj = self._get_timeout(timeout)
  File "/home/flamefly/.anyenv/envs/pyenv/versions/neovim3/lib/python3.6/site-packages/requests/packages/urllib3/connectionpool.py", line 302, in _get_timeout
    return Timeout.from_float(timeout)
  File "/home/flamefly/.anyenv/envs/pyenv/versions/neovim3/lib/python3.6/site-packages/requests/packages/urllib3/util/timeout.py", line 154, in from_float
    return Timeout(read=timeout, connect=timeout)
  File "/home/flamefly/.anyenv/envs/pyenv/versions/neovim3/lib/python3.6/site-packages/requests/packages/urllib3/util/timeout.py", line 94, in __init__
    self._connect = self._validate_timeout(connect, 'connect')
  File "/home/flamefly/.anyenv/envs/pyenv/versions/neovim3/lib/python3.6/site-packages/requests/packages/urllib3/util/timeout.py", line 127, in _validate_timeout
    "int, float or None." % (name, value))
ValueError: Timeout value connect was Timeout(connect=60, read=60, total=None), but it must be an int, float or None.
```
