# AWS CDK v2

aws sts get-caller-identity
$(aws sts get-caller-identity | jq -r .Account)

## install
```sh
$ npm i -D aws-cdk
$ npx cdk --version
$ npm i -D esbuild  # without docker for node.js?
```

## init
### python
```sh
$ cdk init --language python
```
TODO: from requirements.txt to pyproject.toml

### typescript
```sh
$ cdk init --language typescript 
```

## usage
```sh
$ cdk synth  # create cloudformathion's template
$ cdk bootstrap # create s3 bucket
$ cdk bootstrap aws://your-accountid/your-region-code
$ cdk deploy # deploy
$ cdk diff  # diff local code, deployed code
$ cdk destroy # delete stack (not delete s3-bucket.)
```

see cloudFormathion console

# constructs
L1 CfnXyz
L2 (default)
L3
