---
title: Using the Facade Pattern to Split Tightly-Coupled Services
excerpt: Incrementally transition a monolithic service into multiple, smaller chunks.
last_modified_at: 2023-08-30T01:02:43
categories:
 - Programming
tags:
  - api
  - architecture
  - design-patterns
---

As services grow, there are often tendencies to expand existing API surfaces,
rather than breaking contexts apart. This results in more tightly-coupled
services which should be broken apart as the bloated system matures into more
distinct layers.

## Taking Advantage of ***Facades***

Let's take, for example, a persisted `StoreSummary` entity which contains a
myriad of information, though most users only require a small section, like the
contact information for the manager or the shop's hours. This makes changes to
the `StoreSummary` class or its associated components *(like a `Controller`)*
expensive, since it forces downstream users to use the latest definition of
`StoreSummary`, even if the data they care about is unchanged.

This diagram demonstrates some possible "sub-types" of data that are currently
part of `StoreSummary` but should be split out:

{% mermaid %}
flowchart LR
    SP{{StoreProvider}}
    SS>StoreSummary]

    SP --> SS

    SS -.-> SC
    SS -.-> ST
    SS -.-> SH
    SS -.-> SN

    SC>StoreContact]
    SH>StoreHours]
    ST>StoreTags]
    SN>StoreNotes]
{% endmermaid %}

Solving this problem can feel difficult at first, since it seems like we'd need
a sweeping change to split-out and convert all users of `StoreSummary` to
instead use their sub-type. Plus, how can we make these changes mechanical to
maintain parity before and after the split?

This is where the ***Facade Pattern*** helps us out. By inserting a layer
between `StoreProvider` and its users, we can restrict the API surface as needed
so that callers can work with only the necessary data.
{: .notice--success }

We do this by:

1. Identifying the sub-types of `StoreSummary` based on its use-cases elsewhere in
   the code. *(This can also be done iteratively, breaking off small chunks as
   needed.)*
1. Inserting a ***Facade*** Provider between `StoreProvider` and any sub-types.
1. Converting users of the top-level Provider to use the sub-type-specific
   ***Facade***.
1. Extracting the ***Facade*** Provider logic out of the top-level Provider; now
   it can live as a standalone service!

Now let's look at an example. Below, we've introduced a ***Facade*** for three
of our sub-types. The ***Facade*** Providers can each call into `StoreProvider`
to fetch the data they need, but then only expose the sub-type to callers.

Any user of `StoreContact`, for example, is now explicitly using that sub-type,
rather than fetching a `StoreSummary` and then fishing for the data it needs.
This has the added benefit of reducing test setup, since we don't need an
*entire* `StoreSummary` to validate our system's behavior.

We can also see that these chunks can be done iteratively. For example, users of
any `StoreNotes` data still currently rely on `StoreSummary` directly:

{% mermaid %}
flowchart LR
    SP{{StoreProvider}}

    SP <-.-> SCP
    SP <-.-> STP
    SP <-.-> SHP

    subgraph facade
        direction LR
        SCP{{StoreContactProvider}}
        STP{{StoreTagsProvider}}
        SHP{{StoreHoursProvider}}
    end

    SC>StoreContact]
    SCP --> SC

    ST>StoreTags]
    STP --> ST

    SH>StoreHours]
    SHP --> SH

    SS>StoreSummary]
    SP --> SS

    SN>StoreNotes]
    SS -.-> SN
{% endmermaid %}

In the next snapshot of our development process, described by the following
diagram, we have since created a Facade for `StoreNotes` as well as extracted
the `StoreContact`/`TagsProviders` out into their own service.

By this point of development, the `StoreProvider` and the associated
`StoreSummary` type are completely enveloped by the ***Facade***. Modifications
to `StoreSummary` are less invasive, as we only need to update their usage in
the `StoreNotes`/`HoursProviders`.

Furthermore, the two extracted Providers now own their data and can start
following best practices, like consuming their own database. By using the
***Facade***, it's easier to stradle the in-between state without breaking any
existing users.

{% mermaid %}
flowchart LR
    SP{{StoreProvider}}

    SP <-.-> SHP
    SP <-.-> SNP

    subgraph facade
        direction LR
        SHP{{StoreHoursProvider}}
        SNP{{StoreNotesProvider}}
    end

    SCP{{StoreContactProvider}}
    SC>StoreContact]
    SCP --> SC

    SCP{{StoreTagsProvider}}
    ST>StoreTags]
    STP --> ST

    SH>StoreHours]
    SHP --> SH

    SN>StoreNotes]
    SNP --> SN
{% endmermaid %}

## Leveraging Kotlin Delegation

Finally, while it's often best practice to split these services apart, it can
also often be necessary to maintain the existing, tightly-coupled endpoints that
are used by legacy systems.

Thankfully, Kotlin's delegation pattern can make this a breeze by allowing us to
extend the old top-level Provider class by delegating to our newly-extracted
Providers. Let's use our `StoreProvider` as an example once more:

```kotlin
interface IStoreHoursProvider {
  fun getStoreHours(storeId: StoreId): StoreHours;
}

// ## OLD IMPLEMENTATION ##
@Service
class StoreProvider : IStoreHoursProvider {
  override fun getStoreHours(storeId: StoreId): StoreHours {
    // pseudo-implementation
    dbAccessor.masterConnection.useWith { conn ->
      FluentStatement.fromSql(SOME_ACCESS_STATEMENT)
          .onConnection(conn).useWith {
        it.executeQuery().toStoreHours()
      }
    }
  }
}

// ## NEW IMPLEMENTATION ##
@Service
class StoreProvider(
  private val storeHoursProvider: StoreHoursProvider,
) : IStoreHoursProvider by storeHoursProvider // exposes getStoreHours()
```

The `StoreProvider` uses the given `StoreHoursProvider` as its delegate to
implement `IStoreHoursProvider`. Legacy calls to `StoreProvider` will continue
to function, calling through to the sub-service, though any users that only need
`StoreStatus` can be migrated to depend directly on `StoreHoursProvider`
instead.

To sum things up, the Facade Pattern is helpful for:
- Splitting large, tightly-coupled services into smaller chunks incrementally.
- Easily maintaining backwards compatibility with existing callers of these
- endpoints.
- Shoring up Providers to take advantage of modern best practices.
{: .notice--success }
