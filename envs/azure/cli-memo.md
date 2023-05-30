# Azure CLI

## install Azure CLI
* required: Python, libffi, OpenSSL

```sh
$ curl -L https://aka.ms/InstallAzureCli | bash
$ az --version
```

## install Azure DevOps CLI
```sh
$ az extesion add --name zure-devops
$ az extension list
```

## az login

### without browse
```sh
$ az login --use-device-code
$ az login --organization
```


# Azure Functions Core Tools

* see https://github.com/Azure/azure-functions-core-tools
