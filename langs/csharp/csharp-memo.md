# csharp build

require dotnet core

## create project

```sh
$ dotnet new console -o ${project name}
```

```sh
$ dotnet new wpf -o ${project name}
```

### for linux

```xhtml@WpfApp.csproj
<PropertyGroup>
- <TargetFramework>net7.0@windows</TargetFramework>
+ <TargetFramework>net7.0</TargetFramework>
+ <EnableWindowsTargeting>true</EnableWindowsTargeting>
</PropertyGroup>
```

## build

```sh
$ dotnet run
```

```sh
$ dotnet build --debug
$ dotnet build --configuration Release
```

## call other lang

```sh
clang++ -shared -fPIC -fdeclspec -o build/libnativelib.so src/nativelib.cpp
```

### project

```
<PropertyGroup>
  <AllowUnsafeBlocks>true</AllowUnsafeBlocks>
</PropertyGroup>

<ItemGroup>
  <None Include="libnativelib.so" Condition="Exists('libnativelib.so')">
    <CopyToOutputDirectory>Always</CopyToOutputDirectory>
  </None>
</ItemGroup>
```

### source

```csharp
using IS = System.Runtime.InteropServices;

namespace CsBindgen
{
  internal static unsafe partial class NativeMethods
  {
    const string __DllName = "nativelib";

    [IS.DllImport(__DllName, EntryPoint = "add", CallingConvention = IS.CallingConvention.Cdecl, ExactSpelling = true)]
    public static extern int add(int left, int right);

  }
}
```

}

```cpp
#ifdef DLL_EXPORTS
#define DLL_EXPORT __declspec(dllexport)
#else
#define DLL_EXPORT
endif

#ifdef DLL_IMPORTS
#define DLL_IMPORT __declspec(dllimport)
#else
#define DLL_IMPORT
endif

namespace Native
{
    DLL_EXPORT int add(int x, int y);

    DLL_EXPORT int add(int x, int y) {
        return x + y;
    }
}
```

```cpp
useing namespace System

namespace Wrapper {
    public ref class WrpaeerClass
    {
        public: void Max(array<int>^ src, int num, int% mx, int% mxIndex);

    };

    void Wrapper::WrapperClass::Max(array<int>^ src, int num, int% mx, int% mxIndex)
    {
        // protect GC
        pin_ptr<int> psrc = &src[0];
        pin_ptr<int> pmx = &mx;
        pin_ptr<int> pMxIndex = &mxIndex;

        // run native
        Native::Max(psrc, num, pmx, pMxIndex);

        // release protected
        psrc = nullptr;
        pmx = nullptr;
        pMxIndex = nullptr;
    }
}
```
