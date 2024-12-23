# Azure CLI

## install Azure CLI

- required: Python, libffi, OpenSSL

```sh
$ pacman -S azure-cli
$ az --version
```

```sh
$ curl -L https://aka.ms/InstallAzureCli | bash
$ az --version
```

or cat install.sh and run python

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
$ az account show --query user.name
```

### auto install extension

```sh
$ az config set extension.use_dynamic_install=yes_without_prompt
$ az config get
```

# Azure Functions Core Tools

- see https://github.com/Azure/azure-functions-core-tools
