
#### add in dash

```sh
echo '[Desktop Entry]'$'\n''Version=1.0'$'\n''Type=Application'$'\n''Name=Eclipse'$'\n''Name[ja]=Eclipse'$'\n''Comment=Eclipse is an integrated development environment (IDE)'$'\n''Comment[ja]=統合開発環境'$'\n''Keywords=Java;java;IDE'$'\n''Keywords[ja]=Java;java;IDE;統合開発環境'$'\n''Exec=/opt/eclipse/eclipse'$'\n''Icon=/opt/eclipse/icon.xpm'$'\n''Terminal=false'$'\n''Categories=Java;Development;IDE;'$'\n''MimeType=text/x-chdr;text/x-csrc;text/x-c++hdr;text/x-c++src;text/x-java;text/x-dsrc;text/x-perl;text/x-python;application/x-php;application/x-httpd-php3;application/x-httpd-php4;application/x-httpd-php5;application/xml;text/html;text/css;text/x-sql;'$'\n''StartupNotify=true'$'\n''StartupWMClass=Eclipse'$'\n''Actions=Clean;'$'\n'''$'\n''[Desktop Action Clean]'$'\n''Name=eclipse -clean'$'\n''Exec=/opt/eclipse/eclipse -clean'$'\n''OnlyShowIn=Unity;'$'\n' | sudo tee /usr/share/applications/eclipse.desktop 
```


