integration-testing: build

    env TMPDIR=/private$TMPDIR docker-compose -p $(PROJECT_NAME) up -d
    sleep 5s
    aws --endpoint-url=http://localhost:4572 s3 mb s3://bucket-example
    aws --endpoint-url=http://localhost:4572 s3 mb s3://bucket-example-convert
    aws --endpoint-url=http://localhost:4572 s3 cp ./testdata/example.json.gz s3://bucket-example/example.json.gz
    sam local invoke MainFunction --event event_file.json --template ./template/local.yaml \
    --docker-network $$(docker network ls -q -f name=$(PROJECT_NAME))
