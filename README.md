# hyper-service-custom
Experimental web service using hyper framework

This is an sample web service using hyper framework and a custom router. 

Supports the following calls.

* `GET /test`: Dummy request / response
* `POST /send`: Sample JSON parser
* `GET /params/:param`: Samplme parameter parser

## Run with docker

```shell
docker run -p 3001:3001 bbarekas/hyper-service
```

## Next steps:
* Add more routing cases
* Add logging
* Run benchmark
* Random staff

