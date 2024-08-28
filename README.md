# RDC CLI

RDC (Rust Domain-driven design CLI) is a powerful command-line tool designed to facilitate the development of Rust projects following the Domain-driven Design (DDD) architecture. RDC accelerates the development process by automating project structure creation and DDD component generation.

## Features

- Generate Rust project structure following DDD architecture
- Add new features to existing projects
- Create and manage DDD components such as domain services, services, entities, and more
- Automatically generate boilerplate code for DDD components

## Installation

To install RDC, run the following command:

```
cargo install rdc
```

## Usage

### Create a new project

```
rdc new <project-name>
```

### Add a new feature

```
rdc add feature <feature-name>
```

### Add a domain service

```
rdc add domain-service <service-name>
```

### Add a service

```
rdc add service <service-name>
```

### Add an entity

```
rdc add entity <entity-name>
```

## Project Structure

RDC generates the following project structure:

```
src/
├── domain/
│   ├── entities/
│   ├── value_objects/
│   └── domain_services/
├── application/
│   └── services/
├── infrastructure/
│   ├── repositories/
│   └── external_services/
└── interfaces/
    ├── api/
    └── cli/
```

## Contributing

We welcome contributions! Please see the CONTRIBUTING.md file for more details on how to contribute to the project.

## License

RDC is distributed under the MIT license. See the LICENSE file for more details.
