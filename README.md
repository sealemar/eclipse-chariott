# Project Eclipse Chariott

- [What is Chariott?](#what-is-chariott)
- [How to develop with Chariott](#how-to-develop-with-chariott)
  - [Terminology](#terminology)
  - [Concept of Intents](#concept-of-intents)
- [Requirements](#requirements)
- [Getting started](#getting-started)
  - [Using Dev Container](#dev-container)
    - [Build all binaries and run tests](#build-all-binaries-and-run-tests)
      - [Build on MacOS with Docker VirtuoFS](#build-on-macos-with-docker-virtuofs)
    - [Using Podman instead of Docker](#using-podman-instead-of-docker)
  - [Without Dev Container](#without-dev-container)
    - [Install Dependencies](#install-dependencies)
    - [Build all binaries and run tests natively](#build-all-binaries-and-run-tests-natively)
    - [Build and run Chariott only](#build-and-run-chariott-only)
- [Use Chariott Service Discovery only](#use-chariott-service-discovery-only)
- [How to run the examples and interact with Chariott](#how-to-run-the-examples-and-interact-with-chariott)
- [How to run the dog mode demo](#how-to-run-the-dog-mode-demo)
- [Trademarks](#trademarks)

## What is Chariott?

Chariott is a [gRPC](https://grpc.io) service that provides a common interface for interacting with applications. It facilitates Service Discovery for applications to advertise their capabilities by registering themselves with Chariott's service registry. Other applications that need to consume resources and capabilities can Discover services through Chariott's service registry. In the current version of Chariott, applications can also communicate with each other through Chariott. This is done by having applications register an _intent_ which Chariott will then _fulfill_ by brokering the communication with the appropriate application to fulfill that intent. Applications which fulfill these intents are known as _providers_. More information on Chariott's current design with diagrams can be found [here](./docs/design/README.md).

## How to develop with Chariott

Chariott provides gRPC interfaces to interact from a client application. The
client application can be written in any language that supports gRPC. The
examples in this repository are written in Rust, but the same concepts apply to
any language.

### Terminology

| Term | Description |
| --- | --- |
| Application | An application is defined as any software component. |
| Provider | A provider is also an application that in addition registers its capabilities with Chariott's service registry for other applications to consume. |
| Consuming Application | A consuming application is a client application that interacts with Chariott to look up capability providers and interact with them through Chariott or directly. |
>Note: "provider" or "consuming application" are just roles for an application. Specifically, an application can be both a Chariott "provider" and "consuming application".

### Concept of Intents

Intents are the main way to interact with Chariott. Once a provider registers
an intent with Chariott, other applications can use that intent to interact with
the provider. The intent is a gRPC method that is defined in the provider's
protobuf definition. That definition is only used by Chariott itself.

Chariott also provides a gRPC interface for applications to interact with
providers and delegates the calls based on the intent to the provider transparently.
Therefore, clients don't need to know the location and details of the provider as long as
their intent is fulfilled.

Here is a list of the current supported intents:

| Intent | Description |
| --- | --- |
| Discover | Retrieve native interfaces of providers. This comes in handy if you need specific interaction with a provider that you know is available in the system and you don't want to use Chariott to interact with it. This is also used for retrieving the streaming endpoints of a provider. |
| Inspect | Support inspection of functionality, properties and events using a simple query syntax. |
| Invoke | Invoke a method on a provider. |
| Subscribe | Subscribe to events of a provider. Note that this does not open the streaming channel, this is done through the native streaming endpoint of the provider. |
| Read | Read a property of a provider. |
| Write | Write a property to a provider. |

More information can be found in the protobuf definitions in `./proto`.

There is a separate document that describes the example applications and
scenarios that are supported by Chariott. It can be found
[here](./examples/applications/README.md).

## Requirements

The current source is developed and tested under WSL2/Linux running Ubuntu 20.04
on AMD64 architecture. It is not tested against any other configurations. You
might experience missing support for other platforms, but please feel free to
contribute to close the gaps.

## Getting started

### Dev Container

For development and running the examples, we recommend using the
[Devcontainer](https://code.visualstudio.com/docs/remote/containers) template
provided at `.devcontainer/devcontainer.json`.
[These](https://code.visualstudio.com/docs/devcontainers/containers#_system-requirements)
are the system requirements (including the default requirement of docker) for using devcontainers.
If you decide not to use the Devcontainer, refer to the `devcontainer.json` and the Dockerfile
`.devcontainer/Dockerfile` for a list of the plugins and tools we use.

> Note: If you use Devcontainers and you are running on Windows, make sure to check out the
> repository on the WSL2 file system in the target distribution you're using.

#### Build all binaries and run tests

```bash
cargo build --workspace
cargo test --workspace
```

#### Build on MacOS with Docker VirtuoFS

As reported in [this issue](https://github.com/eclipse-chariott/chariott/issues/111), some extra
steps are needed to build if you are using MacOS with Docker VirtuoFS.

Docker Desktop provides a fast VirtuoFS implementation on a Mac but the rust build process breaks
in devcontainers if VirtuoFS is enabled. The solution is not disabling the VirtuoFS, because this
significantly slows down the I/O operations in the containers.

The workaround is to create a target folder outside the chariott workspace and set the environment
variable to the new target folder. The following sequence works in the devcontainer:

```shell
vscode ➜ /workspaces/chariott (main) $ sudo mkdir ../target
vscode ➜ /workspaces/chariott (main) $ sudo chown vscode:vscode ../target
vscode ➜ /workspaces/chariott (main) $ CARGO_TARGET_DIR=../target cargo build -p chariott
vscode ➜ /workspaces/chariott (main) $ CARGO_TARGET_DIR=../target cargo run -p chariott
```

#### Using Podman instead of Docker

If you want to use Podman you have to [enable Podman in Visual Studio
Code][vscode-podman] and update the `.devcontainer/devcontainer.json` file
with the following additions:

  [vscode-podman]: https://code.visualstudio.com/remote/advancedcontainers/docker-options#_podman

```jsonc
{
  // ...
  "runArgs": [
    "--cap-add=SYS_PTRACE",
    "--security-opt",
    "seccomp=unconfined",
    "--userns=keep-id"
  ],
  // ...
  "workspaceMount": "source=${localWorkspaceFolder},target=/workspace,type=bind,Z",
  "workspaceFolder": "/workspace",
  "containerUser": "vscode",
  // ...
}
```

> **NOTE**: Feel free to use another workspace folder name.

### Without dev container

#### Install Dependencies

As stated above, the `devcontainer.json` and the Dockerfile
`.devcontainer/Dockerfile` contain a list of the plugins/tools we use for Chariott.
Below we have listed the steps to get started, but refer to those files if there are any discrepancies.

This guide uses `apt` as the package manager in the examples. You may need to substitute your own
package manager in place of `apt` when going through these steps.

1. Install gcc:

    ```shell
    sudo apt install gcc
    ```

    > **NOTE**: Rust needs gcc's linker.

1. Install git and git-lfs

    ```shell
    sudo apt install -y git git-lfs
    git lfs install
    ```

1. Install [rust](https://rustup.rs/#), using the default installation, for example:

    ```shell
    sudo apt install curl
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

    You will need to restart your shell to refresh the environment variables.
    > **NOTE**: The rust toolchain version is managed by the rust-toolchain.toml file, so once you
                install rustup there is no need to manually install a toolchain or set a default.

1. Install Protobuf Compiler:

    ```shell
    sudo apt install -y protobuf-compiler
    ```

    > **NOTE**: The protobuf compiler is needed for building the project.

1. Ensure you have openssl and pkg-config installed. These are needed for some of the tests and
examples.

    ```shell
    sudo apt install pkg-config
    sudo apt install libssl-dev
    ```

#### Build all binaries and run tests natively

```bash
cargo build --workspace
cargo test --workspace
```

#### Build and run Chariott only

```bash
cargo run -p chariott
```

## Use Chariott Service Discovery Only

There is a new Service Discovery mechanism that is decoupled from intent brokering. It is currently
under development. Please refer to [this document](./service_discovery/README.md) for more
information.

## How to run the examples and interact with Chariott

Refer to individual example applications' documentation for additional setup or dependencies that may be required.

As Chariott's out of the box communication protocol is gRPC, the interaction with the
examples is done through gRPC. To illustrate how to invoke the gRPC methods we
use the [grpcurl](https://github.com/fullstorydev/grpcurl) command line tool with the example application
**kv-app**. The **kv-app** is a key-value store that can be used to store
and read state. The state is stored in memory and is not persisted. It also demonstrates
the use of the `ess` and `keyvalue` crates.

This walkthrough is described in the [examples kv-app README](examples/applications/kv-app/README.md).

## How to run the dog mode demo

To run the dog mode demo, please refer to the [dog mode demo](./examples/applications/README.md).

## Trademarks

This project may contain trademarks or logos for projects, products, or services. Authorized use of Microsoft
trademarks or logos is subject to and must follow
[Microsoft's Trademark & Brand Guidelines](https://www.microsoft.com/legal/intellectualproperty/trademarks/usage/general).
Use of Microsoft trademarks or logos in modified versions of this project must not cause confusion or imply Microsoft sponsorship.
Any use of third-party trademarks or logos are subject to those third-party's policies.
