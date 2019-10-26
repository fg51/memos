# path

* http://s3-ap-northeast-1.amazonaws.com/<name>/<file.zip>
* http://<name>.s3-ap-northeast-1.amazonaws.com/<file.zip>

## rule
* バケットは3-63文字
* バケットは(一文字以上の)小文字、数字、”.”(ドット), “-“(ハイフン)
* バケットは”.”で始まる、終わってはいけない、”-“前後は”.”をつなげてはいけない
* キーは適切にUTF-8にエンコードされたものであればよい。
* キーの文字列”+”はURLエンコードする
