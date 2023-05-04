---
title: A Deep Dive into API Design and Auto-Generation
excerpt: How to design and version APIs with a touch of automated client and server creation.
last_modified_at: 2023-05-04T09:02:21
categories:
 - Programming
tags:
  - open-api
  - swagger
  - infrastructure
---

## Overview
### Why it's Important

### Semantic Versioning
[Semantic Versioning](https://semver.org/spec/v2.0.0.html) is a way of giving
meaning to the version of library code to make dependency management more
digestible. Its core tenants are:

{% capture notice-text %}
Given a version number `MAJOR.MINOR.PATCH`, increment the:

1. `MAJOR` version when you make incompatible API changes
1. `MINOR` version when you add functionality in a backwards compatible manner
1. `PATCH` version when you make backwards compatible bug fixes
{% endcapture %}
<div class="notice--info">
  {{ notice-text | markdownify }}
</div>

### Pet Store (Sample API)
To draw some examples, let's use a sample API [taken from OpenAPI
Generator](https://github.com/OpenAPITools/openapi-generator/tree/master/samples/yaml).

This simple API, called `Pet Store`, outlines a simple storefront where users
can place `Order`s for their `Pet`s. There are then exposed endpoints for most
[CRUD operations](https://en.wikipedia.org/wiki/Create,_read,_update_and_delete)
to operate on `Pet`s and `Order`s.

For simplicity, we're using readable function names *(`getPetById()`)* instead of
the URI a client would call *(`GET /pet/{petId}`)*.
{: .notice--info }

{% mermaid %}
classDiagram
    Order <|-- OrderStatus
    Pet <|-- Tag
    Pet <|-- PetStatus
    Pet <|-- Category
    class Order {
        Int id
        OrderStatus status
        Int petId
        Int quantity
        DateTime shipDate
        getOrderById(String orderId) Order
        deleteOrder(String orderId) void
        placeOrder(Order order) void
    }
    class OrderStatus {
        PLACED
        APPROVED
        DELIVERED
    }
    class Tag {
        Int id
        String name
    }
    class Pet {
        Int id
        List~Tag~? tags
        String name
        PetStatus? status
        Category? category
        List~String~? photoUrls
        getPetById(String petId) Pet
        deletePet(String petId) void
        addPet(Pet pet) void
        updatePet(Pet pet) void
        findPetsByStatus(Status status) List~Pet~
        findPetsByTags(List~Tags~ tags) List~Pet~
    }
    class Category {
        Int id
        String name
    }
    class PetStatus {
        AVAILABLE
        PENDING
        SOLD
    }
{% endmermaid %}

Here is a breakdown of the models and API. We can see some getters/setters for `Order` and
`Pet` along with some search capabilities for `Pet`s by `Tag` or `Category`.

Keep a (rough) outline of this application in your head as we outline how the
contracts can be broken:

## Breaking API Changes
In general, to consider if a change will be breaking, it can be good to think of
it from the perspective, **"will an older client be unable to perform existing
behavior?"** All the same, some examples can make this reasoning more clear:

### Removing an Operation
Removing an operation is probably the most obvious example of a breaking change.

There's not a good way to deal with a client attempting to call an endpoint that
no longer exists. The API has changed, so -- in the best case -- we must make
that clear with our next semantic version and users must adapt.

Once functionality has been established in the contract, we must uphold that
until a new version of the API is cut. (Though [deprecation is an
option](#TODO))
{: .notice--info }

For example, let's say that for `Pet Store` we realize that we actually *don't*
want users to be able to delete orders (perhaps they're expecting that to cancel
orders, but their payment is final). We would have to declare that as an
entirely new, major version of the API, which means more pain for users who must
adjust each caller.

{% mermaid %}
classDiagram
    Order ..> Removed : BREAKING
    Order --|> Added : Minor Version
    class Order {
        Int id
        OrderStatus status
        Int petId
        Int quantity
        DateTime shipDate
        getOrderById(String orderId) Order
        deleteOrder(String orderId) void
        placeOrder(Order order) void
    }
    class Added {
        getOrdersShippedSince(DateTime date) List~Order~
    }
    class Removed {
        deleteOrder(String orderId) void
    }
{% endmermaid %}

By contrast, *adding* an operation is not a breaking change, since any existing
client will not be able to perform functionality that did not yet exist.
<br><br>
For this reason, creating a new version of an API can be a good way to minimize
the impact of otherwise breaking changes. [See more examples below](#TODO)
{: .notice--primary }

### Adding a New Requirement
This is another fairly straightforward example. Requirements to existing API
calls, such as additional parameters, will break the current clients that are
unaware of such obligations.

Using `Pet Store` as an example again, if we were to add a new required field to
`Order`, like `purchaseDate`, this would break all existing client calls, since
they are unaware of this parameter.

We can demote this to a Minor Version change by making the parameter optional.
This is because existing clients can still make a valid version of the call.

When introducing new, optional parameters, it is wise to consider what the
default value(s) of that field will be, since all existing callers are -- by
contract -- able to continue calling the API without it.
{: .notice--primary }

{% mermaid %}
classDiagram
    direction RL
    NewOrder <.. Order : BREAKING
    NonBreakingNewOrder <|-- Order : Minor Version
    class Order {
        Int id
        OrderStatus status
        Int petId
        Int quantity
        DateTime shipDate
    }
    class NewOrder {
        Int id
        OrderStatus status
        Int petId
        Int quantity
        DateTime shipDate
        +DateTime purchaseDate
    }
    class NonBreakingNewOrder {
        Int id
        OrderStatus status
        Int petId
        Int quantity
        DateTime shipDate
        +DateTime? purchaseDate
    }
{% endmermaid %}

We often want parameters to be required in order to reduce the scope of support
our server needs to provide. As such, it's important to consider how the surface
of your models may change over time.

Another scenario to consider is a new requirement on an already-existing
parameter. For example, if an incoming `Int` parameter, like `id`, is now
required to be *positive*, that would also be a breaking change.
{: .notice--warning }

### Altering a Parameter / Response Field
Perhaps the sneakiest of our breaking change examples, even renaming a field is
a breaking change.

Although the general shape of data in a model may hold for renamed parameters,
the relative type of your models has now changed, so callers must adjust
accordingly. Perhaps [bikeshedding](https://en.wiktionary.org/wiki/bikeshedding)
on variable names *is* worth it?

{% mermaid %}
classDiagram
    direction RL
    AlteredOrder <.. Order : BREAKING
    class Order {
        Int id
        OrderStatus status
        Int petId
        Int quantity
        DateTime shipDate
    }
    class AlteredOrder {
        Int id
        OrderStatus status
        Int petId
        *Int amountOrdered
        DateTime shipDate
    }
{% endmermaid %}

Here, we rename `quantity` to `amountOrdered` for ""readability"", **breaking**
anyone using the current version of `Pet Store`.

## Non-Breaking API Changes
There are ways of introducing changes without breaking the API. However, while
it may be tempting to use some of these ideas to work around problems with Major
Version changes, there are -- as in much of software engineering -- tradeoffs.

### Adding an Optional Parameter / Header
As [mentioned above](#removing-an-operation), we can introduce a new parameter
without breaking the API by making it optional. In some cases, this is valid,
but in others it can be difficult.

Take this example in `Pet Store`: we add a new, optional `satisfactionRating` to
users' `Order`s. Handling the `null` case is expected, since we don't anticipate
every `Order` being rated.

{% mermaid %}
classDiagram
    direction RL
    NewOrder <|-- Order : Minor Version
    class Order {
        Int id
        OrderStatus status
        Int petId
        Int quantity
        DateTime shipDate
    }
    class NewOrder {
        Int id
        OrderStatus status
        Int petId
        Int quantity
        DateTime shipDate
        +Int? satisfactionRating
    }
{% endmermaid %}

As a result, this change isn't so much working around breaking changes, but
rather a natural evolution of this API. Just because we *can* add an optional
field doesn't mean we should.

Consider the addition of the *optional* `purchaseDate` field in the example
above. It introduces edge behavior because how do we handle a `null`? We would
expect `purchaseDate` to be known.
{: .notice--danger }

### Adding Enum Values
Adding Enum variants is another way to avoid breaking changes. Existing clients
will be limited to only the old set of options, but they can still perform like
they used to.

{% mermaid %}
classDiagram
    RemovedOrderStatus <.. OrderStatus : BREAKING
    AddedOrderStatus <|-- OrderStatus : Minor Version
    class OrderStatus {
        PLACED
        APPROVED
        DELIVERED
    }
    class RemovedOrderStatus {
        PLACED
        APPROVED
    }
    class AddedOrderStatus {
        PLACED
        APPROVED
        DELIVERED
        CANCELLED
    }
{% endmermaid %}

Enums can be a great, natural way to vertically iterate on API design without
needing to cut a new Major Version.
<br><br>
For example, we can add the `CANCELLED` variant once we've implemented a
cancellation policy in `Pet Store`, all without introducing breaking changes.
{: .notice--primary }

### Adding a Response Field / Header
While the client may not understand the information added to the response, it's
perfectly valid for a server to return *more* information than the caller
expects since response deserializers can drop the extra information.

Since there are no "pure" Response types in `Pet Store` (and thus, any added
Response field would also change a Request field), let's imagine that we're
adding a new cookie in a header, like so:

```json
headers: { 'Set-Cookie': 'myKey=myValue;' }
```

## How to Make Breaking Changes
Step-by-step process to help users prepare for change.

### Deprecation
Twofold:
- Tells users what's going away.
- Creates a TODO list of items to remove when making a Major Version change.

### Making Major Version Chages
Major Version changes should actually be major.
