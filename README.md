# STRATUM

## THIS README IS A DRAFT

Explorational repo. The goal of this repo is to decide the style and structure of the Sv2
implementation.

At the end of the explorational phase (15 Feb?) a repo for the actual impl of Sv2 will be created.
Every meaningful choice about style and structure will be documented thankfully to this
explorational phase.

In order to decide the best structure of the Sv2 repo some code from the (braiins)[todo] repo is
ported here. At the end of the experimental phase, if good enough, the ported code will be
copied in the Sv2 repo.

Things to be decided are: (each point can be discussed in a specific issue)
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

The explorational phase is useful also to set the project's milestones.

Milestones: (an issue for the milestones can be useful)
- [ ] write project's milestones
- [ ] style and structure of the Sv2 repo decided and documented
- [ ] create github org
- [ ] create style.md contribute.md and architecture.md
- [ ] finish the Sv2 BIP
- [ ] Add the necessary crates to the guix repo or create a custom channel
- [ ] Publish version 0.0.1 of (sub)protocols as crates
- [ ] PR on bitcoin core (~June 2021)

## Logs
TODO

## Errors
TO BE DISCUSSED

The error that can be encountered are due to:

1) physical error, IO ecc ecc
2) downstream or upstream node is buggied and respond in a unexpected way
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

Ignoring messages that expect a responses can cause issues. (block the peer)

### Protocol errors V1
The protocol use standard json_rpc errors for format issues.
json_rpc already define what to do for invalid json invalid json_rpc ecc ecc

In Sv1 methods:

Client -> Server
* authorize(username password): username and password not valid
* get_transaction(jobId): not fail if peer is well implemented
* submit(username jobId ExtraNonce2 nTime nOnce): string error can be returned for several reasons
* subscribe(agent/version Option<extranonce1>): not fail if peer is well implemented
* suggest_difficulty(sugg_diff): not fail if peer is well implemented
* suggest_target(..): it can fail if target too low ?

Server -> Client
* get_version(): not fail if peer is well implemented
* reconnect(hostname port waittime) TODO
* show_message(message) TODO
* notify(..): not fail if peer is well implemented
* set_difficulty(difficulty): not fail if peer is well implemented
* set_extranonce(extranonce1, extranonce2Size): not fail if peer is well implemented

If an error occour in handling a _notification_ or a _response_ there is no way to notify the
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

### Errors handled the libraries (protocols):

* message not well formatted

### Errors handled in the implementations (roles):

* lost upstream connection

### Final thoughts

## Documentation
TODO
I like plain cargo docs
`cargo doc --open`

## Test
TODO
`cargo test`

## Examples
TODO
I prefer using examples as examples and tests as tests. Examples are also useful to exploring
various implementations without the burden of writing "robust code".

To run `./protocols/v1/examples/client_and_server/` do `cargo run v1`


## C++ interoperability
TODO

## Monorepo or not
TODO
Monorepo seems simpler

## Build system
TO BE DISCUSSED

The libraries must be buildable on Guix. The libraries should be compilable for the
`arm-openwrt-linux-muslgnueabi-gcc` target. Btw there is no need to compile for
`arm-openwrt-linux-muslgnueabi-gcc` from Guix.

Guix, guix is used to build bitcoin, so the (sub)protocols that are called by the Template Provider
must be buildable with guix. Guix can also be useful to build the roles,  especially the one for
other architectures (Mining Device).
In this explorational phase is analyzed only the possibility to build and package a library crates
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
On the workspace root is present a package module for the Template Distribution Protocol, the source
is defined by the tested git branch and so is the source SHA. In this way the CD can test
unmerged branches. The actual package module for the master branch (the one that is committed on the
guix repository or in a separate channel) is programmatically derived from the one used for testing.

### Guix concept
manifest: used to create an environment.
package module: definition of a package.

### Guix resources
https://guix.gnu.org/manual/en/guix.html
https://guix.gnu.org/cookbook/en/guix-cookbook.html
https://www.youtube.com/watch?v=LnU8SYakZQQ

## CI/CD
TODO
I like github actions

## First experiment Sv1 library + Sv1 client + Sv1 server
The exploration is started porting some Sv1 primitives and implementing a toy Sv1 server and client.

The library do not assume any asynchronous layer and export a Client and a Server trait the can be
implemented using any asynchronous layer.

`cargo run v1` will run the example.

*TO BE DISCUSSED*
IsServer and IsClient are concrete abstraction as probably nothing will need to be generic over them.
IsServer and IsClient are trait and not struct so the final implementation can decide the best
internal structure. Maybe the library should just export a Server and Client struct and then the
implementation just add the network layers?

### Pro
The library part results less complex and it lets more freedom in the implementation choices.
The library follow a [sans-io](https://sans-io.readthedocs.io/) style.

### Cons
TODO
