# Architecture

This lib:
- try to reuse the code as much as possible internally
- wraps and XML-RPC API and then mainly manage XML content
- have to manage XML with a lot of "dynamic" parts
- read and write XML

## Allow template modification

Each OpenNebula resource has a "template" section (VM resource has even an additional template section named "user_template").
Sometimes we may want to retrieve this template section content and modify it before sending it to an update method (with replace parameter for instance).

To achieve this roca could:

- Either a struct named TemplateMut (returned by a template_mut method) allowing to edit XML existing content returned by XML-RPC methods.
  But we also want to be able to build a document from scratch and we don't necessarily want to depends on the XML resource document.
  Infortunately it seems that xml-doc crate is not able to detach the template XML part of the document to put it in an other structure so we need to keep a ref on the initial document...
- Or a struct independant of the resource that could be generated from the read-only Template struct (Builder struct returned via a method make_mut). This imply some heavier XML <-> struct translations.
  However we could replace xml-doc for instance by roxmltree when reading only XML, and an other crate for writing XML document, something working at a lower like quick-xml level could be fine.

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

To solve this we define an other trait named `GetNode` defining some getters methods allowing to retrieve the resource field from `User` struct.
Then we make `ResourceGetters` depends on it.

And last, we use a blanket implementation to automate the implemation of `ResourceGetters` when `GetNode` is defined by a struct.


## Traits

- `Get` trait allow to define methods with a generic parameter to accept a dynamic document like a template (template, template mut, builder)

## XML crates note

I tried a bunch of various XML crates to work with the partially dynamic XML. I didn't benchmark them, I only tried them to see how handy they are for my use case.

- `quick-xml`: allow to read/write. would be a good choice but it's low level so it would require to take time to develop a wrapper to work at a higher level with the XML (note: it's already done by the generic lib `xml-doc`)
- `xmltree`: allow to read/edit/write. from the crate doc: ```Not recommended for large XML files```
- [serde-xml-rs](https://github.com/tafia/quick-xml/issues/526#issuecomment-1434576848): allow to read/write. serde could be a good choice as we already know a bunch of fields that we could add in a structure but I don't like the feeling with dynamic XML parts.
- `sxd-path` and `sxd-document`: allow to read/write.
- `xml-doc`: allow to read/edit/write. built on top of quick-xml provider higher level API. Seems a good compromise for Roca but not maintained anymore.
- `roxmltree`: read only. Keep track of the input via a reference. Maybe the faster, but doesn't seems convenient to integrate with the API I have in mind (use a bunch wrapping struct). (https://stackoverflow.com/questions/76449289/returning-a-parsed-xml-node-from-a-function-in-rust)

At first I chose `sxd-path` and `sxd-document` crates, however I wasn't able to edit XML retrieved from OpenNebula and the code was more complex than with `xml-doc`, so I finally chose `xml-doc`.
The code using sxd crate it available at `master_sxd_crate` repository branch.