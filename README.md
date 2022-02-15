# rust-lambda

A simple lambda written in Rust deployed to AWS.

Since Rust does not have an official Lambda Runtime, it needs to run as a custom runtime. A number of things need to
be taken into account in order to successfully deploy.

- Rust build target needs to be configured to a linux target - `x86_64-unknown-linux-musl`
- AWS is expecting an binary executable named `bootstrap`
- Code asset is a zip file with the single bootstrap binary
- Lambda runtime is set to `PROVIDED.AL2` for Amazon Linux 2

Note: In order to build locally, please have Nightly Rust installd.
