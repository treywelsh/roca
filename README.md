# roca

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

## How to implement a new resource

1. Create a struct with the name of the resource (like `User`)
2. implement trait `ResourceData` and define it's methods to enable specialization of the default methods added in step 3.
   Then traits with blanket implementation `ResourceInternal` and `ResourcePublic` will be implemented.
   They add generic getters like `get_str`, `get_int` etc...
   Internal and Public getters are both exposed to a roca user, they distinguished for now:
   - Internal getters allow to get attributes from an string path that we know at compile time
   - Public ones only need the attribute name (they are more convenient to use for an externl user)
   Are internal getters truly required ?
4. Add more attributes getters via macros `getters`, `group_getters`
   They allow to get attributes defined for all resources: `ID`, `NAME`, `GROUP_ID`...

## TODOs

- allow user to write it's own XML-RPC client based on an other HTTP client (Errors depends on request etc.)
- more code reuse: for user delete and passwd have the same code (except RPC method and parameters). In addition, various resources have some identical or near identical methods (allocate, info, delete...)
- look for another XML-RPC crate ?
  `serde_xmlrpc` lack a bit of flexibility when a method return type may vary regarding it's success:
  a string type if it's an error, or an ID if it's successful
  In `parse_id_resp` method of the controller we need to call response_from_str twice
- implement iterators traits for the templates
