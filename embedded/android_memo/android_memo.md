android-studio (with SDK)
========================================

[Installing the Android SDK | Android Developers](
    https://developer.android.com/sdk/installing/index.html?pkg=tools")


[ref1](http://blog.bgbgbg.net/archives/1690)

[ref2](http://saku-java.be-ourselves.jp/2014/03/12/mac-android-studio-start-new-project/)


## Compare with Eclipse

======================  ========================
Eclipse                 Android Studio
======================  ========================
Workspace               Project
Project                 Module
Project-specific JRE    Module JDK
User library            Global library
Classpath variable      Path variable
Project dependency      Module dependency
Library                 Module library
======================  ========================


## require

```sh
$ sudo dpkg --add-architecture i386
$ sudo apt-get update
$ sudo apt-get install libncurses5:i386 libstdc++6:i386 zlib1g:i386
```

```sh
$ sudo apt-get install openjdk-7-jdk
$ java -version
$ javac -version
```


## install

[download](https://developerandroid.com/sdk/installing/studio.html#download)

```
$ unzip android-studio-***.zip
$ sudo chown -R user android-studio
    必要であれば、ログインユーザーで実行できるよう権限つけておきます

$ which android
```


## execute

```sh
export PATH=”${ANDROID_STUDIO_PATH}/bin:$PATH”
$ ./${ANDROID_STUDIO_PATH}/bin/studio.sh
```
## Config (Setting)

(IDE Setting) Appearance > Theme: Darcula
File > Setting > Plugins > Browse repositories > IdeaVim


## Delete Project

[いいね&#65281;Androidアプリ: プロジェクトの削除&#12288;Android Studio](http://papakingyo-android.blogspot.jp/2014/05/android-studio_24.html))


###  del project list

[File] > [Reopen Project] > [Clear List]


### del project

.. [Open File] or [Open Project]

```sh
$ rm -rf $HOME/AndroidStudioProjects/<MyApp>
```

## run

```sh
$ studio.sh
```


## Project Tree

blue dir is source code
orange dir is GUI code (xml)
(if you select the xml file, 
    tab: Design is displayed the GUI.
    tab: Text   is displayed the src.)

    
res
    drawable
    layout
    menu
    values


:.idea:
Project Structure, Project Settings and others
(Android Studio (IntelliJ) recognize that the dir having the .idea is the android's project.)

:.iml:
Module parameters (a part of "Project Structure", "Run/Debug Configurations")
(iml = Intellij ModuLe ?)

:.gradle:

:<PROJECT_HOME>/build.gradle: That is blank.

<PROJECT_HOME>/settings.gradle
.. code_block:: groovy
    include ':<MyApp>'

<PROJECT_HOME>/<MyApp>/build.gradle
    main build script

(using Gradle Plugin)



### Project Structure

:Project: common setting. SDK path, build path, and so on.
:Module: src path, build path, reference library, reference module.
:Libraries: set project libraries, global libraries
:Facets: other languages support (except JAVA)
:Artifacts: builded data (.jar, .apk)



### Set Project

for main setting
File > Project Structure

for other setting
"File > Settings" or "Preferences(Settings) button"

<PROJECT_HOME>
    - build.gradle      ... MyFirstAppProject.iml
    - settings.gradle
    - MyFirstApp/
       - build.gradle   ... MyFirstApp.iml


If delete the iml,
File > Import Module... > <PROJECT_HOME>/build.gradle



## sample

### ref

http://gihyo.jp/dev/serial/01/android_studio/0003?page=3

### summary

* 新しいレイアウトactivity_second.xmlを作成する（レイアウトにはテキストラベルとボタンを配置する）。
* 新しいレイアウト用にアクティビティSecondActivity.javaを作成し，処理を記述する
* 既存のレイアウトactivity_main.xmlにテキストフィールドとボタンを追加する
* 既存のアクティビティMainActivity.javaを修正してボタンを押された処理を追加する


### create

src/main/res/layout/activity.xml <-> src/main/java/com/<name>/<MyApp>/activity.java


### activity.xml

select <MyAppProject>/<MyApp>/src/main/res/layout
File -> New...
select "Layout resource file"
created new activity xml

:file name : activity_second
:Root element: LinearLayout


### activity.java

create from "Android Component"

======  ====================
項目    入力した値
======  ====================
Name    SecondActivity
Kind    Activity
Label   @string/app_name
======  ====================


### SecondActivity.java

```java
package com.example.myfirstapp;

import android.app.Activity;
import android.content.Intent;
import android.os.Bundle;
import android.view.View;
import android.widget.TextView;

import static android.view.View.OnClickListener;

public class SecondActivity extends Activity {
  public void onCreate(Bundle savedInstanceState) {
    super.onCreate(savedInstanceState);
    setContentView(R.layout.activity_second);

    TextView textView = (TextView) findViewById(R.id.textView);
    textView.setText(getIntent().getStringExtra("inputText"));

    findViewById(R.id.button).setOnClickListener(new OnClickListener() {
      @Override
      public void onClick(View view) {
        Intent intent = new Intent(SecondActivity.this, MainActivity.class);
        startActivity(intent);
      } //END of public void onClick(View view) {
    }); //END of findViewById(R.id.button).setOnClickListener( 
  } //END of public void onCreate(
}   //END of public class SecondActivity extends Activity {
```
