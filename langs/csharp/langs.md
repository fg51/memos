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

```csharp
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

```csharp
// use libnative.so
using IS = System.Runtime.InteropServices;

[IS.DllImport("native", EntryPoint = "add", CallingConvention = IS.CallingConvention.Cdecl, ExactSpelling = true)]
static extern void add(IntPtr xs, IntPtr ys);
//static extern void add(int[] xs, int[] ys);


// run native
{

  // protect GC
  System.IntPtr ptrx = IS.Marshal.AllocCoTaskMem(IS.Marshal.SizeOf(typeof(int)) * xs.Length);
  System.IntPtr ptry = IS.Marshal.AllocCoTaskMem(IS.Marshal.SizeOf(typeof(int)) * ys.Length);

  // managed to unmanaged
  IS.Marshal.Copy(xs, 0, ptrx, xs.Length); // xs -> ptrx

  add(ptrx, ptry);

  // unmanaged to managed
  IS.Marshal.Copy(ptry, ys, 0, ys.Length);  // ptry -> ys

  // release protect
  IS.Marshal.FreeCoTaskMem(ptrx);
  IS.Marshal.FreeCoTaskMem(ptry);
}
```

copy from managed array to unmanaged array

```csharp
[SecurityCriticalAttribute]
public static void Copy(
    int[] source, // source array
    int startIndex, // start index of source
    IntPtr dest, // destination pointer
    int length, // number
)
```

copy from unmanaged array to managed array

```csharp
[System.Security.SecurityCritical]
public static void Copy(
    IntPtr source, // source pointer
    int[] dest, // destination array
    int startIndex, // start index of destination
    int length, // number
)
```
