# Whack-a-Dep!

A dashboard to update your dependencies.

![whackadep](whackadep.png)

## Usage

To run the whole thing for development (requires [docker-compose](https://docs.docker.com/compose/)):

```sh
GITHUB_TOKEN=<PAT> make
```

where PAT is a personnal access token for Github ([see steps here on how to create one](https://github.com/mimoo/cargo-dephell#usage))

This will re-build everything all the time, if you know there hasn't been any changes lately you can simply run:

```sh
GITHUB_TOKEN=<PAT> make fast
```

These commands will run the following four services:

* a web frontend on http://localhost:8080
* an API backend on http://localhost:8081
* a mongodb server on http://localhost:27017
* a mongodb UI on http://localhost:8082

This dev setup has hot reload for the front end (you can change front end files, and it'll get reflected) but not for the backend.

To rebuild the backend (or frontend) you can use:

```sh
make restart-backend
```

To SSH into the backend (or frontend) you can use:

```sh
make ssh-backend
```

## Architecture

![whackadep architecture](architecture.png)

The architecture looks like the following:

- [web-frontend](web-frontend). This is the web dashboard written in [Vue.js](https://vuejs.org/) version 3. It queries the web backend to obtain information on dependencies.
- [web-backend](web-backend). This is the dashboard that you use to manage your dependencies. It is written with the [Rocket](https://rocket.rs/) web framework. It also serves a **metrics** API built on top of the [metrics](web-backend/metrics) crate.
- [db](db). This is the [Mongodb](https://www.mongodb.com/) database where information about dependencies throughout the lifetime of the codebase are persisted.
- [cronjobs](cronjobs). These are cronjobs that call the backend's metric API periodically in order to check if new dependency upgrades are available. (They don't work at the moment!)

## Metrics

Metrics on dependencies are obtained via a **metrics** service within the [web-backend](web-backend).
The service is implemented using the [metrics](web-backend/metrics) crate.

![metrics](metrics.png)

Essentially, what the service does when called is:

1. Make sure it has a local copy of the [diem/diem repository](https://www.github.com/diem/diem).
2. Pull the latest changes from the repository.
3. Parse any dependency file (e.g. `Cargo.toml`) to obtain a list of dependencies.
4. Check if any of these dependencies have updates (for example, by querying crates.io).
5. Check how urgent these updates are (for example, by checking output of cargo-audit).
6. Check how shady these updates are (for example, by checking red flags on Github)
7. Store this information in mongodb under a new `_id`.

Note that for steps 3 and 4, [dependabot](https://github.com/dependabot/dependabot-core/blob/e207e5810b/cargo/lib/dependabot/cargo/file_updater/manifest_updater.rb) has code that handles many types of file and package manager (Rust, Dockerfile, npm, etc.)
While we do use it, we want to perform more granular analysis on our Rust dependency.
For example, we want to understand what updates are more urgent than others based on semver, breaking changes, [RUSTSEC advisories](https://rustsec.org/), Github statistics, dev dependency, etc.
For this reason, we use custom code (built on top of [guppy](https://github.com/facebookincubator/cargo-guppy/)) to analyze parts of Rust dependencies.
