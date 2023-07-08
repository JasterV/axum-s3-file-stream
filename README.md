# axum-s3-file-stream

An example of how to stream a file from S3 down to an http client on axum

## How to try it out

1. Spin up `minio` by executing `docker compose up`, or `docker compose up -d` if you want to keep using the same terminal
2. Open the browser, go to `localhost:9001` and login into the `minio` interface with the credentials you find in the `compose` file
3. Create a bucket and upload some files
4. Update the `BUCKET` env variable in the Makefile to match the name of the one you've created
5. Now you can run the app by executing `cargo make run` or `cargo make run-watch` if you want it to re-build an run real-time when updating the code.
