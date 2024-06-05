
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
    |---------| | ------C_1----- | -------C_2----|-- |
    | \  |  / | | 0,  1,  2,  3, | 4,  5,  6,  7,|8 |
    | <- x -> | | \,  |,  /, ->, | \,  |,  /, <-,| x |
    | /  |  \ | | 0,  1,  2,  3, | 4,  5,  6,  7,|8 |
    |---------| | ------C_1----- | -------C_2----|- |
    """

    def __init__(self, values=None):

        """
        Constructor: Initializes the entity with random values
        values: The values of the entity (genes) [Optional]
        """

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

        C_Child_1 = C1_1[0:4] + C2_2[4:8] x?
        C_Child_2 = C2_1[0:4] + C1_2[4:8] x?
        """

        if not isinstance(other, Entity):
            raise Exception("Invalid type for crossover")

        this_c_1 = self.values[0:4]
        this_c_2 = self.values[4:8]

        other_c_1 = other.values[0:4]
        other_c_2 = other.values[4:8]

        c_1 = np.concatenate((this_c_1, other_c_2))
        c_2 = np.concatenate((other_c_1, this_c_2))

        return Entity(values=c_1), Entity(values=c_2)

    def show(self):
        print(self.values)

