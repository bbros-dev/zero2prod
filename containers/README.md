# Rust uService and Container: Size Baselines

There is a suggestion to stop using musl and Alpine images: [Why does musl make my Rust code so slow?](https://andygrove.io/2020/05/why-musl-extremely-slow/).
As a guard on that we also report container sizes using `slim-buster` and its gcc build chain.  We think it prudent to use the build chain that is part of a distribution - for the reason that it programs compiled with it are likely subjected to more stress testing - per Andy Grove's observation.

https://github.com/rousan/rust-web-frameworks-benchmark/tree/master/actix

https://github.com/the-benchmarker/web-frameworks
https://github.com/flosse/rust-web-framework-comparison#low-level-frameworks

## The point

- What size does a minimal Rust binary look like across HTTP/Web frameworks?
- Is it worth further investigation to see if the size differential enough to fit the code of a (our/your) micro-service?
- Is the size differential worth investigating further?
- What size does a minimal container holding that binary look like?
- What are build times like?
- What containerized build environments are available, "off-the shelf"?

## Why

All decisions involve trade-offs. We measure one tradeoff - size.  You may just be curios and if we satisfied that curiosity, that, in our view, is sufficient reason to justify the exercise.
This may stimulate you to dig further, even better, hopefully we saved you the effort of establishing a baseline.
As time passes you will want to update your baseline, hopefully the repository we setup should save you some effort you can devote elsewhere.

Our conjecture was that some frameworks might be sufficiently larger that we could conceivably fit the code for a microservice in the size differential.
Example: The A crate results in a 8MB executable.  The B crate results in a 15MB executable.
If you can fit the code for your micro-service inside that 7MB (`rg` is 5.5MB), the A vs. B trade-offs might change.

Of course these are not the only trade-offs.
Likewise we don't suggest they are the more important.

## Limitations

We only measure the baseline size of a micro-service - "Hello world!".

## Results

## Conclusions

We will investigate whether we can fit some micro-service code within nMBi.
We will/will not add micro-server size to the list of trade-offs to consider when assessing Rust HTTP server crates.

We won't be using MUSL builds in future - for us the tradeoffs aren't worth it but that is purely subjective.
We don't claim nor suggest anyone else should avoid musl built static binaries - e.g. ripgrep distributes them and we use that quite happily.

The BusyBox and ToyBox build environments are worth keeping an eye on.
It may be worth you while getting them to work?

## Details

To avoid rebuilding dependencies whenever or app changes, we [cache dependencies](https://shaneutt.com/blog/rust-fast-small-docker-image-builds/)

* Actix-Web, port `3002`
* HTTP, port `3003`
* Iron, port `3000`
* Tide, port `3001`
* Warp, port `3004`

Rust binary sizes:

+-----------+---------+------------------+-------------+-------------+----------------+
| Library   | Version | Build            | Release     | Release     | Release (musl) |
+-----------+---------+------------------+-------------+-------------+----------------+
| Actix-Web | 3.3.2   | 83M (linked)     | 8M (linked) |  M (static) |  M (static)    |
| HTTP      |         |     (linked)     |    (linked) |    (static) |    (static)    |
| Iron      |         |     (linked)     |    (linked) |    (static) |    (static)    |
| Tide      |         |     (linked)     |    (linked) |    (static) |    (static)    |
| Warp      |         |     (linked)     |    (linked) |    (static) |    (static)    |
+-----------+---------+------------------+-------------+-------------+----------------+

Container sizes:

+-----------+---------+-----------+--------------+---------------+-------------+
| Library   | Version | Upstream  | Debian-glibc | Busybox-glibc | Alpine-musl |
+-----------|---------|-----------|--------------|---------------|-------------|
| Actix-Web | 3.3.2   | 1.4G      | 80M          | 14M(broken)   |             |
| HTTP      |         |           |              |               |             |
| Iron      |         |           |              |               |             |
| Tide      |         |           |              |               |             |
| Warp      |         |           |              |               |             |
+-----------+---------+-----------+--------------+---------------+-------------+

Build times

+-----------+---------+-----------+--------------+---------------+-------------+
| Library   | Version | Upstream  | Debian-glibc | Busybox-glibc | Alpine-musl |
+-----------|---------|-----------|--------------|---------------|-------------|
| Actix-Web | 3.3.2   |           |              |               |             |
| HTTP      |         |           |              |               |             |
| Iron      |         |           |              |               |             |
| Tide      |         |           |              |               |             |
| Warp      |         |           |              |               |             |
+-----------+---------+-----------+--------------+---------------+-------------+

## Testing

Nov 2020 listing of some HTTP testing libraries [by Alexander Liesenfeld](https://dev.to/alexliesenfeld/rust-http-testing-with-httpmock-2mi0)
The test mock gives some assurance we are comparing like to like minimal-functionality.
Of course some frameworks may be building in default functionality that we don't use -
that is point of these baseline measurements

## Minimal build time: Development builds

## Minimal build size: Production builds

```bash
alias rust-musl-builder='sudo podman run --rm -it -v "$(pwd)":/home/rust/src ekidd/rust-musl-builder'
```

### Iron: Hello world

#### Binary

[Minimal Iron](https://blog.semicolonsoftware.de/building-minimal-docker-containers-for-rust-applications/)

The debug `cargo build` binary (dynamically linked):

``` BASH
$ ls -lh target/debug/iron
-rwxrwxr-x 2 hedge hedge 16M Mar 21 12:36 target/debug/iron
```

Build minimized image for a container:

```bash
alias rust-musl-builder='podman run --rm -it -v "$(pwd)":/home/rust/src ekidd/rust-musl-builder'
$ time rust-musl-builder cargo build --release
```

To verify size:

```bash
$ ldd target/x86_64-unknown-linux-musl/release/myapp
        not a dynamic executable
$ ls -lh target/x86_64-unknown-linux-musl/release/myapp
-rwxr-xr-x 1 user user 1.9M Jul 13 18:53 target/x86_64-unknown-linux-musl/release/myapp
```

#### Container

To build and check the glibc container size

```bash
$ sudo podman build --tag=hw-actix-web --file Dockerfile .
$ sudo podman images | grep hw-actix-web
$ sudo podman run --rm -it --publish 3002:3002 hw-actix-web
```

To build and check the MUSL build (with performance risks) container size

```bash
$ sudo podman build --tag=hw-actix-web --file Dockerfile.musl .
$ sudo podman images | grep hw-actix-web
$ sudo podman run --rm -it --publish 3002:3002 hw-actix-web
```


### Actix-web: Hello world

Follows the Iron build steps.

```bash
$ time cargo build
...
<approx 15 minutes>
$ time cargo build --release
...
real    10m0.507s
user    13m39.187s
sys     0m24.073s

$ cargo run

# $ curl localhost:3001
# Hello world!
```

To verify debug and release application size:

```bash
$ ls -lh target/debug/hw-actix-web
-rwxrwxr-x 2 hedge hedge 83M Mar 21 13:42 target/debug/hw-actix-web

$ ls -lh target/debug/hw-actix-web
-rwxrwxr-x 2 hedge hedge 83M Mar 21 13:42 target/debug/hw-actix-web

$ ls -lh target/release/hw-actix-web
-rwxrwxr-x 2 hedge hedge 7.9M Mar 21 16:17 target/release/hw-actix-web
```

Build minimized image for a container:

```bash
$ time rust-musl-builder cargo build --release
...
real    15m43.847s
user    1m21.019s
sys     0m13.712s
```

To verify size:

```bash
$ ls -lh target/x86_64-unknown-linux-musl/release/hw-actix-web
-rwxr-xr-x 2 user user 8.0M Mar 21 14:16 target/x86_64-unknown-linux-musl/release/hw-actix-web
```

### Tibe: Hello world

Adopts the Iron build steps.
[Minimal Tide](https://medium.com/@gruberbastian/rust-for-the-web-02-x-deploy-your-first-app-51d1ed69cbe3)

```bash
$ time cargo build
...

$ cargo run

# $ curl localhost:3001
# Hello world!
```

Build minimized image for a container:

```bash
$ time rust-musl-builder cargo build --release
```

To verify size:

```bash
$ ldd target/x86_64-unknown-linux-musl/release/myapp
        not a dynamic executable
$ ls -lh target/x86_64-unknown-linux-musl/release/myapp
-rwxr-xr-x 1 seemayer seemayer 1.9M Jul 13 18:53 target/x86_64-unknown-linux-musl/release/myapp
```

### HTTP: Hello world

Follows the Iron build steps.

```bash
$ time cargo build
...

$ cargo run

# $ curl localhost:3001
# Hello world!
```

Build minimized image for a container:

```bash
$ time rust-musl-builder cargo build --release
```

To verify size:

```bash
$ ldd target/x86_64-unknown-linux-musl/release/myapp
        not a dynamic executable
$ ls -lh target/x86_64-unknown-linux-musl/release/myapp
-rwxr-xr-x 1 seemayer seemayer 1.9M Jul 13 18:53 target/x86_64-unknown-linux-musl/release/myapp
```

## References

* https://blog.logrocket.com/packaging-a-rust-web-service-using-docker/
* https://www.artificialworlds.net/blog/2020/04/22/creating-a-tiny-docker-image-of-a-rust-project/
* [circa. 2018](https://whitfin.io/speeding-up-rust-docker-builds/)
