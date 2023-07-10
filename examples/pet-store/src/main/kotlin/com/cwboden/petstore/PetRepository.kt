package com.cwboden.petstore

import com.cwboden.generated.model.Pet
import java.util.stream.Stream

open class Repository<T> {
    private val objects = HashSet<T>()

    /**
     * Adds the specified element to the set.
     *
     * @return `true` if the element has been added, `false` if the element is already contained in the set.
     */
    fun create(newObject: T): Boolean = objects.add(newObject)

    /**
     * @return a Stream of all elements in the set
     */
    fun read(): Stream<T> = objects.stream()
}

@org.springframework.stereotype.Repository
class PetRepository : Repository<Pet>()