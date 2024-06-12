
import numpy as np
from python.entity import Entity

# 10 x 20

class Poblation:

    def __init__(self, n_subjects=15, dim=(10, 20)) -> None:
        
        self.poblation = np.zeros(dim, dtype=object)

        while n_subjects > 0: 

            ent = Entity(n_entity=n_subjects)

            rr = np.random.randint(10)
            rc = np.random.randint(2)

            if self.poblation[rr][rc] == 0:
                self.poblation[rr][rc] = ent
                n_subjects -= 1


    def show(self):
        
        for i in range(len(self.poblation)):
            for j in range(len(self.poblation[0])):

                if isinstance(self.poblation[i][j], Entity):
                    print(self.poblation[i][j].name, end=",")

                else:
                    print(self.poblation[i][j], end="º")


                