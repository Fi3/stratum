# STRATUM

## THIS README IS A DRAFT

Explorational repo. The goal of this repo is to decide the style and structure of the Sv2
implementation.

At the end of the explorational phase (15 Feb?) a repo for the actual impl of Sv2 will be created.
Every meaningful choice about style and structure will be documented thankfully to this
explorational phase.

In order to decide the best structure of the Sv2 repo some code from the [braiins][braiins] repo is
ported here. At the end of the experimental phase, if good enough, the ported code will be
copied in the Sv2 repo.

[braiins]: https://github.com/braiins/braiins/tree/bos-devel/open

Things to be decided are:
* log
* error
* documentation
* test
* examples
* C++ interoperability
* monorepo or not
* build system
* benchmark
* CI/CD
* ...

This repo will be kept as a simpler version of the Sv2 repo so if some big structural change is
needed in the Sv2 repo can be easily experimented here.

The explorational phase is useful also to set the project's milestones.

Milestones/Roadmap:
- [ ] Write project's milestones (~15 February 2021)
- [ ] Style and structure of the Sv2 repo decided and documented (~15 February 2021)
- [ ] Create github organization (~15 February 2021)
- [ ] Create github page with blog and track record for Sv2 (~1 March 2021)
- [ ] Create style.md contribute.md and architecture.md (~1 March 2021)
- [ ] Publish version 1.0.0 of Sv2 noise authentication on crates.io (~15 April 2021)
- [ ] Publish version 1.0.0 of Template Distribution Protocol on crates.io (~15 April 2021)
- [ ] Publish version 1.0.0 of Mining Protocol on crates.io (~15 April 2021)
- [ ] Add the Template Distribution Protocol to the guix repo or create a custom channel (~June 2021)
- [ ] Finish the Sv2 BIP (~June 2021)
- [ ] Implementation of Template Provider in bitcoin (~1 June 2021)
- [ ] PR on bitcoin core (~1 June 2021)
- [ ] Publish version 1.0.0 of Job Negotiation Protocol on crates.io (~15 July 2021)
- [ ] Publish version 1.0.0 of Mining Device on crates.io (~1 September 2021)
  * it is both a library and a binary crate
- [ ] Publish version 1.0.0 of Pool Service on crates.io (~15 October 2021)
  * it is both a library and a binary crate
- [ ] Publish version 1.0.0 of Sv1 on crates.io (~15 December 2021)
- [ ] Publish version 1.0.0 of Mining Proxy on crates.io (~15 December 2021)
  * it is both a library and a binary crate
  * v1 support from both up and down streams
  * job negotiator 
- [ ] PR merged (~1 January 2022)

## Logs
TODO

## Errors
TO BE DISCUSSED

The error that can be encountered are due to:

1) physical error, IO ecc ecc
2) downstream or upstream node is buggy and respond in a unexpected way
3) downstream or upstream node is well implemented but not compatible

V1 and V2 crates should be aware only about error kinds 2 and 3.
V1 and V2 crates should never panic.
Error kinds 1 are handled in the implementations.

If the error is not contemplated by the specifications custom actions must be defined in the
implementation.

The most probable action are:
* retry
* close the connection
* ignore
* panic

Ignoring messages that expect responses can cause issues. (block the peer)

### Protocol errors V1
The protocol uses standard json_rpc errors for format issues.
json_rpc already define what to do for invalid json invalid json_rpc ecc ecc

In Sv1 methods:

Client -> Server
* authorize(username password): username and password not valid
* get_transaction(jobId): not fail if peer is well implemented
* submit(username jobId ExtraNonce2 nTime nOnce): string error can be returned for several reasons
* subscribe(agent/version Option<extranonce1>): not fail if peer is well implemented
* suggest_difficulty(sugg_diff): not fail if peer is well implemented
* suggest_target(..): it can fail if target is too low ?

Server -> Client
* get_version(): not fail if peer is well implemented
* reconnect(hostname port waittime) TODO
* show_message(message) TODO
* notify(..): not fail if peer is well implemented
* set_difficulty(difficulty): not fail if peer is well implemented
* set_extranonce(extranonce1, extranonce2Size): not fail if peer is well implemented

If an error occur in handling a _notification_ or a _response_ there is no way to notify the
counterpart about the error. 

### Protocol errors V2

Messages with an unknown extension_type which are to be processed locally  MUST be discarded and 
ignored.

The protocol uses string error codes. The list of error codes can differ between
implementations, and thus implementations MUST NOT take any automated action(s) on the
basis of an error code. Implementations/pools SHOULD provide documentation on the
meaning of error codes and error codes SHOULD use printable ASCII where possible.
Furthermore, error codes MUST NOT include control characters.
To make interoperability simpler, the following error codes are provided which
implementations SHOULD consider using for the given scenarios. Individual error codes are
also specified along with their respective error messages.
* ‘unknown-user’
* ‘too-low-difficulty’
* ‘stale-share’
* ‘unsupported-feature-flags’
* ‘unsupported-protocol’
* ‘protocol-version-mismatch’

TODO (does it mean that error codes are only used for logging purposes?)

### Protocol errors V2 <-> V1

### Errors handled in the libraries ((sub)protocols):

* message not well formatted

### Errors handled in the implementations (roles):

* lost upstream connection

### Final thoughts

### Best Practice
Library crates should implement basic traits as suggested [here][error1]. Also library crates
should avoid using helper libraries as _thiserror_ cause they can have a big impact on compilation
times (just for now, 02/2021) as stated [here][error2].

[error1]: https://blog.burntsushi.net/rust-error-handling/#advice-for-library-writers
[error2]: https://www.reddit.com/r/rust/comments/gj8inf/rust_structuring_and_handling_errors_in_2020/fqlmknt?utm_source=share&utm_medium=web2x&context=3

## Documentation
TODO

I like plain cargo docs:

`cargo doc --open`

### Style:
https://github.com/rust-lang/rfcs/blob/master/text/1574-more-api-documentation-conventions.md#appendix-a-full-conventions-text

https://stackoverflow.com/questions/31582064/how-to-link-to-other-fns-structs-enums-traits-in-rustdoc

https://deterministic.space/machine-readable-inline-markdown-code-cocumentation.html


## Test
TODO

Doctest are good to keep documentation updated.

`test.sh`

## Examples
TODO

I prefer using examples as examples and tests as tests. Examples are also useful for exploring
various implementations without the burden of writing "robust code".

To run `./protocols/v1/examples/client_and_server/` do `cargo run v1`


## C++ interoperability
TODO

`protocols/guix-example/`

## Monorepo or not
TODO

Monorepo seem simpler

### Cargo workspaces useful links:
https://www.reddit.com/r/rust/comments/a39er8/how_do_you_publish_your_cargo_workspace_packages/

## Build system
TO BE DISCUSSED

The libraries must be buildable on Guix.

The libraries should be compilable for the `arm-openwrt-linux-muslgnueabi-gcc` target. Btw there is
no need to compile for `arm-openwrt-linux-muslgnueabi-gcc` from Guix.

Guix is used to build bitcoin, so at least the _Template Distribution Protocol_ must be buildable 
with guix. Guix can also be useful to build the roles,  especially the one for other architectures 
(Mining Device).
In this explorational phase is analyzed only the possibility to build and package a library crate
as a C lib with guix. The possibility to build the roles using guix is not analyzed cause not
strictly necessary for the success of this project but probably will be used in the future.

### Install guix
In order to install guix I used the [shell install script][guix-install]. In my system I
need to run the script as the root user and not with sudo.
In my system systemctl fail to launch the guix daemon, prababilly [this][guix-install2] would solve
the issue, but I just launch the service with `guix-deamon --build-users-group=guixbuild` from root.
Then I followed chapter [2.6.1][guix-install3], [2.6.2][guix-install4], [2.6.3][guix-install5] and
[10.10][guix-install6].

[guix-install]: https://guix.gnu.org/manual/en/guix.html#Installation
[guix-install2]: https://guix.gnu.org/manual/en/guix.html#SELinux-Support
[guix-install3]: https://guix.gnu.org/manual/en/guix.html#Locales-1
[guix-install4]: https://guix.gnu.org/manual/en/guix.html#Name-Service-Switch-1
[guix-install5]: https://guix.gnu.org/manual/en/guix.html#X11-Fonts
[guix-install6]: https://guix.gnu.org/manual/en/guix.html#X_002e509-Certificates
[guix-getting-started]: https://guix.gnu.org/manual/en/guix.html#Getting-Started

### Test and deploy guix
An example in test.sh

### Guix concept
manifest: used to create an environment.

package module: definition of a package.

### Guix resources
https://guix.gnu.org/manual/en/guix.html

https://guix.gnu.org/cookbook/en/guix-cookbook.html

https://guix.gnu.org/en/blog/2018/a-packaging-tutorial-for-guix/

https://www.youtube.com/watch?v=LnU8SYakZQQ

## CI/CD
TODO

I like github actions

## First experiment Sv1 library + Sv1 client + Sv1 server
The exploration is started porting some Sv1 primitives and implementing a toy Sv1 server and client.

The library does not assume any asynchronous layer and export a Client and a Server trait that can be
implemented using any asynchronous layer.

`cargo run v1` will run the example.

*TO BE DISCUSSED*

IsServer and IsClient are concrete abstractions as probably nothing will need to be generic over them.
IsServer and IsClient are trait and not struct so the final implementation can decide the best
internal structure. Maybe the library should just export a Server and Client struct and then the
implementation just add the network layers?

### Pro
The library part results less complex and it lets more freedom in the implementation choices.
The library follows a [sans-io](https://sans-io.readthedocs.io/) style.

### Cons
A lot of boilerplate when the trait is implemented.
