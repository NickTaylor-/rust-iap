### Rust IAP (Identity-Aware Proxy)
This is a simple Nginx based Identity-Aware Proxy (IAP) with the authorization engine written in Rust.

#### How to use
Right now this is very much just demo code, however it can be used as follows:

1. `docker-compose up`
2. Get a token from `http://localhost:8000/token`
3. Send a request to `http://localhost:8000` with the token you generated before as the `Token` header:
    * `curl -v http://localhost:8000 -H 'Token: <MY_TOKEN>`
4. You should see a proxied request to http://www.example.org.

If you remove the token, or modify it, it will no longer be valid and you will receive a 401 Unauthorized response.