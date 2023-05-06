---
title: A Deep Dive into API Design and Auto-Generation
excerpt: How to design and version APIs with a touch of automated client and server creation.
last_modified_at: 2023-05-04T09:02:21
categories:
 - Programming
tags:
  - api
  - infrastructure
  - open-api
  - rust
  - swagger
---

## Overview
An Application Programming Interface (API) is a set of rules that enable
different applications to communicate with one another. They often expose a list
of operations and associated types that callers can use to interact with a
system.

This is much like the `interface` pattern in most programming languages. For
example, imagine this simple API for placing online orders (which we'll expand
on later):

```rust
struct Order {
  item_name: String,
  quantity: u32,
}

// The API contract for which users can interact with orders
trait OrderApi {
  // Aligns with a POST request to our servers
  fn placeOrder(order: Order): Order

  // Aligns with a DELETE request to our servers
  fn cancelOrder(order: Order)
}
```

Any users of our service can write a client which implements `OrderApi` to
interact with and place orders. Although my snippet above is in Rust, there's
rarely explicit language limits for APIs.

Most APIs are designed in a [RESTful
way](https://en.wikipedia.org/wiki/Representational_state_transfer), where
clients send requests to a specific URL and receive a response from the server,
indicating the status of their request.
<br><br>
For example, the well-known 404 response indicates that a resource, page, or
endpoint was not found.
{: .notice--info }

On the backend, our server will also implement `OrderApi` so that we can process
incoming requests from any user's client.

Because this contract is used externally, our server must ***always*** uphold
the API contract so that anyone who did the work implementing an `OrderApi`
client can continue to call into our service.

### Why it's Important
API design is so important because -- for external-facing code -- we must always
uphold the contract, otherwise we'll ***break*** all existing users!

To consider things from the other perspective, imagine trying to stand up an
application using a library that is ***constantly changing!*** It'd be
frustrating having to frequently update your code when the functionality you
want is largely the same.

For code internal to a single module, we can update both the library and callers
at once, meaning we don't have to think about breaking API changes as much.
<br><br>
This is especially true for monorepos, since callers and providers live in one
place.
{: .notice--info }

As software architecture has largely moved towards microservices (and thus,
multirepos), caution around API changes has expanded to cover internal APIs,
too, since code is deployed in more individual units.

For example, consider the differences in these two architectures:

**Single Deployment:**

{% mermaid %}
flowchart LR
    M[fooService<br>barService<br>fooBarService]
    NM[newFooService<br>newBarService<br>newFooBarService]
    M -..-> NM
{% endmermaid %}

<br>
**Microservices:**

{% mermaid %}
flowchart LR
    F[fooService]
    NF[newFooService]
    F -..-> NF
{% endmermaid %}
{% mermaid %}
flowchart LR
    B[barService]
    NB[newBarService]
    B -..-> NB
{% endmermaid %}
{% mermaid %}
flowchart LR
    FB[fooBarService]
    NFB[newFooBarService]
    FB -..-> NFB
{% endmermaid %}

<br>
Each box represents an individually deployed module and how it would change when
upgrading.

If we assume that `fooBarService` calls into both `fooService` and `barService`,
we can better understand how multirepos must respect API contracts more closely.
{: .notice--info }

In the single deployment approach, since we change all of the services
*together*, we can make bigger changes, because we know each service will always
be in alignment. If `fooService` changes, we know that `fooBarService` will also
change with it.

By contrast, in a microservice architecture, we have no way of knowing *exactly*
which module has been deployed at any given time. If we want to avoid breaking
changes, we need to handle cases where *some* of the new services have been
deployed, but not all.

This grows with the number of services, since we need to handle ***any and
all*** combinations of deployed services. So how can we easily understand the
state of any set of deployments?
{: .notice--warning }

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
<div class="notice--primary">
  {{ notice-text | markdownify }}
</div>

A semantic version gives developers better confidence in integration between
services. A `MINOR` or `PATCH` version bump won't impact existing clients,
though may add new functionality.

It's only when a `MAJOR` version is released that existing clients must adapt to
breaking changes in the API contract, modifying clients along the way.

### Pet Store (Sample API)
To draw some examples, let's expand on the online order API from above, [taken
from OpenAPI Generator' `samples`
library](https://github.com/OpenAPITools/openapi-generator/tree/master/samples/yaml).

This simple API, called `Pet Store`, outlines a storefront where users can place
`Order`s for their `Pet`s. There are exposed endpoints for most [CRUD
operations](https://en.wikipedia.org/wiki/Create,_read,_update_and_delete) to
modify `Pet`s and `Order`s.

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

***Note:*** I will be using UML-*like* diagrams in this article. I am not using
[Mermaid's explicit class
syntax](https://mermaid.js.org/syntax/classDiagram.html#visibility) for my
definitions, but rather, leveraging Mermaid as a means of quickly depicting
examples.
{: .notice--warning }

Here is a breakdown of the models and API. We can see some getters/setters for `Order` and
`Pet` along with some search capabilities for `Pet`s by `Tag` or `Category`.

Keep a (rough) outline of this application in your head as we outline how the
contracts can be broken:

## Breaking API Changes
In general, to consider if a change will be breaking, it can be good to think of
it from the perspective, **"will an older client be unable to perform existing
behavior?"** Some examples can make this reasoning more clear:

### Removing an Operation
This is probably the most obvious example of a breaking change.

There's not a good way to deal with a client attempting to call an endpoint that
no longer exists. The API has changed, so -- in the best case -- we must make
that clear with our next semantic version so that users can adapt.

Once functionality has been established in the contract, we must uphold that
until a new version of the API is cut. (Though [deprecation is an
option](#deprecation))
{: .notice--info }

For example, let's say that -- for `Pet Store` -- we realize that we actually
*don't* want users to be able to delete orders *(perhaps they're expecting this
functionality can cancel their orders, but on our backend, payments are final)*.

We would have to declare that as an entirely new, major version of the API,
which means more pain for users who must adjust each caller:

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
the impact of otherwise breaking changes. *([More examples
below](#versioning-endpoints-and-apis-separately))*
{: .notice--primary }

### Adding a New Requirement
This is another fairly straightforward example. Requirements to existing API
calls -- such as additional parameters -- will break the current clients that
are unaware of such obligations.

Using `Pet Store` as an example again, if we were to add a new required field to
`Order`, like `purchaseDate`, this would break all existing client calls, since
they are unaware of this parameter.

We can demote this to a Minor Version change by making the parameter
**optional**. This is because existing clients can still make a valid version of
the call.

When introducing new, optional parameters, it is wise to consider what the
default value(s) of that field will be, since all existing callers are -- by
contract -- able to continue calling the API without it.
<br><br>
 *(Just because we **can** add a parameter as an optional field doesn't mean we
 **should**.)*
{: .notice--danger }

{% mermaid %}
classDiagram
    direction RL
    BreakingNewOrder <.. Order : BREAKING
    NonBreakingNewOrder <|-- Order : Minor Version
    class Order {
        Int id
        OrderStatus status
        Int petId
        Int quantity
        DateTime shipDate
    }
    class BreakingNewOrder {
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

<br>
We often want parameters to be required in order to reduce the scope of support
our server needs to provide. As such, it's important to consider how the surface
of your models may change over time.

Another scenario to consider is a new requirement on an already-existing
parameter. For example, if an incoming `Int` parameter, like `id`, is now
required to be *positive*, that would also be a breaking change.
{: .notice--warning }

### Altering a Parameter / Response Field
Perhaps the sneakiest of our breaking change examples: even renaming a field is
a breaking change.

Although the general shape of data in a model may hold for renamed parameters,
the schema of the models has still changed, so callers must adjust accordingly.

Perhaps [bikeshedding](https://en.wiktionary.org/wiki/bikeshedding) on variable
names *is* worth it?
{: .notice--info }

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

<br>
In this example, we've renamed `quantity` to `amountOrdered` for "readability",
**breaking** anyone using the current version of `Pet Store`.

## Non-Breaking API Changes
There are ways of introducing changes without breaking the API. However, while
it may be tempting to use some of these ideas to work around problems with Major
Version changes, there are -- as in much of software engineering -- tradeoffs.

### Adding an Optional Parameter / Header
As [mentioned above](#removing-an-operation), we can introduce a new parameter
without breaking the API by making it optional. In some cases, this is valid,
but in others it can introduce complexity.

Take this example in `Pet Store`:

We add a new, optional `satisfactionRating` to users' `Order`s. A `null` value
for this is easy to handle, since we don't anticipate users to rate every
`Order`, even after introducing this feature.

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
rather a natural evolution of this API.

Consider, instead, the addition of the *optional* `purchaseDate` field in [the
example above](#removing-an-operation). It introduces edge behavior because how
do we handle a `null`? We would expect `purchaseDate` to be known.
{: .notice--danger }

### Adding Enum Values
Adding Enum variants is another way to avoid breaking changes. Existing clients
will be limited to only the old set of options, but they can still perform any
operations like they used to.

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

Deleting a variant, on the other hand, is a breaking change, since we are
removing a possible operation. Existing clients will break anytime they send the
now-removed Enum value.

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

Since there are no Response-only types in `Pet Store` (and thus, any added
Response field would also change a Request field), in our example, let's instead
imagine that we're adding a new cookie in a header, like so:

```json
headers: { 'Set-Cookie': 'myKey=myValue;' }
```

Any existing client can ignore this cookie (or, depending on the browser, maybe
still execute it). But there's no *expected* behavior from a client based on
this new Response field.

For this reason, most response fields are considered optional in the API
contract, since clients should be able to ignore those values when new
functionality is added.
{: .notice--info }

## How to Make Breaking Changes
In some (many) cases, we *want* to make a breaking change to our API. While we
should be aware that existing callers must update when migrating to a new
`MAJOR` version, we can take advantage of this larger shift to simplify or
improve our API surface when forcing users to take on breaking changes.

For example, [as mentioned above](#removing-an-operation), perhaps we *want* to
remove the `deleteOrder()` behavior from users, as we don't have the ability to
cancel orders in our system.
{: .notice--info }

As a reminder, once we've exposed an API to users, we must support it for the
short-term. A new version can remove behavior, though we should give users ample
time to adapt.

### Deprecation
Deprecation is a step-by-step process to help users prepare for change. In
general, designers deprecate an endpoint -- notifying users that they should
move away from it -- before eventually removing it.

While deprecation is a good way to inform users that an endpoint will no longer
be supported, it's also a good way of creating a backlog of items to remove in
the next `MAJOR` version bump.

{% mermaid %}
flowchart LR
    V1[[v1.0.0]]
    V2[[v1.1.0]]
    V3[[v2.0.0]]
    V1 -. "deprecation" .-> V2
    V2 -. "removal" .-> V3
{% endmermaid %}

There are many ways to handle deprecation (and eventual removal) of APIs, though
the key is to respect how existing users may be calling your code.
{: .notice--danger }

Typically, endpoints are deprecated for one or more `MINOR` versions and are
then eventually removed in the next `MAJOR` version (which can introduce
breaking changes).

### Making Major Version Changes
Releasing a new `MAJOR` version is a big deal. Existing users must update their
callers to reflect the new state of the API contract.

As such, it's important to make our `MAJOR` version updates *actually* major! I
believe that new `MAJOR` versions of APIs should cull deprecated APIs and make
radical changes as needed, because when else are you going to do it?

A new `MAJOR` API version doesn't *need* to remove all deprecated APIs, though
given that Major Version bumps should be performed (relatively) sparingly,
grouping breaking API changes together is appreciated.
{: .notice--info }

Consider that `MAJOR` version bumps will require real, in-depth changes *(e.g.
endpoints may no longer be available)*. Thus, since users will already be
modifying code to support a new major version of the API, grouping breaking
changes together can make an upgrade more palatable.

Would you rather fix **five** units breaking code of breaking code *once*, or
**one** unit of breaking code *five* times?
{: .notice--success }

### Versioning Endpoints and APIs Separately
Since we want to limit frequency of `MAJOR` version bumps, it can sometimes be
valuable to version endpoints and the API contract itself separately.

For example, since adding a ***new*** endpoint is considered a
backwards-compatible change (and so, only requires a `MINOR` version bump), we
can create an endpoint that ***extends*** existing behavior to avoid cutting
a breaking change to the API contract.

Look at how a new endpoint allows us to add a required `purchaseDate` field
*without* cutting a breaking change to the API:

{% mermaid %}
classDiagram
    direction RL
    BreakingNewOrder <.. Order : BREAKING
    NonBreakingNewOrder <|-- Order : Minor Version
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
    class BreakingNewOrder {
        +DateTime purchaseDate
    }
    class NonBreakingNewOrder {
        +placeOrderV2(Order order, DateTime purchaseDate) void
    }
{% endmermaid %}

<br>
We can alternatively do this by defining a new, renamed type of the `Order`
model:

{% mermaid %}
classDiagram
    class OrderV2 {
        Int id
        OrderStatus status
        Int petId
        Int quantity
        DateTime shipDate
        +DateTime purchaseDate
        getOrderById(String orderId) Order
        deleteOrder(String orderId) void
        placeOrder(Order order) void
    }
{% endmermaid %}

<br>
Existing callers of `Order` endpoints should still work as intended (and, to
recall, ***must*** be supported). In the meantime, users can upgrade and use
`OrderV2` to take advantage of new behavior.

The downside of this approach is that multiple variants of a call must be
supported at once.
<br><br>
Similar to adding an optional requirement, we must still decide what we do with
orders that *don't* provide a `purchaseDate`, since -- according to the API
contract -- we ***must*** support callers of the old endpoint unless we cut a
breaking change.
{: .notice--danger }

## Using OpenAPI
OpenAPI *(formerly known as Swagger)* is a specification for writing APIs for
both client- and server-side consumption.

Path templates (like `GET /order/{orderId}`) and data types (also called models)
are defined to indicate how providers of the API should respond to callers as
well as how users should expect the API to behave.

For example, here's [the complete specification of `Pet
Store`](https://github.com/OpenAPITools/openapi-generator/tree/master/samples/yaml).

### Language Agnosticism (the Good)
The OpenAPI specs are designed to be language agnostic so that any user can
define a client or server which meets the specification.

The spec is just a contract -- much like an `interface` -- that can be
implemented for seamless interaction between server and client.
{: .notice--primary }

The advantages are that front- and back-end implementations can be built in any
type of infrastructure, so long as the contract is respected. This makes it
easier to grow a userbase by lifting a limit on language.

{% mermaid %}
flowchart LR
    S>openapi-spec.yml]
    C1[[Client.kt]]
    C2[[Client.rs]]
    C3[[Client.ts]]
    S --> C1
    S --> C2
    S --> C3
{% endmermaid %}

<br>
This general-purpose format also encourages auto-generation, like via
`openapi-generator`, since REST clients and server stubs are trivial to
implement in most cases.  This makes taking on new versions much simpler.

{% mermaid %}
{% raw %}
flowchart LR
    S1>1.0.0-spec.yml]
    S2>1.1.0-spec.yml]
    S3>2.0.0-spec.yml]
    G{{openapi-generator}}
    S1 --> G
    S2 --> G
    S3 --> G
    C[[ClientLibrary]]
    G --> C
{% endraw %}
{% endmermaid %}

### Language Agnosticism (the Bad)
Unfortunately, across languages, there may not be the common understanding of
types that's present in the OpenAPI spec.

For example, Kotlin and Rust might understand differences between sized,
numerical values, but TypeScript only sees everything as a `number`:

{% mermaid %}
classDiagram
    direction LR
    OrderIdKt .. OrderIdRs
    OrderIdRs .. OrderIdTs
    class OrderIdKt {
        UByte id8
        UShort id16
        UInt id32
        ULong id64
    }
    class OrderIdRs {
        u8 id8
        u16 id16
        u32 id32
        u64 id64
    }
    class OrderIdTs {
        number id8
        number id16
        number id32
        number id64
    }
{% endmermaid %}

<br>
Additionally, hand-rolled clients might provide additional semantics due to
understood behavior by the author.

A common example would be returning `null` instead of a 404 error for `GET`
operations, since `null` often carries a clearer meaning than an error in code:

{% mermaid %}
{% raw %}
flowchart TB
    H[[HandrolledClient]]
    C1(Server returns NOT_FOUND)
    H -. "GET /order/{orderId}" .-> C1
    C1 --> E1{{ returns NULL }}
    G[[GeneratedClient]]
    C2(Server returns NOT_FOUND)
    G -. "GET /order/{orderId}" .-> C2
    C2 --> E2{{ raises 404 Exception }}
{% endraw %}
{% endmermaid %}

<br>
This can add additional cost to supporting auto-generated clients, though
wrapper types can be used to keep the custom behavior:

```rust
struct ClientWrapper {
  clientApi: ClientApi
}

impl OrderApi for ClientWrapper {
  pub fn getOrder(self, orderId: u32): Result<Option<Order>> {
    match (self.clientApi.getOrder(orderId)) {
      // Successful responses are returned
      Ok(response) => Ok(Some(response.order)),

      // Cast 404s to an empty `Option`
      Err(HttpResponse::NOT_FOUND) => Ok(None),

      // Other errors are raised normally
      error => error
    }
  }
}
```

### Client and Server Generation
We can use `openapi-generator` to produce both clients and server stubs based on
a spec. This can be a powerful way of reducing boilerplate code required by
engineers to support your endpoints.

A detailed guide on how to setup the generator can be found on [their Github
page, here](https://github.com/OpenAPITools/openapi-generator#1---installation).
{: .notice--info }

While various generators have mixed support (which is why I hesitate to dive
deep into a specific generator for this article), hopefully it's been made clear
the incentives of generation and why clear definitions of APIs via a spec is
preferred.

## Conclusion
Regardless of your method of *implementing* APIs, its important to consider how
your public interfaces may change over time so that they can be modified in a
way that minimizes impact on users.

While we can utilize techniques to circumvent the need to create breaking API
changes, we must be aware of the tradeoffs made to avoid a break. A `null` value
is not always easy to handle in a robust way from the backend.

Once we make a functionality public, we ***must*** support its callers until
cutting a new `MAJOR` version. And while breaking API changes can be avoided,
sometimes it's important to force users to take on changes to reduce the scope
of supportability.

Don't be afraid to cut a new `MAJOR` version of your API, but be aware of the
impact it may have on users, internal and external.
