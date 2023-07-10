import com.cwboden.generated.api.PetsApiService
import com.cwboden.generated.model.Pet
import com.cwboden.petstore.PetRepository
import org.springframework.stereotype.Service
import java.lang.RuntimeException

@Service
class PetService(
        private val petRepository: PetRepository
) : PetsApiService {
    override
    fun createPets(pet: Pet): Unit =
            petRepository
                    .create(pet)
                    .let { wasCreated ->
                        if (!wasCreated) {
                            throw RuntimeException("pet: $pet could not be created.")
                        }
                    }

    override
    fun listPets(limit: Int?): List<Pet> =
            petRepository
                    .read()
                    .limit(limit?.toLong() ?: 100)
                    .toList()

    override
    fun showPetById(petId: Long): Pet =
            petRepository
                    .read()
                    .filter { it.id == petId }
                    .findFirst()
                    .orElseThrow()
}