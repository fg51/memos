# install

```sh
$ az bicep upgrade
```

## sample

```bicep
// region
param location string = resourceGroup().location

// storage account
param stName string = 'st${uniqueString(resourceGroup().id)}'

// strage account
resource st 'Microsoft.Storage/storageAccount@2021-04-01' = {
  name: stName
  location: location
  kind: 'StorageV2'
  sku: { name: 'Standard_LRS' }
}

```

## deploy

```sh
$ az deployment group create \
    -n {deploy name}  \
    -f {bicep filename}  \
    -g {resource group name}  \
    -p {parameter name} = {parameter value}
```
