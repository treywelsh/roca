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

Let's suppose we're implementing the `User` resource:

1. Add two sub-controllers structures (`UserController` and `UsersController`), then bind them to the main controller via a method for each (respectively `Users(id)` and `Users()`)
   For the `User` resource, define a structure and basic methods via: `define_resource!(User);`. This will allow to work with the user structure returned by one of the `info` methods.
2. Add more attributes getters in implementing traits with default methods, for instance: 
   `impl GetGroup for XXX {}` and
   `impl GetOwner for XXX {}`
   `impl GetPermissions for XXX {}`
   Previously check if they are required: for instance an `User` resource shouldn't implement `Owner` because it doesn't have `UID` and `UNAME` fields in it's XML representation.
3. Implement resource methods binding them the right user controller. 


For more, [see the architecture document](./architecture.md).


