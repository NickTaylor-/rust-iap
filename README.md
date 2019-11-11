### Rust IAP (Identity-Aware Proxy)
This is a simple Nginx based Identity-Aware Proxy (IAP) with the authorization engine written in Rust.

#### How to use
Right now this is very much just demo code, however it can be used as follows:

1. `skaffold dev`
2. Get a token from `http://rust-iap.localtest.me/token`
3. Send a request to `http://iap-test.localtest.me/` with the token you generated before as the `Token` header:
    * `curl -v http://iap-test.localtest.me/ -H 'Token: <MY_TOKEN>`
4. You should see a proxied request to http://www.example.org.

If you remove the token, or modify it, it will no longer be valid and you will receive a 401 Unauthorized response.