# auth

when the app outside azure access in azure resource, the app authed via service principal

when app in azure accecess resource in azure, the app authed via managed-id.

# example C# project

```sh
$ dotnet add package Azure.Identitiy
```

```csharp
using Azure.Identity
var credential = new DefaultAzureCredential();
```

# role

- privileged administrator roles
  - Owner
  - Contributor
  - User Access Administrator
- job function roles
  - reader
  - storage blob data contributor
  - storage blob data reader
  - key vault secrets officer
  - key vault secrets user
