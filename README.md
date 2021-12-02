# Kanidm client for Python

**Proof of concept**

It just wraps the Rust crate kanidm_client.

Shows that you can:

1. Call to the library from python code.
2. Hook up rust logging to python logging. (Kanidm has good logging so this is quite useful)
3. Initialize the KanidmClient type.
4. Make a whoami call.

A couple of alternatives might be possible:

* Hand write Python REST client
* Write a Rust client using native Python/CPython http libraries.
* Write a Swagger, OpenAPI or other spec for Kanidm's API and generate a client.

I attempted this solution simply to learn more about Python <-> Rust bindings.
