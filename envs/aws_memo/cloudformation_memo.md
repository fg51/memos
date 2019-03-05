cloudformation
====

## require

cloudformation:ValidateTemplate


## usage

```sh
$ aws cloudformation validate-template --template-body file://{target}.yaml
```


## detail

* Format Version
* Description
* Metadata
* Parameters
* Mappings
* Resources (must)
* Outputs


```yaml
AWSTemplateFormatVersion: '2010-09-09'
Resources:
  <Logical ID>:               // FirstVPC:
    Type: <Resource type>     // AWS::EC2::VPC
    Properties:
      <Set of properties...>  // CidrBlock: 10.0.0.0/16
```


## embedded function

### !Ref
