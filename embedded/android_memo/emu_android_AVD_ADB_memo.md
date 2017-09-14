
ADB: Android Debug? Bridge

AVD: Android Virtual Device (emulator)


Emulator Android emulator
http://www.techdoctranslator.com/android/developing/tools/emulator


AVD (Android Virtual Device)
========================================

[Android Virtual Device](http://ichitcltk.hustle.ne.jp/gudon2/index.php?pageType=file&id=Android002_Emulator)

## show avd list

```sh
$ android list avds
$ which android
${HOME}/Android/Sdk/tools/android
```


## create avd
```sh
$ android list targets

$ android create avd -n <avd name> -t <target ID> (-c SD card memory size)
or $ android create @<avd name> 
```


## delete
```sh
$ android delete avd -n <avd name>
```


##emulator

emulator is QEMU base.

```sh
$ emulator @<avd name>
```


ADB Android Debug Bridge
========================================

[Android よく使うadbのコマンド Qiita](http://qiita.com/t2low/items/cb37cec5f864c4748e14)


what is the command: am, pm ?  
## connect to the emulator

```sh
$ adb shell     #NOTE: login debug-shell

or $ adb shell <command>    #NOTE: not login debug-shell
```


## input the string

```sh
$ input text <strings>
```


send key-event
----------------------------------------
[ref](http://developer.android.com/reference/android/view/KeyEvent.html)

```sh
adb@android:/ $ input keyevent 3  # HOME
adb@android:/ $ input keyevent 4  # BACK
adb@android:/ $ input keyevent 82 # MENU
```


show package
----------------------------------------
.. sourcecode:: sh
    adb@android:/ $ pm list packages

cat
----------------------------------------
.. sourcecode:: sh
    adb@android:/ $ cat <file_path>


