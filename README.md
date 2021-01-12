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
- [ ] finish the Sv2 BIP
- [ ] PR on bitcoin core

## Logs
TODO

## Errors
TODO

## Documentation
TODO
I like plain cargo docs

## Test
TODO

## Examples
TODO
I prefer using examples as examples and tests as tests. Examples are also useful to exploring
various implementations without the burden of writing "robust code".

## C++ interoperability
TODO

## Monorepo or not
TODO
Monorepo seems simpler

## Build system
TODO
The libraries must be buildable on Guix. The libraries should be compilable for the
arm-openwrt-linux-muslgnueabi-gcc target.

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

### Cons
TODO
