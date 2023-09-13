# roca

EXPERIMENT UNDER HEAVY REFACTORING

Rust binding for OpenNebula API

Early POC inspired of:

- [Golang API](https://github.com/OpenNebula/one/tree/master/src/oca/go/src/goca)
- https://github.com/christian7007/roca

And [here is the XML-RPC documentation for OpenNebula 6.6](https://docs.opennebula.io/6.6/integration_and_development/system_interfaces/api.html)

## Requirements

This package has to be installled (required by crate reqwest):
```
sudo apt install libssl-dev
```

## Implemented resources

- user (partial)
- virtual machine (partial)

## How to implement a new resource

1. Create a struct with the name of the resource (like `User`), and 
   add a `Resource` type field inside.
2. implement trait `XMLDocGetters` for the struct and define it's methods.
   Then `BaseGetters` trait will be implemented automatically (blanket impl).
   It provides some generic methods (`get`, `get_vector` ...).
3. Add more attributes getters in implementing traits with default methods, for instance: 
   `impl GetGroup for XXX {}` and
   `impl GetOwner for XXX {}`
   `impl GetPermissions for XXX {}`
   Previously check if they are required: for instance an `User` resource shouldn't implement `Owner` because it doesn't have `UID` and `UNAME` fields in it's XML representation.


For more, [see the architecture document](./architecture.md).

## TODOs

- is it enough to identify a template (?): BaseGetters + Display
- UPDATE README AND ARCHITECTURE
- share methods between Resource and ResourceFromPool ?
- be more verbose on thread safety
- map state strings to integers
- merge resource and template_builder ?
- implement pool methods, and manage parameters (-1, -1...)
- fix all TODOs
- allow to parse OpenNebula specific template format
- improve tests, increase coverage
- more code reuse for common resource methods like with delete, chmod
- look for another XML-RPC crate ?
  `serde_xmlrpc` lack a bit of flexibility when a method return type may vary regarding it's success:
  a string type if it's an error, or an ID if it's successful
  In `parse_id_resp` method of the controller we need to call response_from_str twice
- implement iterators traits at least for resource pool
