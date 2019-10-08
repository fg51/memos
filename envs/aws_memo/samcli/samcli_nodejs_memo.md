# sam with nodejs

```sh
$ sam init --runtime nodejs
$ cd sam_app  # ROOT

$ cd hello-world
$ npm install
$ npm test

$ cd ..  # to ROOT
# create mock-event: api-gateway-event
$ sam local generate-event apigateway aws-proxy > event_file.json
# for < 0.5.0 $ sam local generate-event api > event_file.json
$ sam local invoke HelloWorldFunction --event event_file.json
Error: Could not find lambci/lambda:nodejs10.x image locally and failed to pull it from docker.
$ docker pull lambci/lambda:nodejs10.x

$ sam local start-api
```
