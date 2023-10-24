# Architecture

This lib:
- try to reuse the code as much as possible internally
- wraps and XML-RPC API and then mainly manage XML content
- have to manage XML with a lot of "dynamic" parts
- read XML, and write XML and/or OpenNebula custom template format

## XML crates notes

For now roca relies on `xml-doc` to read/write the XML with dynamic parts, but it don't use the editing capability.
It's not the fatest crate and not maintained anymore but it's handy for now.

Here are some notes I took while testing various XML crates:

I tried a bunch of various XML crates to work with the partially dynamic XML. I didn't benchmark them, I only tried them to see how handy they are for my use case.

Low level xml crates: `quick-xml`, `xml-rs`, `xmlparser`
They're all pull parsers and `xmlparser` is read-only.
However we need a parser working at a higher level than XML events.

Some XML crates working at a higher level:

- `xmltree`: allow to read/edit/write. from the crate doc: ```Not recommended for large XML files```
- [serde-xml-rs](https://github.com/tafia/quick-xml/issues/526#issuecomment-1434576848): allow to read/write. serde could be a good choice as we already know a bunch of fields that we could add in a structure but I don't like the feeling with dynamic XML parts.
- `sxd-path` and `sxd-document`: allow to read/write.
- `xml-doc`: allow to read/edit/write. built on top of `quick-xml` provider higher level API. Seems a good compromise for Roca but not maintained anymore.
- `roxmltree`: read only, based on `xmlparser`. Keep track of the input via a reference. Maybe the faster, but doesn't seems convenient to integrate with the API I have in mind (use a bunch wrapping struct). (https://stackoverflow.com/questions/76449289/returning-a-parsed-xml-node-from-a-function-in-rust)
- didn't tried other crates depending on an external lib like `libxml`

At first I tried `sxd-path` and `sxd-document` crates in roca, the code using sxd crate it available at `master_sxd_crate` repository branch.

## Update an existing resource

There's two ways to update an existing resource, we send a bunch of attribute to OpenNebula and ask it either to merge them with existing attributes, or replace the existing attributes.
We may want to use the replace behavior when deleting some attributes for instance.

Let's imagine we want to delete a custom attribute: 
Each OpenNebula resource has a dynamic section named "template" and containing some optionals and custom keys (VM resource has even an additional template section named "user_template").
So first we retrieve the template section content, then we remove the custom attribute and send it back to OpenNebula with the `replace` behavior.

So we need to be able to work with the template content to modify it.
To achieve this roca could:

1. Either define a struct named `TemplateMut` (returned by a `template_mut` method) allowing to edit XML existing content returned by XML-RPC methods.
  But we also want to be able to build a template from scratch and `TemplateMut` would be here to edit existing XML
2. Or define a single struct `TemplateBuilder` that we use for all of these operations: create or edit a template
  We could generate the `TemplateBuilder` from a read-only template via an `into` method then edit it's content

In both case we'll need a `TemplateBuilder` to build a template from scratch so the 2. solution would be probably less efficient but simpler from the API point of view.

Unless I'm able to hide the additional struct of 1. behind a wrapper (via some enum ?) I'll prefer solution 2.

## Define a resource

This section will mainly describe what's happening when writing `define_resource!(User);` to define a `User` resource and how this crate try to share as most code as possible among the resources to manage their attributes.

`define_resource` create a new struct embedding a `Resource`, for instance:
```
struct User {
   resource: Resource,
}
```

Then it will define some basic methods on it.
`Resource` is a struct providing some generic methods (`id`, `name`, `get`...) to retrieve informations from the XML content returned by XML-RPC methods of the OpenNebula API.

Now the goal is to expose the generics methods from `Resource` directly on `User` without making `resource` field public. In Rust to share behavior we use traits.

So now we want to use a trait (we name it `ResourceGetters`) with default methods (named identically `id`, `name`... and calling `Resource` methods internally) that we want to be implemented by `User`.

But there's still a problem, how could `ResourceGetters` knows and access the `User` resource field ?
A trait is neither able to store datas, nor to suppose the fields names of a struct, but, a trait is able to depends on another trait defining some getters methods.

To solve this we define an other trait named `XMLDocGetters` defining a getter methods allowing to retrieve the resource field from `User` struct.
Then we make `ResourceGetters` depends on it.

And last, we use a blanket implementation to automate the implemation of `ResourceGetters` when `XMLDocGetters` is defined by a struct.


