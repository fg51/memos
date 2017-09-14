Command line samples
====

## AVD & ADB

```sh
$ android list avds

shown <avd name>
$ emulator @<avd name>  # execute emulator
$ adb shell             # connect to emulator

# Show the http Browser (Action_view + URL)
shell@android:/ $ am start -a android.internet.action.VIEW -d http://google.com

# execute Activity (class)
shell@android:/ $ am start -n com.Foo.app/.BarActivity
```
