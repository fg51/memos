# csharp langs

## access modifiers

### class

- public : call from all
- internal : call from assembly
- abstract : abstract member
- static : static member
- sealed : cannot inheritance

default is internal

#### others

- protected
- protected internal
- private protected
- private

```csharp
namespace SelfCharp.Chapter  // namespace
{
    internal class Person
    {
        public string Name  // field

        public Person(string Name) {  // constructor
            ...
        }

        public string Name  // property
        {

        }

        public void Show(  // method
        {

        }
    }

}
```

### field

- public field : Pascal
- private field : camelCase

- public
- protected internal
- protected
- internal
- private protected
- private
- readonly
- static : class field
- new : inherited class
- volatile

## string format

```csharp
$"my name is {xyz}"
```

## constructor

```csharp
class Person
{
    string Name;

    public Person(string name)
    {
        this.Name = name;
    }

    public Person(): this("apple") { // call other constructor before Person()}
}
```

###

initialize field -> initialize constructor -> initialize object initilizer

### finalizer (destructor)

```csharp
~Person()
{

}
```

#### Dispose

```charp
class Person : IDisposable
    public void Dispose()
    {

    }
```

#### static method (class method)

```charp
public class Name {
    public static string Name = "apple" // class field
    public static GetName() {  // class method


    }
}

Name.GetName();  // cnanot call this
```

## const / readonly field

- local varialbe : readonly : not, const : enable
- initialize : readonly : declared / constructor, const : declared
- class member : readonly: static only, constructor : class member
- timming : readonly : at runtime, const : at complie

## static constructor

initialize class field.

## pointer

method(ref int data)

- in : readonly ref

## record

```csharp
public record class User
{
    public int Id { get; } // {get; init; }
    public string FirstName { get; }
    public string LastName { get; }

    // => is lambda
    public User(int id, string first, string last) => (Id, FirstName, LastName) = (id, first, last);
}
```
