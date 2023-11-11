BLOCK_SIZE=10
KEY = [0x9f, 0x96, 0xd1, 0xef, 0x3a, 0x79, 0x98, 0x29, 0x9e, 0x8a]

raw = open('shellcode_raw', 'rb').read()

out = b''

for i in range(0, int(len(raw) / BLOCK_SIZE)):
    start = i * BLOCK_SIZE
    end = start + BLOCK_SIZE
    block = raw[start:end]
    for j in range(BLOCK_SIZE):
        k = (KEY[j] + i) % 256
        out += bytes([block[j] ^ k])

open('shellcode_raw_encrypted', 'wb').write(out)

