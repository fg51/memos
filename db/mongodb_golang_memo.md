## install

```sh
$ go get gopkg.in/mgo.v2
```

## sample

```go
package main

import (
	"fmt"
	"log"
)

import (
	mgo "gopkg.in/mgo.v2"
	"gopkg.in/mgo.v2/bson"
)

type Fruit struct {
	ID   bson.ObjectId `bson:"_id"`
	Name string        `bson:"name"`
	Price int           `bson:"price"`
}

func main() {
	session, err := mgo.Dial("mongodb://localhost/study")
	if err != nil {
		log.Fatalln(err)
	}
	defer session.Close()

	db := session.DB("test")
	apple := &Fruit {
		ID: bson.NewObjectId(),
		Name: "apple",
		Price: 100,
	}

	const CollectionName = "fruits"
	collection := db.C(CollectionName)
	if err := collection.Insert(apple); err != nil {
		log.Fatalln(err)
	}

	f := new(Fruit)
	query := db.C(CollectionName).Find(bson.M{})
	query.One(&f)

		fmt.Printf("%+v\n", f)
}
```
