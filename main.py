from random import randint
import math

X_PER_CHUNK = 5#16
MAX_Y = 200 #384
Z_PER_CHUNK = 5#16

def set_block(x, y, z, seed):
    block = ' '
    if y < 113:
        block = '\033[36m~\033[39m'
    if y < math.sin(x/16)*16+112:
        block = '\033[32m/\033[39m'
    if y < math.sin(x/16)*16+111:
        block = '#'
    return block

class Chunk:
    def __init__(self, chunk_x=0, chunk_z=0, seed=randint(0,100000)): #Size is 16 by 16 by 384
        self.data = [0]*X_PER_CHUNK
        for x in range(0, X_PER_CHUNK):
            temp_1 = [0]*MAX_Y
            for y in range(0, MAX_Y):
                temp_2 = [0]*Z_PER_CHUNK
                for z in range(0, Z_PER_CHUNK):
                    temp_2[z] = set_block(x + chunk_x * X_PER_CHUNK, y, z + chunk_z * Z_PER_CHUNK, seed)
                temp_1[y] = temp_2
            self.data[x] = temp_1

    def get_block(self, x, y, z):
        return self.data[x][y][z]
    
    def to_string(self, z:int=0):
        for y in range(len(self.data[0])-1, -1, -1):
            s = ''
            for x in range(len(self.data)):
                s+=self.data[x][y][z]
            print(s)
                

class World:
    def __init__(self, chunks_x, chunks_z, seed=randint(0,100000)):
        self.chunks = [0]*chunks_x
        for chunk_x in range(0, chunks_x):
            temp = [0]*chunks_z
            for chunk_z in range(0, chunks_z):
                temp[chunk_z] = Chunk(chunk_x, chunk_z, seed)
            self.chunks[chunk_x] = temp

    def get_block(self, x:int, y:int, z:int):
        return self.chunks[x//X_PER_CHUNK][z//Z_PER_CHUNK].get_block(x%X_PER_CHUNK, y, z%Z_PER_CHUNK)
    
    def to_string(self, chunk_x:int= 0, chunk_z:int= 0, n_chunks:int= 1):
        for z in range(Z_PER_CHUNK):
            for y in range(MAX_Y-1, -1, -1):
                s = ''
                for x in range(X_PER_CHUNK*n_chunks):
                    s += self.get_block(chunk_x + x, y, chunk_z + z)
                print(s)
            print("\n")
            
                

world = World(9, 8, 0)
world.to_string(1,0,8)
#chunk = Chunk()
#chunk.to_string()
print("Done")
while True:
    a = 0
