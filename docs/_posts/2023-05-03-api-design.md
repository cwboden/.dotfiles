---
title: A Deep Dive into OpenAPI Generation
excerpt: Automated creation of clients and servers as well as how to design and version APIs.
last_modified_at:
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

### Pet Store (Sample API)
[sample taken from OpenAPI Generator](https://github.com/OpenAPITools/openapi-generator/tree/master/samples/yaml)

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

## Breaking API Changes
### Removing an Operation
To contrast: adding an operation

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

### Adding a New Requirement
Either a new required parameter or new validation for an existing parameter.

{% mermaid %}
classDiagram
    direction RL
    NewOrder <.. Order : BREAKING
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
{% endmermaid %}

### Altering a Parameter / Response Field
This includes renaming, removing, or changing the type of

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

## Non-Breaking API Changes
### Adding an Optional Parameter / Header
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

### Adding Enum Values
To contrast, removing enum values

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
        CANCELED
    }
{% endmermaid %}

### Adding a *Response* Field / Header
Add new cookie?
