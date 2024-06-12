
import numpy as np

class Entity:

    """
    Entity: Represents a single entity in the population
    values: The values of the entity (genes)
    Where: 

    V_i are the Pb of move on that direction 
    and x is the P[0, 1] of kill
    """

    """ 
    |---------| | ------C_1----- | -------C_2------- |
    | \  |  / | | 0,  1,  2,  3, | 4,  5,  6,  7, 8  |
    | <- x -> | | \,  |,  /, ->, | \,  |,  /, <-, x  |
    | /  |  \ | | 0,  1,  2,  3, | 4,  5,  6,  7, 8  |
    |---------| | ------C_1----- | -------C_2------  |
    """

    def __init__(self, values=None, n_entity=1):

        """
        Constructor: Initializes the entity with random values
        values: The values of the entity (genes) [Optional]
        """

        self.name = f"E-{n_entity}"

        if values is None:
            self.values = [np.random.uniform() for _i in range(8)]
            self.values.append(np.random.randint(2))
            self.values = np.array(self.values)

        else:
            self.values = values

    def get_values(self):
        return self.values

    def crossover(self, other) -> tuple:

        """
        Crossover: Combine the genes of two entities
        Returns a tuple with two entities

        C_Child_1 = C1_1[0:3] + C2_2[4:8] x?
        C_Child_2 = C2_1[0:3] + C1_2[4:8] x?
        """

        if not isinstance(other, Entity):
            raise Exception("Invalid type for crossover")

        this_c_1 = self.values[0:4]
        this_c_2 = self.values[4:9]

        other_c_1 = other.values[0:4]
        other_c_2 = other.values[4:9]

        child_1 = np.concatenate((this_c_1, other_c_2))
        child_2 = np.concatenate((other_c_1, this_c_2))

        return Entity(values=child_1), Entity(values=child_2)

    def mutate(self):

        # denormalize

        value = np.random.randint(0, 9)

        if value != 8:
            self.values[value] = np.random.uniform()
        else:
            self.values[value] = np.random.randint(2)
        

    def show(self):
        print(self.values)

