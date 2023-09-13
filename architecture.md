# Architecture

This lib:
- try to reuse the code as much as possible internally
- wraps and XML-RPC API and then mainly manage XML content
- have to manage XML with a lot of "dynamic" parts

## Share behavior among resources

`Resource` is a struct providing some generic methods (`id`, `name`, `get`...) to retrieve informations from the XML content returned by XML-RPC methods of the OpenNebula API. This struct is then embedded in specialized structs that represents a resource (`VM`, `User`...)
For instance:
```
struct User {
   resource: Resource,
}
```

Now our goal is to expose the generics methods from `Resource` directly on `User` without making "resource" field public.
In Rust to share behavior we use traits.

So now we want to use a trait (we name it `ResourceGetters`) with default methods (named identically `id`, `name`... and calling `Resource` methods internally) that we want to be implemented by `User`.

But there's still a problem, how could `ResourceGetters` knows and access the `User` resource field ?
A trait is neither able to store datas, nor to suppose the fields names of a struct, but, a trait is able to depends on another trait defining some getters methods.

To solve this we define an other trait named `XMLDocGetters` defining some getters methods allowing to retrieve the resource field from `User` struct.
Then we make `ResourceGetters` depends on it.

And last, we use a blanket implementation to automate the implemation of `ResourceGetters` when `XMLDocGetters` is defined by a struct.


## Traits

- `Get` trait allow to define methods with a generic parameter to accept a dynamic document like a template (template, template mut, builder)

## XML crates note

I tried a bunch of various XML crates (`quick-xml`, `xmltree`, [serde-xml-rs](https://github.com/tafia/quick-xml/issues/526#issuecomment-1434576848), `sxd-XXX`, `xml-doc`) to work with the partially dynamic XML. I didn't benchmark them, I only tried them to see how handy they are for my use case.

Temporarily I chose `sxd-path` and `sxd-document` crates, however I wasn't able to edit XML retrieved from OpenNebula and the code was more complex than with `xml-doc`, so I finally chose `xml-doc`.

`xml-doc` is not maintained anymore but it appears to me that's a good fit for `roca` so I may want to provide bug fixes if needed.
If it's a problem `xmltree` may be the nearest it's behavior.
It's built on top of `quick-xml` making it a lot more easy to use.

The code using sxd crate it available at `master_sxd_crate` repository branch.