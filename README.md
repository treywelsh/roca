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

1. Create a struct with the name of the resource (like `User`), and 
   add a `Resource` type field inside
2. implement trait `ResourceGetter` and define it's methods.
   Then `CommonGetters` trait with blanket implementation will be implemented automatically.
   It provides some generic methods (`id`, `name`, `get_str`, ...).
3. Add more attributes getters in implementing traits with default methods, for instance: 
   `impl Group for XXX {}` and
   `impl Owner for XXX {}`
   Previously check if they are required: for instance an `User` resource shouldn't implement `Owner` because it doesn't have `UID` and `UNAME` fields in it's XML representation.

## TODOs

- more code reuse: for user delete and passwd have the same code (except RPC method and parameters). In addition, various resources have some identical or near identical methods (allocate, info, delete...)
- look for another XML-RPC crate ?
  `serde_xmlrpc` lack a bit of flexibility when a method return type may vary regarding it's success:
  a string type if it's an error, or an ID if it's successful
  In `parse_id_resp` method of the controller we need to call response_from_str twice
- implement iterators traits for the templates

## Note

I tried a bunch of various XML crates (`quick-xml`, `xmltree`, [serde-xml-rs](https://github.com/tafia/quick-xml/issues/526#issuecomment-1434576848), `sxd-XXX`, `xml-doc`) to work with the partially dynamic XML. I didn't benchmark them, I only tried them to see how handy they are for my use case.

At the end I chose `sxd-path` and `sxd-document` crates I wasn't able to edit XML retrieved from OpenNebula and the code was more complex than with `xml-doc`, so I finally chose `xml-doc`.

`xml-doc` is not maintained anymore but it appears to me that's a good fit for `roca` so I may want to provide bug fixes if needed.
If it's a problem `xmltree` may be the nearest it's behavior.
It's built on top of `quick-xml` making it a lot more easy to use.

The code using sxd crate it available at `master_sxd_crate` repository branch.
