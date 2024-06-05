
from src.entity import Entity

a = Entity()
b = Entity()

print("A")
a.show()
print("B")
b.show()

c1, c2 = a.crossover(b)

print("Crossover")

print("C1")
c1.show()

print("C2")
c2.show()
