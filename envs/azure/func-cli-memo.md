
```sh
$ func init ${function-name}
```

## in node javascript
* create index.js as the entry point.
* create function.json as config.

proj
  - host.json
  - local.settings.json
  - package.json
  - examples
    - function.json
    - index.js


## ex.
```json
{
  "bindings": [
    {
      "authLevel": "function", 
      "type": "httpTrigger", 
      "direction": "in", 
      "name": "req", 
      "methods": [
        "get", 
        "post"
      ]
    }, 
    {
      "type": "http", 
      "direction": "out",
      "name": "res"
    }
  ]
}

```

```javascript
'use strict';
const moment = require('moment');

module.exports = async function (context, req) {
  context.log('javascript http trigger function processed a request.');
  const responseMessage = "Azure CLI : " + moment().utcOffset(9).format('YYYY-MM-DD HH:mm:ss');
  context.res = {
    // status: 200,  /* Defaults to 200 */
    body: responseMessage
  };
}
```


# deploy

## create resource, storage
```sh
$ az group create --name ${RESOURCE_GROUP_NAME}  --location ${REGION}
$ az storage account create --name ${STORAGE_NAME}  --location ${REGION} --resource-group ${RESOURCE_GROUP_NAME}
```

## create function in storage
```sh
$ az functionapp create \
  --resource-group ${RESOURCE_GROUP_NAME}
  --consumption-plan-location ${REGION}
  --runtime node
  --runtime-version 18 \
  --function-version 4 \
  --name ${APP_NAME} \
  --storage-account ${STORAGE_NAME}
```

## deploy
```sh
$ func azure functionapp publish <FUNCTION_NAME>
```


