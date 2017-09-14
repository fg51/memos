
NDK: Native Development Kit
========================================

applied C/C++

[natsu ken puro](http://c-loft.com/blog/?p=1882)

[download](https://developer.android.com/tools/sdk/ndk/index.html#Installing)


## install

```sh
$ chmod a+x android-ndk-r10c-darwin-x86_64.bin
$ ./android-ndk-r10c-darwin-x86_64.bin
/android-ndk-r10c-darwin-x86_64/
```


## usage

```sh
$ mkdir jni
```


```java: app/build.gradle
android {
    defaultConfig {
        ...

        ndk {
            moduleName "calcvalues"
            stl "gnustl_shared" //NOTE: shared library
            abiFilters "armeabi", "armeabi-v7a", "x86", "mips" //NOTE: CPU arch
            //ldLibs "log"
        }
    }
}
```

### build

when doing Gradle, execute ndk-build .
(does not execute ndk-build doing build only)

created in "app/build/intermediates/ndk/debug/lib/"

```sh
$ cp -rf app/build/intermediates/ndk/debug/lib/ app/src/main/jniLibs/
```


```java: app/build.gradle
android {
    defaultConfig { }

    sourceSets {
        main {
            jni.srcDirs = [] //NOTE: ndk-buildを毎回コールするのを避ける
        }
    }
}
```

## build the stand alone binary on the shell

[ref](http://cubeundcube.blogspot.jp/2013/06/androidcc.html)


```sh
$ mkdir -p ndkwork/hellondk-jni
ndkwork $ cat ./jni/hellondk.c
```

ndkwork - hellondk - jni - hellondk.c   (src file)
                   |     - Android.mk   (make file)
                   - libs - armeabi - hellondk (dest file)
                   - obj  - <obj files>

### create src file

NOTE: use LF

```sh
$ cat ./jni/hello.c
```

```cpp
#include <stdio.h>
int main() {
    printf("Hello, NDK! \n");
    return 0;
}
```


### create mk file

NOTE: use LF

```sh
$ cat ./jni/Android.mk
LOCAK_PATH := $(call my-dir)
include $(CLEAR_VARS)

LOCAL_CFLAGS += -std=c99
LOCAL_MODULE    := <dest file name>
LOCAL_SRC_FILES := <src file name>

include $(BUILD_EXECUTABLE)
```

### build

```sh
hellondk $ ndk-build
```


### execute

```sh
$ cd libs/armeabi
$ adb push <dest> /data/local/tmp #????
$ adb shell
shell@androoid:/ $ cd /data/local/tmp
shell@androoid:/data/local/tmp $ chmod 755 <dest>
shell@androoid:/data/local/tmp $ ./<dest>
```


