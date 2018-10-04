SAM CLI
====


## install

```sh
$ pip install --upgrade aws-sam-cli
```

```sh
$ sam local start-api --help
```

## debug sample

### show sample

```sh
$ cat template.yaml
AWSTemplateFormatVersion: '2010-09-09'
Transform: AWS::Serverless-2016-10-31
Resources:
  MyApiFunction:
    Type: AWS::Serverless::Function
    Properties:
      CodeUri: .
      Handler: debugtest
      Runtime: go1.x
      Events:
        TestApi:
          Type: Api
          Properties:
            Path: '/test'
            Method: get
```

```go
$ cat main.go
package main

import (
	"context"
	"fmt"

	"github.com/aws/aws-lambda-go/events"
	"github.com/aws/aws-lambda-go/lambda"
)

func handleRequest(ctx context.Context, request events.APIGatewayProxyRequest) (events.APIGatewayProxyResponse, error) {
	fmt.Printf("Processing request data for request %s.\n", request.RequestContext.RequestID)
	fmt.Printf("Body size = %d.\n", len(request.Body))
	
	fmt.Println("Headers:")
	for key, value := range request.Headers {
		fmt.Printf("    %s: %s\n", key, value)
	}

	return events.APIGatewayProxyResponse{Body: "OK!", StatusCode: 200}, nil
}

func main() {
	lambda.Start(handleRequest)
}
```


### debugger

we use delve. delve is mount in docker.

```sh
GOARCH=amd64 GOOS=linux go build -o debug/dlv github.com/derekparker/delve/cmd/dlv
$ ls debug/
dlv
```

### build target

```sh
GOARCH=amd64 GOOS=linux go build -gcflags='-N -l' -o debugtest
$ ls debug/
dlv
```


### run sam local

```sh
$ sam local start-api -d 5986 --debugger-path
$ curl http://127.0.0.1:3000/test
```


